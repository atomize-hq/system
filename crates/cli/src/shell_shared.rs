use clap::CommandFactory;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub(crate) fn discover_managed_repo_root(start: &Path) -> PathBuf {
    if let Some(git_root) = crate::discover_enclosing_git_root(start) {
        return git_root;
    }

    if let Some(managed_root) = crate::discover_nearest_managed_root(start) {
        return managed_root;
    }

    start.to_path_buf()
}

pub(crate) fn print_subcommand_help<C>(path: &[&str]) -> ExitCode
where
    C: CommandFactory,
{
    let mut command = C::command();
    let mut current = &mut command;
    for segment in path {
        current = current
            .find_subcommand_mut(segment)
            .expect("subcommand help path");
    }
    current.print_help().expect("help output");
    println!();
    ExitCode::SUCCESS
}
