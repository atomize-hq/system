use clap::CommandFactory;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub(crate) fn discover_managed_repo_root(start: &Path) -> PathBuf {
    if let Some(git_root) = discover_enclosing_git_root(start) {
        return git_root;
    }

    if let Some(managed_root) = discover_nearest_managed_root(start) {
        return managed_root;
    }

    start.to_path_buf()
}

pub(crate) fn read_stdin() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input)
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

fn path_is_dir_or_file(path: &Path) -> bool {
    match std::fs::symlink_metadata(path) {
        Ok(meta) => meta.is_dir() || meta.is_file(),
        Err(_) => false,
    }
}

fn discover_enclosing_git_root(start: &Path) -> Option<PathBuf> {
    for candidate in start.ancestors() {
        if path_is_dir_or_file(&candidate.join(".git")) {
            return Some(candidate.to_path_buf());
        }
    }

    None
}

fn discover_nearest_managed_root(start: &Path) -> Option<PathBuf> {
    for candidate in start.ancestors() {
        if std::fs::symlink_metadata(candidate.join(".handbook")).is_ok() {
            return Some(candidate.to_path_buf());
        }
    }

    None
}
