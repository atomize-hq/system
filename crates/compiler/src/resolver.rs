use crate::budget::evaluate_budget;
use crate::{
    blocker_category_priority, ArtifactManifest, Blocker, BlockerCategory, BudgetDisposition,
    BudgetOutcome, BudgetPolicy, BudgetNextSafeAction, CanonicalArtifactKind, CompilerError,
    DecisionLog, FreshnessIssueKind, FreshnessStatus, ManifestInputs, NextSafeAction, Refusal,
    RefusalCategory, SubjectRef, SystemRootStatus,
};
use std::path::Path;
use std::cmp::Ordering;

const C04_RESULT_VERSION: &str = "reduced-v1";
const DEFAULT_PACKET_ID: &str = "planning.packet";
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
    pub decision_log: DecisionLog,
    pub budget_outcome: BudgetOutcome,
    pub selection: PacketSelection,
    pub refusal: Option<Refusal>,
    pub blockers: Vec<Blocker>,
}

pub fn resolve(repo_root: impl AsRef<Path>, request: ResolveRequest) -> Result<ResolverResult, CompilerError> {
    let manifest = ArtifactManifest::generate(repo_root.as_ref(), ManifestInputs::default())
        .map_err(CompilerError::Manifest)?;

    let mut decision_log = DecisionLog { entries: Vec::new() };

    decision_log
        .entries
        .push(format!("c03.system_root status={:?}", manifest.system_root_status));

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

    Ok(ResolverResult {
        c04_result_version: C04_RESULT_VERSION.to_string(),
        c03_schema_version: manifest.version.schema.version.to_string(),
        c03_manifest_generation_version: manifest.version.generation,
        c03_fingerprint_sha256: manifest.freshness.fingerprint_sha256.clone(),
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

    for artifact in &manifest.artifacts {
        if !artifact.required {
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
            broken_subject: SubjectRef::Policy { policy_id: "budget" },
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

fn compute_blockers(
    manifest: &ArtifactManifest,
    budget_outcome: &BudgetOutcome,
    request: &ResolveRequest,
) -> Vec<Blocker> {
    let mut blockers = Vec::new();

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
            subject: SubjectRef::Policy { policy_id: "budget" },
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
        ) => (canonical_artifact_kind_priority(*kind_a), path_a).cmp(&(
            canonical_artifact_kind_priority(*kind_b),
            path_b,
        )),
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
