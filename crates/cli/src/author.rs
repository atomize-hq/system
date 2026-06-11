use std::process::ExitCode;

pub(crate) fn run(args: crate::AuthorArgs) -> ExitCode {
    crate::author(args)
}
