use super::model::RenderOutputModel;
use crate::{
    blocker::Blocker,
    refusal::{NextSafeAction, RefusalCategory, SubjectRef},
    BlockerCategory, CanonicalArtifactKind, PacketSelectionStatus,
};

pub fn render_markdown(model: &RenderOutputModel) -> String {
    let mut output = String::new();

    push_line(&mut output, format!("OUTCOME: {}", render_outcome(model)));
    push_line(&mut output, format!("OBJECT: {}", model.packet_id));
    push_line(
        &mut output,
        format!("NEXT SAFE ACTION: {}", render_next_safe_action(model)),
    );

    match model.refusal.as_ref() {
        Some(refusal) => {
            output.push('\n');
            push_line(&mut output, "## REFUSAL".to_string());
            push_line(
                &mut output,
                format!("CATEGORY: {}", render_refusal_category(refusal.category)),
            );
            push_line(&mut output, format!("SUMMARY: {}", refusal.summary));
            push_line(
                &mut output,
                format!("BROKEN SUBJECT: {}", render_subject_ref(&refusal.broken_subject)),
            );
            push_line(
                &mut output,
                format!(
                    "NEXT SAFE ACTION: {}",
                    render_next_safe_action_value(&refusal.next_safe_action)
                ),
            );
        }
        None if !model.blockers.is_empty() => {
            output.push('\n');
            push_line(&mut output, "## BLOCKERS".to_string());
            for (index, blocker) in model.blockers.iter().enumerate() {
                if index > 0 {
                    output.push('\n');
                }
                render_blocker(&mut output, blocker);
            }
        }
        None => {
            output.push('\n');
            push_line(&mut output, "## PACKET BODY".to_string());
            push_line(
                &mut output,
                "Packet body rendering is not implemented yet (owned by SEAM-5).".to_string(),
            );
        }
    }

    output
}

fn render_outcome(model: &RenderOutputModel) -> &'static str {
    if model.refusal.is_some() {
        return "REFUSED";
    }

    match model.packet_status {
        PacketSelectionStatus::Selected => "READY",
        PacketSelectionStatus::Blocked => "BLOCKED",
    }
}

fn render_next_safe_action(model: &RenderOutputModel) -> String {
    if let Some(refusal) = model.refusal.as_ref() {
        return render_next_safe_action_value(&refusal.next_safe_action);
    }

    if let Some(blocker) = model.blockers.first() {
        return render_next_safe_action_value(&blocker.next_safe_action);
    }

    "render packet body once implemented (SEAM-5)".to_string()
}

fn render_refusal_category(category: RefusalCategory) -> &'static str {
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

fn render_blocker(output: &mut String, blocker: &Blocker) {
    push_line(
        output,
        format!("CATEGORY: {}", render_blocker_category(blocker.category)),
    );
    push_line(output, format!("SUMMARY: {}", blocker.summary));
    push_line(
        output,
        format!("SUBJECT: {}", render_subject_ref(&blocker.subject)),
    );
    push_line(
        output,
        format!(
            "NEXT SAFE ACTION: {}",
            render_next_safe_action_value(&blocker.next_safe_action)
        ),
    );
}

fn render_blocker_category(category: BlockerCategory) -> &'static str {
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

fn render_next_safe_action_value(action: &NextSafeAction) -> String {
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

fn render_subject_ref(subject: &SubjectRef) -> String {
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

fn render_canonical_artifact_kind(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "Charter",
        CanonicalArtifactKind::ProjectContext => "ProjectContext",
        CanonicalArtifactKind::FeatureSpec => "FeatureSpec",
    }
}

fn push_line(output: &mut String, line: String) {
    output.push_str(&line);
    output.push('\n');
}
