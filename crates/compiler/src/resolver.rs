use crate::budget::evaluate_budget;
use crate::{
    ArtifactManifest, BudgetDisposition, BudgetOutcome, BudgetPolicy, CompilerError, DecisionLog,
    BudgetNextSafeAction, FreshnessIssueKind, FreshnessStatus, ManifestInputs, NextSafeAction,
    Refusal, RefusalCategory, SubjectRef, SystemRootStatus,
};
use std::path::Path;

const C04_RESULT_VERSION: &str = "reduced-v1";
const DEFAULT_PACKET_ID: &str = "planning.packet";
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
pub struct BlockerPlaceholder {
    pub summary: String,
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
    pub blockers: Vec<BlockerPlaceholder>,
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

    let refusal = compute_refusal(&manifest, &budget_outcome);
    if let Some(refusal) = &refusal {
        decision_log.entries.push(format!(
            "refusal category={:?} broken_subject={:?} next_safe_action={:?}",
            refusal.category, refusal.broken_subject, refusal.next_safe_action
        ));
    }

    let selection_status = if manifest.freshness.status == FreshnessStatus::Ok
        && budget_outcome.disposition != BudgetDisposition::Refuse
        && refusal.is_none()
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
        blockers: Vec::new(),
    })
}

fn compute_refusal(manifest: &ArtifactManifest, budget_outcome: &BudgetOutcome) -> Option<Refusal> {
    match manifest.system_root_status {
        SystemRootStatus::Ok => {}
        SystemRootStatus::Missing => {
            return Some(Refusal {
                category: RefusalCategory::SystemRootMissing,
                summary: "missing canonical .system root".to_string(),
                broken_subject: SubjectRef::SystemRoot {
                    canonical_repo_relative_path: SYSTEM_ROOT_PATH,
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
                broken_subject: SubjectRef::SystemRoot {
                    canonical_repo_relative_path: SYSTEM_ROOT_PATH,
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
                broken_subject: SubjectRef::SystemRoot {
                    canonical_repo_relative_path: SYSTEM_ROOT_PATH,
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
        let issue_kind = manifest
            .freshness
            .issues
            .iter()
            .map(|issue| issue.kind)
            .find(|kind| matches!(kind, FreshnessIssueKind::ForbiddenOverride))
            .unwrap_or(FreshnessIssueKind::ForbiddenOverride);

        return Some(Refusal {
            category: RefusalCategory::FreshnessInvalid,
            summary: "freshness truth is invalid".to_string(),
            broken_subject: SubjectRef::FreshnessIssue { kind: issue_kind },
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

    None
}
