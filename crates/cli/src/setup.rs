use std::process::ExitCode;

pub(crate) fn run(args: crate::SetupArgs) -> ExitCode {
    crate::setup(args)
}
