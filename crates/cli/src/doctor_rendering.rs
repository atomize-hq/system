use std::fmt::Write;

pub(crate) fn render_json(report: &handbook_compiler::DoctorReport) -> Result<String, String> {
    serde_json::to_string_pretty(report)
        .map(|mut output| {
            output.push('\n');
            output
        })
        .map_err(|err| format!("failed to serialize doctor json: {err}"))
}

pub(crate) fn render_text(report: &handbook_compiler::DoctorReport) -> String {
    let mut output = String::new();
    writeln!(&mut output, "{}", doctor_status_name(report.status)).expect("string write");
    writeln!(
        &mut output,
        "ROOT STATUS: {}",
        doctor_root_status_name(report.system_root_status)
    )
    .expect("string write");
    if let Some(next_safe_action) = &report.next_safe_action {
        writeln!(
            &mut output,
            "NEXT SAFE ACTION: {}",
            handbook_compiler::render_next_safe_action_value(next_safe_action)
        )
        .expect("string write");
    } else {
        writeln!(&mut output, "NEXT SAFE ACTION: <none>").expect("string write");
    }
    writeln!(&mut output, "## BASELINE CHECKLIST").expect("string write");
    for item in &report.checklist {
        let subject_path = match &item.subject {
            handbook_compiler::SubjectRef::CanonicalArtifact {
                canonical_repo_relative_path,
                ..
            } => *canonical_repo_relative_path,
            _ => item.canonical_repo_relative_path,
        };
        writeln!(
            &mut output,
            "{} [{}] STATUS: {} ACTION: {}",
            item.artifact_label,
            subject_path,
            doctor_artifact_status_name(item.status),
            item.author_command
        )
        .expect("string write");
    }
    output
}

fn doctor_status_name(status: handbook_compiler::DoctorBaselineStatus) -> &'static str {
    match status {
        handbook_compiler::DoctorBaselineStatus::Scaffolded => "SCAFFOLDED",
        handbook_compiler::DoctorBaselineStatus::PartialBaseline => "PARTIAL_BASELINE",
        handbook_compiler::DoctorBaselineStatus::InvalidBaseline => "INVALID_BASELINE",
        handbook_compiler::DoctorBaselineStatus::BaselineComplete => "BASELINE_COMPLETE",
    }
}

fn doctor_root_status_name(status: handbook_compiler::SystemRootStatus) -> &'static str {
    match status {
        handbook_compiler::SystemRootStatus::Ok => "OK",
        handbook_compiler::SystemRootStatus::Missing => "MISSING",
        handbook_compiler::SystemRootStatus::NotDir => "NOT_DIR",
        handbook_compiler::SystemRootStatus::SymlinkNotAllowed => "SYMLINK_NOT_ALLOWED",
    }
}

fn doctor_artifact_status_name(status: handbook_compiler::DoctorArtifactStatus) -> &'static str {
    match status {
        handbook_compiler::DoctorArtifactStatus::Missing => "MISSING",
        handbook_compiler::DoctorArtifactStatus::Empty => "EMPTY",
        handbook_compiler::DoctorArtifactStatus::StarterOwned => "STARTER_OWNED",
        handbook_compiler::DoctorArtifactStatus::Invalid => "INVALID",
        handbook_compiler::DoctorArtifactStatus::ValidCanonicalTruth => "VALID_CANONICAL_TRUTH",
    }
}
