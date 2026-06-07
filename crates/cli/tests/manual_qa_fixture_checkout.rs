use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct PreparedCheckout {
    temp_parent: PathBuf,
    checkout_root: PathBuf,
    effective_cwd: PathBuf,
}

impl Drop for PreparedCheckout {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.temp_parent);
    }
}

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
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

fn prepare_fixture_checkout(relative_fixture_root: &str, nested_cwd: &str) -> PreparedCheckout {
    let workspace = workspace_root();
    let script = workspace.join("tools/qa/prepare_fixture_checkout.sh");
    let output = Command::new("bash")
        .arg(&script)
        .args([
            "--fixture-root",
            relative_fixture_root,
            "--nested-cwd",
            nested_cwd,
        ])
        .current_dir(&workspace)
        .output()
        .unwrap_or_else(|err| panic!("run {}: {err}", script.display()));

    assert!(
        output.status.success(),
        "prepare fixture checkout should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    let mut checkout_root = None;
    let mut effective_cwd = None;

    for line in stdout.lines() {
        if let Some(value) = line.strip_prefix("CHECKOUT_ROOT=") {
            checkout_root = Some(PathBuf::from(value));
        } else if let Some(value) = line.strip_prefix("EFFECTIVE_CWD=") {
            effective_cwd = Some(PathBuf::from(value));
        }
    }

    let checkout_root = checkout_root.unwrap_or_else(|| panic!("missing CHECKOUT_ROOT: {stdout}"));
    let effective_cwd = effective_cwd.unwrap_or_else(|| panic!("missing EFFECTIVE_CWD: {stdout}"));
    let temp_parent = checkout_root
        .parent()
        .expect("checkout root should have a parent")
        .to_path_buf();

    PreparedCheckout {
        temp_parent,
        checkout_root,
        effective_cwd,
    }
}

fn run_in(dir: &Path, args: &[&str]) -> String {
    let output = binary()
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("run `{}`: {err}", args.join(" ")));

    assert!(
        output.status.success(),
        "`{}` should succeed:\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("stdout is utf-8")
}

#[test]
fn prepare_fixture_checkout_supports_execution_demo_fixture_root_from_nested_cwd() {
    let prepared = prepare_fixture_checkout("tests/fixtures/execution_demo/basic", "work/nested");

    assert!(
        prepared
            .checkout_root
            .join("tests/fixtures/execution_demo/basic/.handbook/charter/CHARTER.md")
            .is_file(),
        "expected execution demo fixture tree to remain available under tests/fixtures/**"
    );

    let generate = run_in(
        &prepared.effective_cwd,
        &[
            "generate",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ],
    );
    assert!(generate.contains("OUTCOME: READY"), "{generate}");
    assert!(
        generate.contains("OBJECT: execution.demo.packet"),
        "{generate}"
    );
    assert!(
        generate.contains("FIXTURE BASIS ROOT: tests/fixtures/execution_demo/basic/.handbook/"),
        "{generate}"
    );

    let inspect = run_in(
        &prepared.effective_cwd,
        &[
            "inspect",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ],
    );
    assert!(inspect.contains("OUTCOME: READY"), "{inspect}");
    assert!(
        inspect.contains("OBJECT: execution.demo.packet"),
        "{inspect}"
    );
    assert!(
        inspect.contains("FIXTURE BASIS ROOT: tests/fixtures/execution_demo/basic/.handbook/"),
        "{inspect}"
    );
}

#[test]
fn prepare_fixture_checkout_preserves_repo_shaped_nested_inspect_flow() {
    let prepared = prepare_fixture_checkout("tests/fixtures/planning_ready_repo", "work/nested");

    assert!(
        prepared
            .checkout_root
            .join(".handbook/charter/CHARTER.md")
            .is_file(),
        "expected repo-shaped fixture contents at checkout root"
    );
    assert!(
        prepared
            .checkout_root
            .join("tests/fixtures/planning_ready_repo/.handbook/charter/CHARTER.md")
            .is_file(),
        "expected tests/fixtures/** ancestry to remain available inside the temp checkout"
    );

    let inspect = run_in(&prepared.effective_cwd, &["inspect"]);
    assert!(inspect.contains("OUTCOME: READY"), "{inspect}");
    assert!(inspect.contains("OBJECT: planning.packet"), "{inspect}");
}
