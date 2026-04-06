use crate::{ArtifactPresence, CanonicalArtifactIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetDisposition {
    Keep,
    Summarize,
    Exclude,
    Refuse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetReason {
    WithinBudget,
    OptionalArtifactTooLarge,
    TotalBytesExceeded,
    RequiredArtifactTooLarge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextSafeAction {
    ReduceCanonicalArtifactSize {
        canonical_repo_relative_path: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetTarget {
    pub canonical_repo_relative_path: &'static str,
    pub byte_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetOutcome {
    pub disposition: BudgetDisposition,
    pub reason: BudgetReason,
    pub targets: Vec<BudgetTarget>,
    /// Present only when `disposition == Refuse`.
    pub next_safe_action: Option<NextSafeAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetPolicy {
    pub max_total_bytes: Option<u64>,
    pub max_per_artifact_bytes: Option<u64>,
}

impl Default for BudgetPolicy {
    fn default() -> Self {
        // Default to "effectively unlimited" so budget policy does not
        // accidentally refuse on normal repos until the contract hardens.
        Self {
            max_total_bytes: None,
            max_per_artifact_bytes: None,
        }
    }
}

pub fn evaluate_budget(
    artifacts: &[CanonicalArtifactIdentity],
    policy: BudgetPolicy,
) -> BudgetOutcome {
    let present: Vec<(&CanonicalArtifactIdentity, u64)> = artifacts
        .iter()
        .filter_map(|artifact| match artifact.presence {
            ArtifactPresence::Missing => None,
            ArtifactPresence::PresentEmpty | ArtifactPresence::PresentNonEmpty => {
                artifact.byte_len.map(|len| (artifact, len))
            }
        })
        .collect();

    if let Some(max_per) = policy.max_per_artifact_bytes {
        for (artifact, len) in &present {
            if *len <= max_per {
                continue;
            }

            if artifact.required {
                return BudgetOutcome {
                    disposition: BudgetDisposition::Refuse,
                    reason: BudgetReason::RequiredArtifactTooLarge,
                    targets: vec![BudgetTarget {
                        canonical_repo_relative_path: artifact.relative_path,
                        byte_len: *len,
                    }],
                    next_safe_action: Some(NextSafeAction::ReduceCanonicalArtifactSize {
                        canonical_repo_relative_path: artifact.relative_path,
                    }),
                };
            }

            return BudgetOutcome {
                disposition: BudgetDisposition::Summarize,
                reason: BudgetReason::OptionalArtifactTooLarge,
                targets: vec![BudgetTarget {
                    canonical_repo_relative_path: artifact.relative_path,
                    byte_len: *len,
                }],
                next_safe_action: None,
            };
        }
    }

    if let Some(max_total) = policy.max_total_bytes {
        let total: u64 = present.iter().map(|(_, len)| *len).sum();
        if total > max_total {
            // Prefer excluding optional artifacts first, in canonical order.
            for (artifact, len) in &present {
                if artifact.required {
                    continue;
                }
                return BudgetOutcome {
                    disposition: BudgetDisposition::Exclude,
                    reason: BudgetReason::TotalBytesExceeded,
                    targets: vec![BudgetTarget {
                        canonical_repo_relative_path: artifact.relative_path,
                        byte_len: *len,
                    }],
                    next_safe_action: None,
                };
            }

            // No optional artifacts to exclude; refuse and point at the largest required artifact.
            let mut largest_required: Option<(&CanonicalArtifactIdentity, u64)> = None;
            for (artifact, len) in &present {
                if !artifact.required {
                    continue;
                }
                match largest_required {
                    None => largest_required = Some((artifact, *len)),
                    Some((_, best_len)) if *len > best_len => {
                        largest_required = Some((artifact, *len))
                    }
                    _ => {}
                }
            }

            if let Some((artifact, len)) = largest_required {
                return BudgetOutcome {
                    disposition: BudgetDisposition::Refuse,
                    reason: BudgetReason::TotalBytesExceeded,
                    targets: vec![BudgetTarget {
                        canonical_repo_relative_path: artifact.relative_path,
                        byte_len: len,
                    }],
                    next_safe_action: Some(NextSafeAction::ReduceCanonicalArtifactSize {
                        canonical_repo_relative_path: artifact.relative_path,
                    }),
                };
            }
        }
    }

    BudgetOutcome {
        disposition: BudgetDisposition::Keep,
        reason: BudgetReason::WithinBudget,
        targets: Vec::new(),
        next_safe_action: None,
    }
}
