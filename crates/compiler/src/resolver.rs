use crate::budget::evaluate_budget;
use crate::{
    ArtifactManifest, BudgetDisposition, BudgetOutcome, BudgetPolicy, CompilerError, DecisionLog,
    FreshnessStatus, ManifestInputs, RefusalPlaceholder,
};
use std::path::Path;

const C04_RESULT_VERSION: &str = "reduced-v1";
const DEFAULT_PACKET_ID: &str = "planning.packet";

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
    pub refusal: Option<RefusalPlaceholder>,
    pub blockers: Vec<BlockerPlaceholder>,
}

pub fn resolve(repo_root: impl AsRef<Path>, request: ResolveRequest) -> Result<ResolverResult, CompilerError> {
    let manifest = ArtifactManifest::generate(repo_root.as_ref(), ManifestInputs::default())
        .map_err(CompilerError::Manifest)?;

    let mut decision_log = DecisionLog { entries: Vec::new() };

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

    let selection_status = if manifest.freshness.status == FreshnessStatus::Ok
        && budget_outcome.disposition != BudgetDisposition::Refuse
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
        refusal: None,
        blockers: Vec::new(),
    })
}
