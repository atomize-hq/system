use crate::{
    blocker::Blocker,
    budget::{BudgetDisposition, BudgetReason, NextSafeAction as BudgetNextSafeAction},
    refusal::{NextSafeAction, RefusalCategory, SubjectRef},
    BlockerCategory, CanonicalArtifactKind, PacketSelectionStatus,
};

pub fn push_line(output: &mut String, line: impl AsRef<str>) {
    output.push_str(line.as_ref());
    output.push('\n');
}

pub fn render_outcome(packet_status: PacketSelectionStatus, refusal_present: bool) -> &'static str {
    if refusal_present {
        return "REFUSED";
    }

    match packet_status {
        PacketSelectionStatus::Selected => "READY",
        PacketSelectionStatus::Blocked => "BLOCKED",
    }
}

pub fn render_next_safe_action_from_model(
    refusal: Option<&crate::Refusal>,
    blockers: &[Blocker],
) -> String {
    if let Some(refusal) = refusal {
        return render_next_safe_action_value(&refusal.next_safe_action);
    }

    if let Some(blocker) = blockers.first() {
        return render_next_safe_action_value(&blocker.next_safe_action);
    }

    "render packet body once implemented (SEAM-5)".to_string()
}

pub fn render_next_safe_action_value(action: &NextSafeAction) -> String {
    match action {
        NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        } => format!("create canonical .system root at {canonical_repo_relative_path}"),
        NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => format!(
            "ensure canonical .system root is a directory at {canonical_repo_relative_path}"
        ),
        NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => format!("remove canonical .system symlink at {canonical_repo_relative_path}"),
        NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => format!("create canonical artifact at {canonical_repo_relative_path}"),
        NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => format!("fill canonical artifact at {canonical_repo_relative_path}"),
        NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => format!("reduce canonical artifact size at {canonical_repo_relative_path}"),
        NextSafeAction::RunDoctor => "run `doctor`".to_string(),
    }
}

pub fn render_refusal_category(category: RefusalCategory) -> &'static str {
    match category {
        RefusalCategory::NonCanonicalInputAttempt => "NonCanonicalInputAttempt",
        RefusalCategory::SystemRootMissing => "SystemRootMissing",
        RefusalCategory::SystemRootNotDir => "SystemRootNotDir",
        RefusalCategory::SystemRootSymlinkNotAllowed => "SystemRootSymlinkNotAllowed",
        RefusalCategory::RequiredArtifactMissing => "RequiredArtifactMissing",
        RefusalCategory::RequiredArtifactEmpty => "RequiredArtifactEmpty",
        RefusalCategory::ArtifactReadError => "ArtifactReadError",
        RefusalCategory::FreshnessInvalid => "FreshnessInvalid",
        RefusalCategory::BudgetRefused => "BudgetRefused",
        RefusalCategory::UnsupportedRequest => "UnsupportedRequest",
    }
}

pub fn render_blocker_category(category: BlockerCategory) -> &'static str {
    match category {
        BlockerCategory::SystemRootMissing => "SystemRootMissing",
        BlockerCategory::SystemRootNotDir => "SystemRootNotDir",
        BlockerCategory::SystemRootSymlinkNotAllowed => "SystemRootSymlinkNotAllowed",
        BlockerCategory::RequiredArtifactMissing => "RequiredArtifactMissing",
        BlockerCategory::RequiredArtifactEmpty => "RequiredArtifactEmpty",
        BlockerCategory::ArtifactReadError => "ArtifactReadError",
        BlockerCategory::FreshnessInvalid => "FreshnessInvalid",
        BlockerCategory::BudgetRefused => "BudgetRefused",
        BlockerCategory::UnsupportedRequest => "UnsupportedRequest",
    }
}

pub fn render_subject_ref(subject: &SubjectRef) -> String {
    match subject {
        SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        } => format!(
            "canonical artifact {} at {}",
            render_canonical_artifact_kind(*kind),
            canonical_repo_relative_path
        ),
        SubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => match version {
            Some(version) => format!("inherited dependency {dependency_id}@{version}"),
            None => format!("inherited dependency {dependency_id}"),
        },
        SubjectRef::Policy { policy_id } => format!("policy {policy_id}"),
    }
}

pub fn render_canonical_artifact_kind(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "Charter",
        CanonicalArtifactKind::ProjectContext => "ProjectContext",
        CanonicalArtifactKind::FeatureSpec => "FeatureSpec",
    }
}

pub fn render_budget_disposition(disposition: BudgetDisposition) -> &'static str {
    match disposition {
        BudgetDisposition::Keep => "Keep",
        BudgetDisposition::Summarize => "Summarize",
        BudgetDisposition::Exclude => "Exclude",
        BudgetDisposition::Refuse => "Refuse",
    }
}

pub fn render_budget_reason(reason: &BudgetReason) -> &'static str {
    match reason {
        BudgetReason::WithinBudget => "WithinBudget",
        BudgetReason::OptionalArtifactTooLarge => "OptionalArtifactTooLarge",
        BudgetReason::TotalBytesExceeded => "TotalBytesExceeded",
        BudgetReason::RequiredArtifactTooLarge => "RequiredArtifactTooLarge",
    }
}

pub fn render_budget_next_safe_action(action: Option<&BudgetNextSafeAction>) -> Option<String> {
    action.map(|action| match action {
        BudgetNextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => format!("reduce canonical artifact size at {canonical_repo_relative_path}"),
    })
}

pub fn render_packet_status(status: PacketSelectionStatus) -> &'static str {
    match status {
        PacketSelectionStatus::Selected => "Selected",
        PacketSelectionStatus::Blocked => "Blocked",
    }
}

pub fn json_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len() + 8);
    for character in input.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            c if c.is_control() => {
                use std::fmt::Write;
                write!(&mut escaped, "\\u{:04x}", c as u32).expect("json escape");
            }
            other => escaped.push(other),
        }
    }
    escaped
}

pub fn json_string(input: &str) -> String {
    format!("\"{}\"", json_escape(input))
}
