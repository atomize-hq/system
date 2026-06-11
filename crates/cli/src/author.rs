use std::process::ExitCode;

pub(crate) fn run(args: crate::AuthorArgs) -> ExitCode {
    match args.command {
        Some(crate::AuthorCommand::Charter(args)) => crate::author_charter_command(args),
        Some(crate::AuthorCommand::ProjectContext(args)) => {
            crate::author_project_context_command(args)
        }
        Some(crate::AuthorCommand::EnvironmentInventory) => {
            crate::author_environment_inventory_command()
        }
        None => crate::shell_shared::print_subcommand_help::<crate::Cli>(&["author"]),
    }
}
