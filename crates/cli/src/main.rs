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
            Command::Inspect => placeholder_exit("inspect", "reserved proof-surface command"),
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

    let result = match system_compiler::resolve(&repo_root, system_compiler::ResolveRequest::default())
    {
        Ok(result) => result,
        Err(err) => {
            println!("REFUSED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    if let Some(refusal) = result.refusal {
        println!("REFUSED");
        println!("SUMMARY: {}", refusal.summary);
        println!("CATEGORY: {:?}", refusal.category);
        println!("BROKEN: {:?}", refusal.broken_subject);
        println!("NEXT ACTION: {:?}", refusal.next_safe_action);
        return ExitCode::from(1);
    }

    // Until SEAM-5 lands, this command remains honest about rendering being unimplemented, but it
    // can still prove deterministic packet resolution at the contract level.
    println!("RESOLVED");
    println!("PACKET ID: {}", result.selection.packet_id);
    println!("NOTE: packet rendering is not implemented yet (owned by SEAM-5).");
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

    let result = match system_compiler::resolve(&repo_root, system_compiler::ResolveRequest::default())
    {
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

const _: () = {
    let _ = (
        std::mem::size_of::<system_compiler::DecisionLog>(),
        std::mem::size_of::<system_compiler::PacketResult>(),
        std::mem::size_of::<system_compiler::CompilerError>(),
        std::mem::size_of::<system_compiler::Refusal>(),
    );
};
