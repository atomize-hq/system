use crate::{rendering, request_shared, shell_shared::discover_managed_repo_root, RequestArgs};
use std::path::Path;
use std::process::ExitCode;

pub(super) fn run(args: RequestArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let request = match request_shared::prepare_request(&args, &repo_root) {
        Ok(request) => request,
        Err(err) => {
            println!("BLOCKED: {err}");
            return ExitCode::from(1);
        }
    };

    let result = match handbook_flow::resolve(
        &request.compiler_root,
        handbook_flow::ResolveRequest {
            packet_id: request.packet_id.as_str(),
            ..handbook_flow::ResolveRequest::default()
        },
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("BLOCKED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    let output = match rendering::prepare_flow_output(result) {
        Ok(output) => output,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    if output.is_ready() {
        println!("{}", output.render_inspect());
    } else {
        let rendered = output.render_inspect();
        if let Some(fixture_set_id) = request.demo_fixture_set_id.as_deref() {
            let section = fixture_section_for_demo(&repo_root, fixture_set_id);
            println!("{}", inject_after_first_three_lines(&rendered, &section));
        } else {
            println!("{rendered}");
        }
    }

    output.exit_code()
}

fn fixture_lineage_for_demo(repo_root: &Path, fixture_set_id: &str) -> Vec<String> {
    let base =
        request_shared::execution_demo_fixture_set_dir(repo_root, fixture_set_id).join(".handbook");
    let project_context = base.join("project_context/PROJECT_CONTEXT.md");

    let mut out = Vec::new();
    out.push(format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/charter/CHARTER.md"
    ));
    if project_context.is_file() {
        out.push(format!(
            "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/project_context/PROJECT_CONTEXT.md"
        ));
    }
    out.push(format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/feature_spec/FEATURE_SPEC.md"
    ));
    out
}

fn fixture_section_for_demo(repo_root: &Path, fixture_set_id: &str) -> String {
    let mut out = String::new();
    out.push_str("MODE: fixture-backed execution demo\n");
    out.push_str("## FIXTURE DEMO\n");
    out.push_str(&format!("FIXTURE SET: {fixture_set_id}\n"));
    out.push_str(&format!(
        "FIXTURE BASIS ROOT: tests/fixtures/execution_demo/{fixture_set_id}/.handbook/\n"
    ));
    out.push_str("FIXTURE LINEAGE:\n");
    for (index, item) in fixture_lineage_for_demo(repo_root, fixture_set_id)
        .iter()
        .enumerate()
    {
        out.push_str(&format!("{}. {}\n", index + 1, item));
    }
    out
}

fn inject_after_first_three_lines(rendered: &str, injection: &str) -> String {
    let mut lines: Vec<&str> = rendered.split('\n').collect();
    let insert_at = 3.min(lines.len());
    lines.insert(insert_at, injection.trim_end_matches('\n'));
    lines.join("\n")
}
