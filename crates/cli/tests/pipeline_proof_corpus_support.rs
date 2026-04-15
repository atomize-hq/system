use std::fs;
use std::path::{Path, PathBuf};

pub const FOUNDATION_INPUTS_PIPELINE_ID: &str = "pipeline.foundation_inputs";

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
    let mut placeholders = Vec::new();
    if let Some(state_path) = state_path {
        placeholders.push((state_path, "{{STATE_PATH}}"));
    }
    assert_matches_golden_with_placeholders(actual, repo_root, &placeholders, golden_name);
}

pub fn assert_matches_golden_with_explicit_placeholders(
    actual: &str,
    placeholders: &[(&Path, &str)],
    golden_name: &str,
) {
    let normalized_actual = normalize_output_with_explicit_placeholders(actual, placeholders);
    assert_matches_normalized_output(&normalized_actual, golden_name);
}

pub fn assert_matches_golden_with_placeholders(
    actual: &str,
    repo_root: &Path,
    placeholders: &[(&Path, &str)],
    golden_name: &str,
) {
    let normalized_actual = normalize_output(actual, repo_root, placeholders);
    assert_matches_normalized_output(&normalized_actual, golden_name);
}

fn assert_matches_normalized_output(actual: &str, golden_name: &str) {
    let expected = read_golden(golden_name);
    assert_eq!(
        actual,
        expected,
        "pipeline proof output drifted for {golden_name}; update the golden at {} if intentional",
        committed_case_root()
            .join("goldens")
            .join(golden_name)
            .display()
    );
}

fn normalize_output(actual: &str, repo_root: &Path, placeholders: &[(&Path, &str)]) -> String {
    let mut normalized = normalize_output_with_explicit_placeholders(actual, placeholders);
    for candidate in path_candidates(repo_root) {
        normalized = normalized.replace(&candidate, "{{REPO_ROOT}}");
    }
    normalized.trim_end().to_string()
}

fn normalize_output_with_explicit_placeholders(
    actual: &str,
    placeholders: &[(&Path, &str)],
) -> String {
    let mut normalized = normalize_newlines(actual);

    for (path, placeholder) in placeholders {
        for candidate in path_candidates(path) {
            normalized = normalized.replace(&candidate, placeholder);
        }
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

fn committed_repo_root() -> PathBuf {
    committed_case_root().join("repo")
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
