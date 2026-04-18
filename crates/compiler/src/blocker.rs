use crate::{NextSafeAction, SubjectRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockerCategory {
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
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
        BlockerCategory::ArtifactReadError => 6,
        BlockerCategory::FreshnessInvalid => 7,
        BlockerCategory::BudgetRefused => 8,
        BlockerCategory::UnsupportedRequest => 9,
    }
}
