use crate::{doctor_rendering, shell_shared::discover_managed_repo_root, DoctorArgs};
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
        match doctor_rendering::render_json(&report) {
            Ok(json) => print!("{json}"),
            Err(err) => {
                println!("INVALID_BASELINE");
                println!("SUMMARY: {err}");
                return ExitCode::from(1);
            }
        }
    } else {
        print!("{}", doctor_rendering::render_text(&report));
    }

    if report.status == handbook_compiler::DoctorBaselineStatus::BaselineComplete {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
