use std::fs;
use std::path::{Path, PathBuf};

use system_compiler::{
    ResolvedPipelineRoute, RouteStageReason, RouteState, RouteStateMutationOutcome,
};

pub const FOUNDATION_INPUTS_PIPELINE_ID: &str = "pipeline.foundation_inputs";

pub fn committed_repo_root() -> PathBuf {
    committed_case_root().join("repo")
}

pub fn install_foundation_inputs_repo() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    copy_tree(&committed_repo_root(), &root);
    (dir, root)
}

pub fn install_state_seed(repo_root: &Path, seed_name: &str) -> PathBuf {
    let source = committed_case_root().join("state_seeds").join(seed_name);
    let target = repo_root
        .join(".system")
        .join("state")
        .join("pipeline")
        .join(format!("{FOUNDATION_INPUTS_PIPELINE_ID}.yaml"));

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("state seed parent");
    }

    fs::copy(&source, &target).unwrap_or_else(|err| {
        panic!(
            "copy state seed {} -> {}: {err}",
            source.display(),
            target.display()
        )
    });
    target
}

pub fn assert_matches_golden(
    actual: &str,
    repo_root: &Path,
    state_path: Option<&Path>,
    golden_name: &str,
) {
    let normalized_actual = normalize_output(actual, repo_root, state_path);
    let expected = read_golden(golden_name);
    assert_eq!(
        normalized_actual,
        expected,
        "pipeline proof output drifted for {golden_name}; update the golden at {} if intentional",
        committed_case_root()
            .join("goldens")
            .join(golden_name)
            .display()
    );
}

pub fn render_pipeline_resolve_output(
    pipeline_id: &str,
    state: &RouteState,
    route: &ResolvedPipelineRoute,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: RESOLVED\n");
    out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
    out.push_str("ROUTE BASIS:\n");
    out.push_str(&format!("  revision = {}\n", state.revision));
    out.push_str("  routing:\n");
    if state.routing.is_empty() {
        out.push_str("    <empty>\n");
    } else {
        for (name, value) in &state.routing {
            out.push_str(&format!("    {} = {}\n", name, value));
        }
    }
    out.push_str("  refs:\n");
    render_optional_route_basis_field(&mut out, "charter_ref", state.refs.charter_ref.as_deref());
    render_optional_route_basis_field(
        &mut out,
        "project_context_ref",
        state.refs.project_context_ref.as_deref(),
    );
    out.push_str("  run:\n");
    render_optional_route_basis_field(&mut out, "runner", state.run.runner.as_deref());
    render_optional_route_basis_field(&mut out, "profile", state.run.profile.as_deref());
    render_optional_route_basis_field(&mut out, "repo_root", state.run.repo_root.as_deref());
    out.push_str("ROUTE:\n");

    for (index, stage) in route.stages.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {}\n",
            index + 1,
            stage.stage_id,
            stage.status.as_str()
        ));
        if let Some(reason) = &stage.reason {
            out.push_str(&format!(
                "     REASON: {}\n",
                render_route_stage_reason(reason)
            ));
        }
    }

    out.trim_end().to_string()
}

pub fn render_pipeline_state_set_output(
    pipeline_id: &str,
    outcome: RouteStateMutationOutcome,
) -> String {
    let mut out = String::new();
    match outcome {
        RouteStateMutationOutcome::Applied(state) => {
            out.push_str("OUTCOME: APPLIED\n");
            out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
            out.push_str(&format!("REVISION: {}\n", state.revision));
            out.push_str("ROUTING:\n");
            if state.routing.is_empty() {
                out.push_str("  <empty>\n");
            } else {
                for (name, value) in state.routing {
                    out.push_str(&format!("  {} = {}\n", name, value));
                }
            }
            out.push_str("REFS:\n");
            render_optional_state_field(&mut out, "charter_ref", state.refs.charter_ref.as_deref());
            render_optional_state_field(
                &mut out,
                "project_context_ref",
                state.refs.project_context_ref.as_deref(),
            );
            out.push_str("RUN:\n");
            render_optional_state_field(&mut out, "runner", state.run.runner.as_deref());
            render_optional_state_field(&mut out, "profile", state.run.profile.as_deref());
            render_optional_state_field(&mut out, "repo_root", state.run.repo_root.as_deref());
        }
        RouteStateMutationOutcome::Refused(refusal) => {
            out.push_str("OUTCOME: REFUSED\n");
            out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
            out.push_str(&format!("REASON: {}\n", refusal));
        }
    }

    out.trim_end().to_string()
}

pub fn render_load_route_state_refusal(err: impl std::fmt::Display) -> String {
    format!("REFUSED: {err}")
}

fn render_optional_route_basis_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("    {} = {}\n", name, value)),
        None => out.push_str(&format!("    {} = <unset>\n", name)),
    }
}

fn render_optional_state_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("  {} = {}\n", name, value)),
        None => out.push_str(&format!("  {} = <unset>\n", name)),
    }
}

fn render_route_stage_reason(reason: &RouteStageReason) -> String {
    match reason {
        RouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        } => format!(
            "activation evaluated false for variables: {}",
            unsatisfied_variables.join(", ")
        ),
        RouteStageReason::NextMissingRouteVariables {
            missing_variables, ..
        } => format!("missing route variables: {}", missing_variables.join(", ")),
        RouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => format!(
            "blocked by unresolved stage {} ({})",
            upstream_stage_id,
            upstream_status.as_str()
        ),
    }
}

fn normalize_output(actual: &str, repo_root: &Path, state_path: Option<&Path>) -> String {
    let mut normalized = normalize_newlines(actual);

    if let Some(state_path) = state_path {
        for candidate in path_candidates(state_path) {
            normalized = normalized.replace(&candidate, "{{STATE_PATH}}");
        }
    }

    for candidate in path_candidates(repo_root) {
        normalized = normalized.replace(&candidate, "{{REPO_ROOT}}");
    }
    normalized.trim_end().to_string()
}

fn read_golden(golden_name: &str) -> String {
    let golden_path = committed_case_root().join("goldens").join(golden_name);
    normalize_newlines(
        &fs::read_to_string(&golden_path)
            .unwrap_or_else(|err| panic!("read {}: {err}", golden_path.display())),
    )
    .trim_end()
    .to_string()
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

fn path_candidates(path: &Path) -> Vec<String> {
    let mut candidates = vec![path.display().to_string()];
    if let Ok(canonical) = fs::canonicalize(path) {
        let canonical_display = canonical.display().to_string();
        if !candidates.contains(&canonical_display) {
            candidates.push(canonical_display);
        }
    }
    candidates.sort_by_key(|candidate| std::cmp::Reverse(candidate.len()));
    candidates
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap_or_else(|err| panic!("mkdir {}: {err}", target.display()));

    for entry in
        fs::read_dir(source).unwrap_or_else(|err| panic!("read_dir {}: {err}", source.display()))
    {
        let entry =
            entry.unwrap_or_else(|err| panic!("dir entry under {}: {err}", source.display()));
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let file_type = entry
            .file_type()
            .unwrap_or_else(|err| panic!("file_type {}: {err}", source_path.display()));

        if file_type.is_dir() {
            copy_tree(&source_path, &target_path);
        } else if file_type.is_file() {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)
                    .unwrap_or_else(|err| panic!("mkdir {}: {err}", parent.display()));
            }
            fs::copy(&source_path, &target_path).unwrap_or_else(|err| {
                panic!(
                    "copy {} -> {}: {err}",
                    source_path.display(),
                    target_path.display()
                )
            });
        }
    }
}

fn committed_case_root() -> PathBuf {
    workspace_root()
        .join("tests")
        .join("fixtures")
        .join("pipeline_proof_corpus")
        .join("foundation_inputs")
}

fn workspace_root() -> PathBuf {
    let start = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in start.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if !cargo_toml.is_file() {
            continue;
        }
        let Ok(contents) = fs::read_to_string(&cargo_toml) else {
            continue;
        };
        if contents.contains("[workspace]") {
            return ancestor.to_path_buf();
        }
    }

    panic!(
        "failed to locate workspace root from CARGO_MANIFEST_DIR={}",
        env!("CARGO_MANIFEST_DIR")
    );
}
