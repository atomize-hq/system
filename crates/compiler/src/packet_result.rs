use crate::budget::{BudgetDisposition, BudgetReason};
use crate::resolver::PacketSelectionStatus;
use crate::{ArtifactPresence, CanonicalArtifactKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketVariant {
    Planning,
    ExecutionDemo,
    ExecutionLive,
}

impl PacketVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            PacketVariant::Planning => "planning.packet",
            PacketVariant::ExecutionDemo => "execution.demo.packet",
            PacketVariant::ExecutionLive => "execution.live.packet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketSourceSummary {
    pub kind: CanonicalArtifactKind,
    pub canonical_repo_relative_path: &'static str,
    pub required: bool,
    pub presence: ArtifactPresence,
    pub byte_len: Option<u64>,
    pub content_sha256: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketBodyNoteKind {
    Omission,
    Budget,
    InheritedDependency,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketBodyNote {
    pub kind: PacketBodyNoteKind,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketSection {
    pub kind: CanonicalArtifactKind,
    pub canonical_repo_relative_path: &'static str,
    pub title: String,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketFixtureContext {
    pub fixture_set_id: String,
    pub fixture_basis_root: String,
    pub fixture_lineage: Vec<PacketSourceSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketDecisionSummary {
    pub packet_status: PacketSelectionStatus,
    pub budget_disposition: BudgetDisposition,
    pub budget_reason: BudgetReason,
    pub decision_log_entries: usize,
    pub summary_line: String,
    pub ready_next_safe_action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketResult {
    pub packet_id: String,
    pub variant: PacketVariant,
    pub fixture_context: Option<PacketFixtureContext>,
    pub included_sources: Vec<PacketSourceSummary>,
    pub notes: Vec<PacketBodyNote>,
    pub decision_summary: PacketDecisionSummary,
    pub sections: Vec<PacketSection>,
}

impl PacketResult {
    pub fn is_ready(&self) -> bool {
        self.decision_summary.packet_status == PacketSelectionStatus::Selected
    }
}
