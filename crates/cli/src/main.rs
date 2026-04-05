use clap::{CommandFactory, Parser, Subcommand};
use std::process::ExitCode;

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
    Generate,
    /// Reserved proof-surface command.
    Inspect,
    /// Reserved recovery and diagnosis command.
    Doctor,
}

impl Command {
    fn run(self) -> ExitCode {
        match self {
            Command::Setup => placeholder_exit("setup", "reserved setup entrypoint"),
            Command::Generate => generate(),
            Command::Inspect => inspect(),
            Command::Doctor => doctor(),
        }
    }
}

fn generate() -> ExitCode {
    let repo_root = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };

    let result =
        match system_compiler::resolve(&repo_root, system_compiler::ResolveRequest::default()) {
            Ok(result) => result,
            Err(err) => {
                println!("REFUSED: resolver error: {err:?}");
                return ExitCode::from(1);
            }
        };

    match system_compiler::build_output_model(&result) {
        Ok(model) => {
            println!("{}", system_compiler::render_markdown(&model));
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

fn inspect() -> ExitCode {
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

    let model = match system_compiler::build_output_model(&result) {
        Ok(model) => model,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    println!("{}", system_compiler::render_inspect(&model));

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
