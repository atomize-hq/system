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
            Command::Generate => placeholder_exit("generate", "reserved packet generation command"),
            Command::Inspect => placeholder_exit("inspect", "reserved proof-surface command"),
            Command::Doctor => placeholder_exit("doctor", "reserved recovery and diagnosis command"),
        }
    }
}

fn placeholder_exit(command: &str, description: &str) -> ExitCode {
    let contract_version = system_compiler::workspace_contract_version();
    println!(
        "system CLI scaffold (contract {contract_version}): `{command}` is a {description}; reduced v1 behavior is not implemented yet."
    );
    ExitCode::from(1)
}
