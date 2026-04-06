use clap::{CommandFactory, Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PACKET_PLANNING_ID: &str = "planning.packet";
const PACKET_EXECUTION_DEMO_ID: &str = "execution.demo.packet";
const PACKET_EXECUTION_LIVE_ID: &str = "execution.live.packet";

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => command.run(),
        None => {
            let mut command = Cli::command();
            command.print_help().expect("help output");
            println!();
            ExitCode::SUCCESS
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "system",
    version,
    disable_help_subcommand = true,
    about = "Rust CLI scaffold for the reduced v1 system.",
    long_about = "Rust CLI scaffold for the reduced v1 system. Commands are reserved placeholders only, and the setup-first help ordering is pinned to the reviewed command surface."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Reserved setup entrypoint for the reduced v1 trust flow.
    Setup,
    /// Reserved packet generation command.
    Generate(RequestArgs),
    /// Reserved proof-surface command.
    Inspect(RequestArgs),
    /// Reserved recovery and diagnosis command.
    Doctor,
}

impl Command {
    fn run(self) -> ExitCode {
        match self {
            Command::Setup => placeholder_exit("setup", "reserved setup entrypoint"),
            Command::Generate(args) => generate(args),
            Command::Inspect(args) => inspect(args),
            Command::Doctor => doctor(),
        }
    }
}

#[derive(clap::Args, Debug)]
struct RequestArgs {
    /// Packet identity to generate or inspect.
    #[arg(long, default_value = "planning.packet")]
    packet: String,
    /// Fixture set id (required for `execution.demo.packet`).
    #[arg(long)]
    fixture_set: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PacketId {
    Planning,
    ExecutionDemo,
    ExecutionLive,
}

impl PacketId {
    fn as_str(self) -> &'static str {
        match self {
            PacketId::Planning => PACKET_PLANNING_ID,
            PacketId::ExecutionDemo => PACKET_EXECUTION_DEMO_ID,
            PacketId::ExecutionLive => PACKET_EXECUTION_LIVE_ID,
        }
    }
}

fn parse_packet_id(packet: &str) -> Result<PacketId, String> {
    let packet = packet.trim();
    match packet {
        PACKET_PLANNING_ID => Ok(PacketId::Planning),
        PACKET_EXECUTION_DEMO_ID => Ok(PacketId::ExecutionDemo),
        PACKET_EXECUTION_LIVE_ID => Ok(PacketId::ExecutionLive),
        _ => Err(format!(
            "unsupported --packet {packet:?} (allowed: {PACKET_PLANNING_ID:?}, {PACKET_EXECUTION_DEMO_ID:?}, {PACKET_EXECUTION_LIVE_ID:?})"
        )),
    }
}

fn validate_fixture_set_id(value: &str) -> Result<(), String> {
    let value = value.trim();
    if value.is_empty() {
        return Err("fixture_set_id must not be empty".to_string());
    }
    if value == "." || value == ".." {
        return Err("fixture_set_id must not be '.' or '..'".to_string());
    }
    if value
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
    {
        return Err("fixture_set_id must be ASCII [A-Za-z0-9_-] only".to_string());
    }
    Ok(())
}

fn execution_demo_fixture_set_dir(repo_root: &Path, fixture_set_id: &str) -> PathBuf {
    repo_root
        .join("tests/fixtures/execution_demo")
        .join(fixture_set_id)
}

fn ensure_dir(path: &Path, what: &str) -> Result<(), String> {
    match std::fs::metadata(path) {
        Ok(meta) if meta.is_dir() => Ok(()),
        Ok(_) => Err(format!("{what} is not a directory: {}", path.display())),
        Err(err) => Err(format!("{what} is missing: {} ({err})", path.display())),
    }
}

fn fixture_lineage_for_demo(repo_root: &Path, fixture_set_id: &str) -> Vec<String> {
    let base = execution_demo_fixture_set_dir(repo_root, fixture_set_id).join(".system");

    let project_context = base.join("project_context/PROJECT_CONTEXT.md");

    let mut out = Vec::new();
    out.push(format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/.system/charter/CHARTER.md"
    ));
    if project_context.is_file() {
        out.push(format!(
            "tests/fixtures/execution_demo/{fixture_set_id}/.system/project_context/PROJECT_CONTEXT.md"
        ));
    }
    out.push(format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/.system/feature_spec/FEATURE_SPEC.md"
    ));
    out
}

fn fixture_section_for_demo(repo_root: &Path, fixture_set_id: &str) -> String {
    let mut out = String::new();
    out.push_str("MODE: fixture-backed execution demo\n");
    out.push_str("## FIXTURE DEMO\n");
    out.push_str(&format!("FIXTURE SET: {fixture_set_id}\n"));
    out.push_str(&format!(
        "FIXTURE BASIS ROOT: tests/fixtures/execution_demo/{fixture_set_id}/.system/\n"
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

fn generate(args: RequestArgs) -> ExitCode {
    let repo_root = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };

    let packet_id = match parse_packet_id(&args.packet) {
        Ok(packet_id) => packet_id,
        Err(err) => {
            println!("REFUSED: {err}");
            return ExitCode::from(1);
        }
    };

    let (compiler_root, demo_fixture_set_id) = match packet_id {
        PacketId::Planning | PacketId::ExecutionLive => (repo_root.clone(), None),
        PacketId::ExecutionDemo => {
            let fixture_set_id = match args.fixture_set.as_deref() {
                Some(id) => id.trim(),
                None => {
                    println!("REFUSED: --fixture-set is required when --packet {PACKET_EXECUTION_DEMO_ID}");
                    return ExitCode::from(1);
                }
            };
            if let Err(err) = validate_fixture_set_id(fixture_set_id) {
                println!("REFUSED: invalid --fixture-set {fixture_set_id:?}: {err}");
                return ExitCode::from(1);
            }

            let fixture_set_dir = execution_demo_fixture_set_dir(&repo_root, fixture_set_id);
            if let Err(err) = ensure_dir(&fixture_set_dir, "fixture set directory") {
                println!("REFUSED: {err}");
                return ExitCode::from(1);
            }
            let basis_root = fixture_set_dir.join(".system");
            if let Err(err) = ensure_dir(&basis_root, "fixture basis root") {
                println!("REFUSED: {err}");
                return ExitCode::from(1);
            }
            (fixture_set_dir, Some(fixture_set_id.to_string()))
        }
    };

    let result = match system_compiler::resolve(
        &compiler_root,
        system_compiler::ResolveRequest {
            packet_id: packet_id.as_str(),
            ..system_compiler::ResolveRequest::default()
        },
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("REFUSED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    match system_compiler::build_output_model(&result) {
        Ok(model) => {
            let rendered = system_compiler::render_markdown(&model);
            if let Some(fixture_set_id) = demo_fixture_set_id.as_deref() {
                let section = fixture_section_for_demo(&repo_root, fixture_set_id);
                println!("{}", inject_after_first_three_lines(&rendered, &section));
            } else {
                println!("{rendered}");
            }
        }
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
        }
    }

    ExitCode::from(1)
}

fn placeholder_exit(command: &str, description: &str) -> ExitCode {
    let contract_version = system_compiler::workspace_contract_version();
    println!(
        "system CLI scaffold (contract {contract_version}): `{command}` is a {description}; reduced v1 behavior is not implemented yet."
    );
    ExitCode::from(1)
}

fn doctor() -> ExitCode {
    let repo_root = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };

    let result =
        match system_compiler::resolve(&repo_root, system_compiler::ResolveRequest::default()) {
            Ok(result) => result,
            Err(err) => {
                println!("BLOCKED: resolver error: {err:?}");
                return ExitCode::from(1);
            }
        };

    if result.blockers.is_empty() {
        println!("READY");
        return ExitCode::SUCCESS;
    }

    println!("BLOCKED");
    for blocker in result.blockers {
        println!("CATEGORY: {:?}", blocker.category);
        println!("SUMMARY: {}", blocker.summary);
        println!("SUBJECT: {:?}", blocker.subject);
        println!("NEXT ACTION: {:?}", blocker.next_safe_action);
        println!();
    }

    ExitCode::from(1)
}

fn inspect(args: RequestArgs) -> ExitCode {
    let repo_root = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };

    let packet_id = match parse_packet_id(&args.packet) {
        Ok(packet_id) => packet_id,
        Err(err) => {
            println!("BLOCKED: {err}");
            return ExitCode::from(1);
        }
    };

    let (compiler_root, demo_fixture_set_id) = match packet_id {
        PacketId::Planning | PacketId::ExecutionLive => (repo_root.clone(), None),
        PacketId::ExecutionDemo => {
            let fixture_set_id = match args.fixture_set.as_deref() {
                Some(id) => id.trim(),
                None => {
                    println!(
                        "BLOCKED: --fixture-set is required when --packet {PACKET_EXECUTION_DEMO_ID}"
                    );
                    return ExitCode::from(1);
                }
            };
            if let Err(err) = validate_fixture_set_id(fixture_set_id) {
                println!("BLOCKED: invalid --fixture-set {fixture_set_id:?}: {err}");
                return ExitCode::from(1);
            }

            let fixture_set_dir = execution_demo_fixture_set_dir(&repo_root, fixture_set_id);
            if let Err(err) = ensure_dir(&fixture_set_dir, "fixture set directory") {
                println!("BLOCKED: {err}");
                return ExitCode::from(1);
            }
            let basis_root = fixture_set_dir.join(".system");
            if let Err(err) = ensure_dir(&basis_root, "fixture basis root") {
                println!("BLOCKED: {err}");
                return ExitCode::from(1);
            }
            (fixture_set_dir, Some(fixture_set_id.to_string()))
        }
    };

    let result = match system_compiler::resolve(
        &compiler_root,
        system_compiler::ResolveRequest {
            packet_id: packet_id.as_str(),
            ..system_compiler::ResolveRequest::default()
        },
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("BLOCKED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    let model = match system_compiler::build_output_model(&result) {
        Ok(model) => model,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    let rendered = system_compiler::render_inspect(&model);
    if let Some(fixture_set_id) = demo_fixture_set_id.as_deref() {
        let section = fixture_section_for_demo(&repo_root, fixture_set_id);
        println!("{}", inject_after_first_three_lines(&rendered, &section));
    } else {
        println!("{rendered}");
    }

    if model.packet_status == system_compiler::PacketSelectionStatus::Selected
        && model.refusal.is_none()
        && model.blockers.is_empty()
    {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

const _: () = {
    let _ = (
        std::mem::size_of::<system_compiler::DecisionLog>(),
        std::mem::size_of::<system_compiler::PacketResult>(),
        std::mem::size_of::<system_compiler::CompilerError>(),
        std::mem::size_of::<system_compiler::Refusal>(),
    );
};
