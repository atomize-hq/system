use std::process::ExitCode;

pub(crate) fn run(args: crate::SetupArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = crate::shell_shared::discover_managed_repo_root(&cwd);

    let (request, routed_from_auto) = match args.command {
        None => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Auto,
                ..handbook_compiler::SetupRequest::default()
            },
            true,
        ),
        Some(crate::SetupCommand::Init) => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Init,
                ..handbook_compiler::SetupRequest::default()
            },
            false,
        ),
        Some(crate::SetupCommand::Refresh(refresh)) => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Refresh,
                rewrite: refresh.rewrite,
                reset_state: refresh.reset_state,
            },
            false,
        ),
    };

    match handbook_compiler::run_setup(&repo_root, &request) {
        Ok(outcome) => {
            println!("{}", crate::render_setup_success(&outcome, routed_from_auto));
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!("{}", crate::render_setup_refusal(&refusal));
            ExitCode::from(1)
        }
    }
}
