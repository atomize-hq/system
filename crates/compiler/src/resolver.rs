use crate::budget::evaluate_budget;
use crate::packet_result::{
    PacketBodyNote, PacketBodyNoteKind, PacketDecisionSummary, PacketFixtureContext, PacketResult,
    PacketSection, PacketSectionMode, PacketSourceSummary, PacketVariant,
};
use crate::{
    blocker_category_priority, ArtifactIngestIssueKind, ArtifactManifest, Blocker, BlockerCategory,
    BudgetDisposition, BudgetNextSafeAction, BudgetOutcome, BudgetPolicy, CanonicalArtifact,
    CanonicalArtifactKind, CanonicalArtifacts, CompilerError, DecisionLog, FreshnessIssueKind,
    FreshnessStatus, ManifestInputs, NextSafeAction, Refusal, RefusalCategory, SubjectRef,
    SystemRootStatus,
};
use std::cmp::Ordering;
use std::path::Path;

const C04_RESULT_VERSION: &str = "reduced-v1";
const DEFAULT_PACKET_ID: &str = "planning.packet";
const DEMO_EXECUTION_PACKET_ID: &str = "execution.demo.packet";
const LIVE_EXECUTION_PACKET_ID: &str = "execution.live.packet";
const SYSTEM_ROOT_PATH: &str = ".system";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveRequest {
    pub budget_policy: BudgetPolicy,
    pub packet_id: &'static str,
}

impl Default for ResolveRequest {
    fn default() -> Self {
        Self {
            budget_policy: BudgetPolicy::default(),
            packet_id: DEFAULT_PACKET_ID,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketSelectionStatus {
    Selected,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketSelection {
    pub packet_id: String,
    pub status: PacketSelectionStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolverResult {
    pub c04_result_version: String,
    pub c03_schema_version: String,
    pub c03_manifest_generation_version: u32,
    pub c03_fingerprint_sha256: String,
    pub packet_result: PacketResult,
    pub decision_log: DecisionLog,
    pub budget_outcome: BudgetOutcome,
    pub selection: PacketSelection,
    pub refusal: Option<Refusal>,
    pub blockers: Vec<Blocker>,
}

pub fn resolve(
    repo_root: impl AsRef<Path>,
    request: ResolveRequest,
) -> Result<ResolverResult, CompilerError> {
    let canonical_artifacts = CanonicalArtifacts::load(repo_root.as_ref())
        .map_err(|err| CompilerError::Manifest(crate::ManifestError::Ingest(err)))?;

    let manifest =
        ArtifactManifest::from_canonical_artifacts(&canonical_artifacts, ManifestInputs::default());

    let mut decision_log = DecisionLog {
        entries: Vec::new(),
    };

    decision_log.entries.push(format!(
        "c03.system_root status={:?}",
        manifest.system_root_status
    ));

    decision_log.entries.push(format!(
        "c03.provenance schema_version={} manifest_generation_version={} fingerprint_sha256={}",
        manifest.version.schema.version,
        manifest.version.generation,
        &manifest.freshness.fingerprint_sha256
    ));

    for artifact in &manifest.artifacts {
        decision_log.entries.push(format!(
            "c03.artifact kind={:?} required={} presence={:?} byte_len={:?} sha256={:?} path={}",
            artifact.kind,
            artifact.required,
            artifact.presence,
            artifact.byte_len,
            artifact.content_sha256.as_deref(),
            artifact.relative_path
        ));
    }

    for issue in &manifest.ingest_issues {
        decision_log.entries.push(format!(
            "c03.ingest.issue kind={:?} required={} path={}",
            issue.kind, issue.required, issue.canonical_repo_relative_path
        ));
    }

    decision_log.entries.push(format!(
        "c03.freshness status={:?} issue_count={}",
        manifest.freshness.status,
        manifest.freshness.issues.len()
    ));
    for issue in &manifest.freshness.issues {
        decision_log.entries.push(format!(
            "c03.freshness.issue kind={:?} detail={}",
            issue.kind, issue.detail
        ));
    }

    let budget_outcome = evaluate_budget(&manifest.artifacts, request.budget_policy);
    decision_log.entries.push(format!(
        "budget disposition={:?} reason={:?} targets={} next_safe_action={}",
        budget_outcome.disposition,
        budget_outcome.reason,
        budget_outcome.targets.len(),
        budget_outcome.next_safe_action.is_some()
    ));

    let refusal = compute_refusal(&manifest, &budget_outcome, &request);
    if let Some(refusal) = &refusal {
        decision_log.entries.push(format!(
            "refusal category={:?} broken_subject={:?} next_safe_action={:?}",
            refusal.category, refusal.broken_subject, refusal.next_safe_action
        ));
    }

    let blockers = compute_blockers(&manifest, &budget_outcome, &request);
    for blocker in &blockers {
        decision_log.entries.push(format!(
            "blocker category={:?} subject={:?} next_safe_action={:?}",
            blocker.category, blocker.subject, blocker.next_safe_action
        ));
    }

    let selection_status = if manifest.freshness.status == FreshnessStatus::Ok
        && budget_outcome.disposition != BudgetDisposition::Refuse
        && refusal.is_none()
        && blockers.is_empty()
    {
        PacketSelectionStatus::Selected
    } else {
        PacketSelectionStatus::Blocked
    };

    decision_log.entries.push(format!(
        "selection packet_id={} status={:?}",
        request.packet_id, selection_status
    ));

    let packet_result = build_packet_result(BuildPacketResultInput {
        repo_root: repo_root.as_ref(),
        request: &request,
        artifacts: &canonical_artifacts,
        manifest: &manifest,
        budget_outcome: &budget_outcome,
        selection_status,
        refusal: refusal.as_ref(),
        blockers: &blockers,
        decision_log_entries: decision_log.entries.len(),
    });

    Ok(ResolverResult {
        c04_result_version: C04_RESULT_VERSION.to_string(),
        c03_schema_version: manifest.version.schema.version.to_string(),
        c03_manifest_generation_version: manifest.version.generation,
        c03_fingerprint_sha256: manifest.freshness.fingerprint_sha256.clone(),
        packet_result,
        decision_log,
        budget_outcome,
        selection: PacketSelection {
            packet_id: request.packet_id.to_string(),
            status: selection_status,
        },
        refusal,
        blockers,
    })
}

struct BuildPacketResultInput<'a> {
    repo_root: &'a Path,
    request: &'a ResolveRequest,
    artifacts: &'a CanonicalArtifacts,
    manifest: &'a ArtifactManifest,
    budget_outcome: &'a BudgetOutcome,
    selection_status: PacketSelectionStatus,
    refusal: Option<&'a Refusal>,
    blockers: &'a [Blocker],
    decision_log_entries: usize,
}

fn build_packet_result(input: BuildPacketResultInput<'_>) -> PacketResult {
    let BuildPacketResultInput {
        repo_root,
        request,
        artifacts,
        manifest,
        budget_outcome,
        selection_status,
        refusal,
        blockers,
        decision_log_entries,
    } = input;

    let variant = packet_variant_for(request.packet_id);
    let included_sources = included_sources_for(artifacts, budget_outcome);
    let packet_body_ready = selection_status == PacketSelectionStatus::Selected
        && refusal.is_none()
        && blockers.is_empty();
    let notes = packet_notes_for(manifest, budget_outcome, artifacts, packet_body_ready);
    let sections = if packet_body_ready {
        packet_sections_for(artifacts, budget_outcome)
    } else {
        Vec::new()
    };
    let fixture_context = fixture_context_for(repo_root, request.packet_id, artifacts);

    let summary_line = if selection_status == PacketSelectionStatus::Selected {
        let fixture_suffix = fixture_context
            .as_ref()
            .map(|context| format!(" (fixture set {})", context.fixture_set_id))
            .unwrap_or_default();
        format!(
            "READY {}{}: {} included sources, budget {:?}/{:?}, {} decision log entries",
            request.packet_id,
            fixture_suffix,
            included_sources.len(),
            budget_outcome.disposition,
            budget_outcome.reason,
            decision_log_entries
        )
    } else if let Some(refusal) = refusal {
        format!(
            "REFUSED {}: category={:?}, blockers={}, budget {:?}/{:?}, {} decision log entries",
            request.packet_id,
            refusal.category,
            blockers.len(),
            budget_outcome.disposition,
            budget_outcome.reason,
            decision_log_entries
        )
    } else {
        format!(
            "BLOCKED {}: blockers={}, budget {:?}/{:?}, {} decision log entries",
            request.packet_id,
            blockers.len(),
            budget_outcome.disposition,
            budget_outcome.reason,
            decision_log_entries
        )
    };

    let ready_next_safe_action =
        next_safe_action_for_ready_packet(request.packet_id, variant, fixture_context.as_ref());

    PacketResult {
        packet_id: request.packet_id.to_string(),
        variant,
        fixture_context,
        included_sources,
        notes,
        decision_summary: PacketDecisionSummary {
            packet_status: selection_status,
            budget_disposition: budget_outcome.disposition,
            budget_reason: budget_outcome.reason.clone(),
            decision_log_entries,
            summary_line,
            ready_next_safe_action,
        },
        sections,
    }
}

fn packet_variant_for(packet_id: &str) -> PacketVariant {
    if packet_id == DEMO_EXECUTION_PACKET_ID {
        PacketVariant::ExecutionDemo
    } else if packet_id == LIVE_EXECUTION_PACKET_ID {
        PacketVariant::ExecutionLive
    } else {
        PacketVariant::Planning
    }
}

fn included_sources_for(
    artifacts: &CanonicalArtifacts,
    budget_outcome: &BudgetOutcome,
) -> Vec<PacketSourceSummary> {
    present_sources_for(artifacts)
        .into_iter()
        .filter(|source| {
            !should_exclude_artifact(source.canonical_repo_relative_path, budget_outcome)
        })
        .collect()
}

fn present_sources_for(artifacts: &CanonicalArtifacts) -> Vec<PacketSourceSummary> {
    artifacts
        .identities()
        .into_iter()
        .filter_map(|identity| {
            if matches!(identity.presence, crate::ArtifactPresence::Missing) {
                None
            } else {
                Some(PacketSourceSummary {
                    kind: identity.kind,
                    canonical_repo_relative_path: identity.relative_path,
                    required: identity.required,
                    presence: identity.presence,
                    byte_len: identity.byte_len,
                    content_sha256: identity.content_sha256.clone(),
                })
            }
        })
        .collect()
}

fn packet_notes_for(
    manifest: &ArtifactManifest,
    budget_outcome: &BudgetOutcome,
    artifacts: &CanonicalArtifacts,
    packet_body_ready: bool,
) -> Vec<PacketBodyNote> {
    let mut notes = Vec::new();
    let project_context_path = artifacts.project_context.identity.relative_path;

    if artifacts.project_context.identity.presence == crate::ArtifactPresence::Missing
        && ingest_issue_for_path(manifest, project_context_path).is_none()
    {
        notes.push(PacketBodyNote {
            kind: PacketBodyNoteKind::Omission,
            text: format!("optional source omitted: {project_context_path}"),
        });
    }

    match budget_outcome.disposition {
        BudgetDisposition::Summarize => {
            for target in &budget_outcome.targets {
                notes.push(PacketBodyNote {
                    kind: PacketBodyNoteKind::Budget,
                    text: format!(
                        "optional source summarized due to budget: {}",
                        target.canonical_repo_relative_path
                    ),
                });
            }
        }
        BudgetDisposition::Exclude => {
            for target in &budget_outcome.targets {
                notes.push(PacketBodyNote {
                    kind: PacketBodyNoteKind::Omission,
                    text: format!(
                        "optional source excluded due to budget: {}",
                        target.canonical_repo_relative_path
                    ),
                });
            }
        }
        BudgetDisposition::Keep | BudgetDisposition::Refuse => {}
    }

    if !packet_body_ready {
        notes.push(PacketBodyNote {
            kind: PacketBodyNoteKind::Omission,
            text: "packet body omitted because request is not ready".to_string(),
        });
    }

    let budget_text = format!(
        "budget: {:?}/{:?} across {} tracked canonical artifacts",
        budget_outcome.disposition,
        budget_outcome.reason,
        manifest.artifacts.len()
    );
    notes.push(PacketBodyNote {
        kind: PacketBodyNoteKind::Budget,
        text: budget_text,
    });

    notes
}

fn packet_sections_for(
    artifacts: &CanonicalArtifacts,
    budget_outcome: &BudgetOutcome,
) -> Vec<PacketSection> {
    let mut sections = Vec::new();
    push_packet_section(&mut sections, &artifacts.charter, "CHARTER", budget_outcome);
    push_packet_section(
        &mut sections,
        &artifacts.project_context,
        "PROJECT_CONTEXT",
        budget_outcome,
    );
    push_packet_section(
        &mut sections,
        &artifacts.feature_spec,
        "FEATURE_SPEC",
        budget_outcome,
    );
    sections
}

fn push_packet_section(
    sections: &mut Vec<PacketSection>,
    artifact: &CanonicalArtifact,
    title: &str,
    budget_outcome: &BudgetOutcome,
) {
    if matches!(artifact.identity.presence, crate::ArtifactPresence::Missing) {
        return;
    }

    if should_exclude_artifact(artifact.identity.relative_path, budget_outcome) {
        return;
    }

    let (mode, contents) =
        if should_summarize_artifact(artifact.identity.relative_path, budget_outcome) {
            (PacketSectionMode::Summary, summarize_artifact(artifact))
        } else {
            let contents = artifact
                .bytes
                .as_ref()
                .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
                .unwrap_or_default();
            (PacketSectionMode::Verbatim, contents)
        };

    sections.push(PacketSection {
        kind: artifact.identity.kind,
        canonical_repo_relative_path: artifact.identity.relative_path,
        title: title.to_string(),
        mode,
        contents,
    });
}

fn should_summarize_artifact(
    canonical_repo_relative_path: &str,
    budget_outcome: &BudgetOutcome,
) -> bool {
    budget_outcome.disposition == BudgetDisposition::Summarize
        && budget_targets_artifact(canonical_repo_relative_path, budget_outcome)
}

fn should_exclude_artifact(
    canonical_repo_relative_path: &str,
    budget_outcome: &BudgetOutcome,
) -> bool {
    budget_outcome.disposition == BudgetDisposition::Exclude
        && budget_targets_artifact(canonical_repo_relative_path, budget_outcome)
}

fn budget_targets_artifact(
    canonical_repo_relative_path: &str,
    budget_outcome: &BudgetOutcome,
) -> bool {
    budget_outcome
        .targets
        .iter()
        .any(|target| target.canonical_repo_relative_path == canonical_repo_relative_path)
}

fn summarize_artifact(artifact: &CanonicalArtifact) -> String {
    let byte_len = artifact.identity.byte_len.unwrap_or(0);
    let sha256 = artifact
        .identity
        .content_sha256
        .as_deref()
        .unwrap_or("unavailable");

    format!(
        "budget summary: full contents omitted for {} ({} bytes, sha256={})",
        artifact.identity.relative_path, byte_len, sha256
    )
}

fn fixture_context_for(
    repo_root: &Path,
    packet_id: &str,
    artifacts: &CanonicalArtifacts,
) -> Option<PacketFixtureContext> {
    if packet_variant_for(packet_id) != PacketVariant::ExecutionDemo {
        return None;
    }

    let fixture_set_id = repo_root
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())?;

    let parent = repo_root.parent()?;
    let grandparent = parent.parent()?;
    let great_grandparent = grandparent.parent()?;
    if parent.file_name().and_then(|name| name.to_str())? != "execution_demo"
        || grandparent.file_name().and_then(|name| name.to_str())? != "fixtures"
        || great_grandparent
            .file_name()
            .and_then(|name| name.to_str())?
            != "tests"
    {
        return None;
    }

    let fixture_basis_root = format!("tests/fixtures/execution_demo/{fixture_set_id}/.system/");

    Some(PacketFixtureContext {
        fixture_set_id,
        fixture_basis_root,
        fixture_lineage: present_sources_for(artifacts),
    })
}

fn next_safe_action_for_ready_packet(
    packet_id: &str,
    variant: PacketVariant,
    fixture_context: Option<&PacketFixtureContext>,
) -> String {
    match variant {
        PacketVariant::ExecutionLive => "run `doctor`".to_string(),
        PacketVariant::Planning => {
            format!("run `system inspect --packet {packet_id}` for proof")
        }
        PacketVariant::ExecutionDemo => {
            if let Some(context) = fixture_context {
                format!(
                    "run `system inspect --packet {packet_id} --fixture-set {}` for proof",
                    context.fixture_set_id
                )
            } else {
                format!("run `system inspect --packet {packet_id}` for proof")
            }
        }
    }
}

fn compute_refusal(
    manifest: &ArtifactManifest,
    budget_outcome: &BudgetOutcome,
    request: &ResolveRequest,
) -> Option<Refusal> {
    match manifest.system_root_status {
        SystemRootStatus::Ok => {}
        SystemRootStatus::Missing => {
            return Some(Refusal {
                category: RefusalCategory::SystemRootMissing,
                summary: "missing canonical .system root".to_string(),
                broken_subject: SubjectRef::Policy {
                    policy_id: "system_root",
                },
                next_safe_action: NextSafeAction::CreateSystemRoot {
                    canonical_repo_relative_path: SYSTEM_ROOT_PATH,
                },
            });
        }
        SystemRootStatus::NotDir => {
            return Some(Refusal {
                category: RefusalCategory::SystemRootNotDir,
                summary: "canonical .system root is not a directory".to_string(),
                broken_subject: SubjectRef::Policy {
                    policy_id: "system_root",
                },
                next_safe_action: NextSafeAction::EnsureSystemRootIsDirectory {
                    canonical_repo_relative_path: SYSTEM_ROOT_PATH,
                },
            });
        }
        SystemRootStatus::SymlinkNotAllowed => {
            return Some(Refusal {
                category: RefusalCategory::SystemRootSymlinkNotAllowed,
                summary: "canonical .system root must not be a symlink".to_string(),
                broken_subject: SubjectRef::Policy {
                    policy_id: "system_root",
                },
                next_safe_action: NextSafeAction::RemoveSystemRootSymlink {
                    canonical_repo_relative_path: SYSTEM_ROOT_PATH,
                },
            });
        }
    }

    if let Some(refusal) = refusal_for_ingest_issues(manifest) {
        return Some(refusal);
    }

    for artifact in &manifest.artifacts {
        if !artifact.required {
            continue;
        }

        if ingest_issue_for_path(manifest, artifact.relative_path).is_some() {
            continue;
        }

        match artifact.presence {
            crate::ArtifactPresence::Missing => {
                return Some(Refusal {
                    category: RefusalCategory::RequiredArtifactMissing,
                    summary: "missing required canonical artifact".to_string(),
                    broken_subject: SubjectRef::CanonicalArtifact {
                        kind: artifact.kind,
                        canonical_repo_relative_path: artifact.relative_path,
                    },
                    next_safe_action: NextSafeAction::CreateCanonicalArtifact {
                        canonical_repo_relative_path: artifact.relative_path,
                    },
                });
            }
            crate::ArtifactPresence::PresentEmpty => {
                return Some(Refusal {
                    category: RefusalCategory::RequiredArtifactEmpty,
                    summary: "required canonical artifact is empty".to_string(),
                    broken_subject: SubjectRef::CanonicalArtifact {
                        kind: artifact.kind,
                        canonical_repo_relative_path: artifact.relative_path,
                    },
                    next_safe_action: NextSafeAction::FillCanonicalArtifact {
                        canonical_repo_relative_path: artifact.relative_path,
                    },
                });
            }
            crate::ArtifactPresence::PresentNonEmpty => {}
        }
    }

    if manifest.freshness.status == FreshnessStatus::Invalid {
        let has_forbidden_override = manifest
            .freshness
            .issues
            .iter()
            .any(|issue| matches!(issue.kind, FreshnessIssueKind::ForbiddenOverride));
        return Some(Refusal {
            category: RefusalCategory::FreshnessInvalid,
            summary: if has_forbidden_override {
                "freshness truth is invalid (forbidden override)".to_string()
            } else {
                "freshness truth is invalid".to_string()
            },
            broken_subject: SubjectRef::Policy {
                policy_id: "freshness",
            },
            next_safe_action: NextSafeAction::RunDoctor,
        });
    }

    if budget_outcome.disposition == BudgetDisposition::Refuse {
        let canonical_repo_relative_path = match budget_outcome.next_safe_action.as_ref() {
            Some(BudgetNextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            }) => *canonical_repo_relative_path,
            None => SYSTEM_ROOT_PATH,
        };

        return Some(Refusal {
            category: RefusalCategory::BudgetRefused,
            summary: "budget refused packet generation".to_string(),
            broken_subject: SubjectRef::Policy {
                policy_id: "budget",
            },
            next_safe_action: NextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            },
        });
    }

    if request.packet_id == LIVE_EXECUTION_PACKET_ID {
        return Some(Refusal {
            category: RefusalCategory::UnsupportedRequest,
            summary:
                "unsupported live slice execution request: reduced v1 supports live planning packets plus fixture-backed execution demos only"
                    .to_string(),
            broken_subject: SubjectRef::Policy {
                policy_id: "packet_id",
            },
            next_safe_action: NextSafeAction::RunGenerate {
                packet_id: DEFAULT_PACKET_ID,
            },
        });
    }

    None
}

fn refusal_for_ingest_issues(manifest: &ArtifactManifest) -> Option<Refusal> {
    let mut first_symlink_issue = None;
    let mut first_required_read_issue = None;

    for issue in &manifest.ingest_issues {
        match issue.kind {
            ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed => {
                first_symlink_issue.get_or_insert(issue);
            }
            ArtifactIngestIssueKind::CanonicalArtifactReadError => {
                if issue.required {
                    first_required_read_issue.get_or_insert(issue);
                }
            }
        }
    }

    if let Some(issue) = first_symlink_issue {
        let kind = issue.artifact_kind;
        let canonical_repo_relative_path = issue.canonical_repo_relative_path;
        return Some(Refusal {
            category: RefusalCategory::NonCanonicalInputAttempt,
            summary: "canonical artifact path must not be a symlink".to_string(),
            broken_subject: SubjectRef::CanonicalArtifact {
                kind,
                canonical_repo_relative_path,
            },
            next_safe_action: NextSafeAction::CreateCanonicalArtifact {
                canonical_repo_relative_path,
            },
        });
    }

    if let Some(issue) = first_required_read_issue {
        let kind = issue.artifact_kind;
        let canonical_repo_relative_path = issue.canonical_repo_relative_path;
        return Some(Refusal {
            category: RefusalCategory::ArtifactReadError,
            summary: "failed to read canonical artifact".to_string(),
            broken_subject: SubjectRef::CanonicalArtifact {
                kind,
                canonical_repo_relative_path,
            },
            next_safe_action: NextSafeAction::CreateCanonicalArtifact {
                canonical_repo_relative_path,
            },
        });
    }

    None
}

fn ingest_issue_for_path(
    manifest: &ArtifactManifest,
    canonical_repo_relative_path: &'static str,
) -> Option<ArtifactIngestIssueKind> {
    manifest
        .ingest_issues
        .iter()
        .find(|issue| issue.canonical_repo_relative_path == canonical_repo_relative_path)
        .map(|issue| issue.kind)
}

fn compute_blockers(
    manifest: &ArtifactManifest,
    budget_outcome: &BudgetOutcome,
    request: &ResolveRequest,
) -> Vec<Blocker> {
    let mut blockers = Vec::new();

    for issue in &manifest.ingest_issues {
        blockers.push(Blocker {
            category: BlockerCategory::ArtifactReadError,
            subject: SubjectRef::CanonicalArtifact {
                kind: issue.artifact_kind,
                canonical_repo_relative_path: issue.canonical_repo_relative_path,
            },
            summary: match issue.kind {
                ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed => {
                    "canonical artifact path must not be a symlink".to_string()
                }
                ArtifactIngestIssueKind::CanonicalArtifactReadError => {
                    "failed to read canonical artifact".to_string()
                }
            },
            next_safe_action: NextSafeAction::CreateCanonicalArtifact {
                canonical_repo_relative_path: issue.canonical_repo_relative_path,
            },
        });
    }

    match manifest.system_root_status {
        SystemRootStatus::Ok => {}
        SystemRootStatus::Missing => blockers.push(Blocker {
            category: BlockerCategory::SystemRootMissing,
            subject: SubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: "missing canonical .system root".to_string(),
            next_safe_action: NextSafeAction::CreateSystemRoot {
                canonical_repo_relative_path: SYSTEM_ROOT_PATH,
            },
        }),
        SystemRootStatus::NotDir => blockers.push(Blocker {
            category: BlockerCategory::SystemRootNotDir,
            subject: SubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: "canonical .system root is not a directory".to_string(),
            next_safe_action: NextSafeAction::EnsureSystemRootIsDirectory {
                canonical_repo_relative_path: SYSTEM_ROOT_PATH,
            },
        }),
        SystemRootStatus::SymlinkNotAllowed => blockers.push(Blocker {
            category: BlockerCategory::SystemRootSymlinkNotAllowed,
            subject: SubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: "canonical .system root must not be a symlink".to_string(),
            next_safe_action: NextSafeAction::RemoveSystemRootSymlink {
                canonical_repo_relative_path: SYSTEM_ROOT_PATH,
            },
        }),
    }

    if blockers.is_empty() {
        for artifact in &manifest.artifacts {
            if !artifact.required {
                continue;
            }

            if ingest_issue_for_path(manifest, artifact.relative_path).is_some() {
                continue;
            }

            match artifact.presence {
                crate::ArtifactPresence::Missing => blockers.push(Blocker {
                    category: BlockerCategory::RequiredArtifactMissing,
                    subject: SubjectRef::CanonicalArtifact {
                        kind: artifact.kind,
                        canonical_repo_relative_path: artifact.relative_path,
                    },
                    summary: "missing required canonical artifact".to_string(),
                    next_safe_action: NextSafeAction::CreateCanonicalArtifact {
                        canonical_repo_relative_path: artifact.relative_path,
                    },
                }),
                crate::ArtifactPresence::PresentEmpty => blockers.push(Blocker {
                    category: BlockerCategory::RequiredArtifactEmpty,
                    subject: SubjectRef::CanonicalArtifact {
                        kind: artifact.kind,
                        canonical_repo_relative_path: artifact.relative_path,
                    },
                    summary: "required canonical artifact is empty".to_string(),
                    next_safe_action: NextSafeAction::FillCanonicalArtifact {
                        canonical_repo_relative_path: artifact.relative_path,
                    },
                }),
                crate::ArtifactPresence::PresentNonEmpty => {}
            }
        }
    }

    let has_forbidden_override = manifest
        .freshness
        .issues
        .iter()
        .any(|issue| matches!(issue.kind, FreshnessIssueKind::ForbiddenOverride));
    if has_forbidden_override {
        blockers.push(Blocker {
            category: BlockerCategory::FreshnessInvalid,
            subject: SubjectRef::Policy {
                policy_id: "freshness",
            },
            summary: "freshness truth is invalid (forbidden override)".to_string(),
            next_safe_action: NextSafeAction::RunDoctor,
        });
    }

    if budget_outcome.disposition == BudgetDisposition::Refuse {
        let canonical_repo_relative_path = match budget_outcome.next_safe_action.as_ref() {
            Some(BudgetNextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            }) => *canonical_repo_relative_path,
            None => SYSTEM_ROOT_PATH,
        };

        blockers.push(Blocker {
            category: BlockerCategory::BudgetRefused,
            subject: SubjectRef::Policy {
                policy_id: "budget",
            },
            summary: "budget refused packet generation".to_string(),
            next_safe_action: NextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            },
        });
    }

    if blockers.is_empty() && request.packet_id == LIVE_EXECUTION_PACKET_ID {
        blockers.push(Blocker {
            category: BlockerCategory::UnsupportedRequest,
            subject: SubjectRef::Policy {
                policy_id: "packet_id",
            },
            summary:
                "unsupported live slice execution request: reduced v1 supports live planning packets plus fixture-backed execution demos only"
                    .to_string(),
            next_safe_action: NextSafeAction::RunGenerate {
                packet_id: DEFAULT_PACKET_ID,
            },
        });
    }

    blockers.sort_by(|a, b| {
        let cat = blocker_category_priority(a.category).cmp(&blocker_category_priority(b.category));
        if cat != Ordering::Equal {
            return cat;
        }

        let subj = cmp_subject(&a.subject, &b.subject);
        if subj != Ordering::Equal {
            return subj;
        }

        a.summary.cmp(&b.summary)
    });
    blockers
}

fn cmp_subject(a: &SubjectRef, b: &SubjectRef) -> Ordering {
    let kind_a = subject_kind_priority(a);
    let kind_b = subject_kind_priority(b);
    let kind_cmp = kind_a.cmp(&kind_b);
    if kind_cmp != Ordering::Equal {
        return kind_cmp;
    }

    match (a, b) {
        (
            SubjectRef::CanonicalArtifact {
                kind: kind_a,
                canonical_repo_relative_path: path_a,
            },
            SubjectRef::CanonicalArtifact {
                kind: kind_b,
                canonical_repo_relative_path: path_b,
            },
        ) => (canonical_artifact_kind_priority(*kind_a), path_a)
            .cmp(&(canonical_artifact_kind_priority(*kind_b), path_b)),
        (
            SubjectRef::InheritedDependency {
                dependency_id: id_a,
                version: ver_a,
            },
            SubjectRef::InheritedDependency {
                dependency_id: id_b,
                version: ver_b,
            },
        ) => (id_a, ver_a).cmp(&(id_b, ver_b)),
        (SubjectRef::Policy { policy_id: id_a }, SubjectRef::Policy { policy_id: id_b }) => {
            id_a.cmp(id_b)
        }
        _ => Ordering::Equal,
    }
}

fn subject_kind_priority(subject: &SubjectRef) -> u8 {
    match subject {
        SubjectRef::CanonicalArtifact { .. } => 0,
        SubjectRef::InheritedDependency { .. } => 1,
        SubjectRef::Policy { .. } => 2,
    }
}

fn canonical_artifact_kind_priority(kind: CanonicalArtifactKind) -> u8 {
    match kind {
        CanonicalArtifactKind::Charter => 0,
        CanonicalArtifactKind::ProjectContext => 1,
        CanonicalArtifactKind::FeatureSpec => 2,
    }
}
