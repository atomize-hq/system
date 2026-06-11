use super::{discover_managed_repo_root, DoctorArgs};
use std::process::ExitCode;

pub(super) fn run(args: DoctorArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let report = match handbook_compiler::doctor(&repo_root) {
        Ok(report) => report,
        Err(err) => {
            println!("INVALID_BASELINE");
            println!("SUMMARY: failed to inspect baseline truth: {err}");
            return ExitCode::from(1);
        }
    };

    if args.json {
        match render_json(&report) {
            Ok(json) => print!("{json}"),
            Err(err) => {
                println!("INVALID_BASELINE");
                println!("SUMMARY: {err}");
                return ExitCode::from(1);
            }
        }
    } else {
        println!("{}", doctor_status_name(report.status));
        println!(
            "ROOT STATUS: {}",
            doctor_root_status_name(report.system_root_status)
        );
        if let Some(next_safe_action) = &report.next_safe_action {
            println!(
                "NEXT SAFE ACTION: {}",
                handbook_compiler::render_next_safe_action_value(next_safe_action)
            );
        } else {
            println!("NEXT SAFE ACTION: <none>");
        }
        println!("## BASELINE CHECKLIST");
        for item in &report.checklist {
            let subject_path = match &item.subject {
                handbook_compiler::SubjectRef::CanonicalArtifact {
                    canonical_repo_relative_path,
                    ..
                } => *canonical_repo_relative_path,
                _ => item.canonical_repo_relative_path,
            };
            println!(
                "{} [{}] STATUS: {} ACTION: {}",
                item.artifact_label,
                subject_path,
                doctor_artifact_status_name(item.status),
                item.author_command
            );
        }
    }

    if report.status == handbook_compiler::DoctorBaselineStatus::BaselineComplete {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn render_json(report: &handbook_compiler::DoctorReport) -> Result<String, String> {
    serde_json::to_string_pretty(report)
        .map(|mut output| {
            output.push('\n');
            output
        })
        .map_err(|err| format!("failed to serialize doctor json: {err}"))
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
