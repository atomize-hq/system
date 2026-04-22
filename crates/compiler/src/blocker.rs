use crate::{NextSafeAction, SubjectRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockerCategory {
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
    RequiredArtifactInvalid,
    ArtifactReadError,
    FreshnessInvalid,
    BudgetRefused,
    UnsupportedRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blocker {
    pub category: BlockerCategory,
    pub subject: SubjectRef,
    pub summary: String,
    pub next_safe_action: NextSafeAction,
}

pub fn blocker_category_priority(category: BlockerCategory) -> u8 {
    match category {
        BlockerCategory::SystemRootMissing => 0,
        BlockerCategory::SystemRootSymlinkNotAllowed => 1,
        BlockerCategory::SystemRootNotDir => 2,
        BlockerCategory::RequiredArtifactMissing => 3,
        BlockerCategory::RequiredArtifactEmpty => 4,
        BlockerCategory::RequiredArtifactStarterTemplate => 5,
        BlockerCategory::RequiredArtifactInvalid => 6,
        BlockerCategory::ArtifactReadError => 7,
        BlockerCategory::FreshnessInvalid => 8,
        BlockerCategory::BudgetRefused => 9,
        BlockerCategory::UnsupportedRequest => 10,
    }
}
