use clap::{CommandFactory, Parser, Subcommand};
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PACKET_PLANNING_ID: &str = "planning.packet";
const PACKET_EXECUTION_DEMO_ID: &str = "execution.demo.packet";
const PACKET_EXECUTION_LIVE_ID: &str = "execution.live.packet";
const RELEASE_VERSION: &str = env!("SYSTEM_RELEASE_VERSION");

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
    version = RELEASE_VERSION,
    disable_help_subcommand = true,
    about = "Rust CLI for the reduced v1 system: `setup` initializes or refreshes canonical repo-local `.system/` inputs, `author` is the charter authoring surface, `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations, planning packet generation uses canonical repo-local `.system/` inputs, fixture-backed execution demo flows through `execution.demo.packet`, live execution is explicitly refused, `inspect` is the packet proof surface, and `doctor` is the recovery surface.",
    long_about = "Rust CLI for the reduced v1 system. `setup` initializes or refreshes canonical repo-local `.system/` inputs. `author` is the charter authoring surface. `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations. planning packet generation uses canonical repo-local `.system/` inputs. fixture-backed execution demo flows through `execution.demo.packet`. live execution is explicitly refused. `inspect` is the packet proof surface. `doctor` is the recovery surface."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize or refresh canonical repo-local `.system/` inputs.
    Setup(SetupArgs),
    /// Human-guided and deterministic charter authoring surfaces.
    Author(AuthorArgs),
    /// Pipeline operator surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations.
    Pipeline(PipelineArgs),
    /// Generate a reduced-v1 packet.
    Generate(RequestArgs),
    /// Inspect packet composition and decision evidence.
    Inspect(RequestArgs),
    /// Diagnose blockers and safe next actions.
    Doctor,
}

impl Command {
    fn run(self) -> ExitCode {
        match self {
            Command::Setup(args) => setup(args),
            Command::Author(args) => author(args),
            Command::Pipeline(args) => pipeline(args),
            Command::Generate(args) => generate(args),
            Command::Inspect(args) => inspect(args),
            Command::Doctor => doctor(),
        }
    }
}

#[derive(clap::Args, Debug)]
struct SetupArgs {
    #[command(subcommand)]
    command: Option<SetupCommand>,
}

#[derive(Subcommand, Debug)]
enum SetupCommand {
    /// Create canonical `.system/` scaffold and starter files for first-run setup.
    Init,
    /// Preserve canonical files by default and optionally rewrite starter files or reset `.system/state/**`.
    Refresh(SetupRefreshArgs),
}

#[derive(clap::Args, Debug)]
struct SetupRefreshArgs {
    /// Rewrite setup-owned starter files in place.
    #[arg(long)]
    rewrite: bool,
    /// Reset only `.system/state/**`.
    #[arg(long = "reset-state")]
    reset_state: bool,
}

#[derive(clap::Args, Debug)]
struct AuthorArgs {
    #[command(subcommand)]
    command: Option<AuthorCommand>,
}

#[derive(Subcommand, Debug)]
enum AuthorCommand {
    /// Author canonical `.system/charter/CHARTER.md`.
    Charter(AuthorCharterArgs),
}

#[derive(clap::Args, Debug)]
struct AuthorCharterArgs {
    /// Read normalized structured inputs from a YAML file or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: Option<String>,
}

#[derive(clap::Args, Debug)]
struct PipelineArgs {
    #[command(subcommand)]
    command: PipelineCommand,
}

#[derive(Subcommand, Debug)]
enum PipelineCommand {
    /// List available pipelines.
    List,
    /// Show one canonical pipeline or stage declaration.
    Show(PipelineShowArgs),
    /// Resolve one pipeline route from persisted route state.
    Resolve(PipelineSelectorArgs),
    /// Compile one supported stage payload from persisted route basis.
    Compile(PipelineCompileArgs),
    /// Capture one supported stage output and materialize declared artifact and repo-mirror files for `pipeline.foundation_inputs` stages `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, and `stage.10_feature_spec`.
    Capture(PipelineCaptureArgs),
    /// Emit one supported downstream handoff bundle from persisted stage and provenance surfaces.
    Handoff(PipelineHandoffArgs),
    /// Route-state operations.
    State(PipelineStateArgs),
}

#[derive(clap::Args, Debug)]
struct PipelineHandoffArgs {
    #[command(subcommand)]
    command: PipelineHandoffCommand,
}

#[derive(Subcommand, Debug)]
enum PipelineHandoffCommand {
    /// Emit one bounded handoff bundle for `pipeline.foundation_inputs` -> `feature-slice-decomposer`.
    Emit(PipelineHandoffEmitArgs),
}

#[derive(clap::Args, Debug)]
struct PipelineHandoffEmitArgs {
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: String,
    /// Supported downstream consumer id.
    #[arg(long)]
    consumer: String,
}

#[derive(clap::Args, Debug)]
struct PipelineStateArgs {
    #[command(subcommand)]
    command: PipelineStateCommand,
}

#[derive(Subcommand, Debug)]
enum PipelineStateCommand {
    /// Set one supported route-state field.
    Set(PipelineStateSetArgs),
}

#[derive(clap::Args, Debug)]
struct PipelineShowArgs {
    /// Canonical id or unambiguous shorthand for a pipeline or stage.
    #[arg(long)]
    id: String,
}

#[derive(clap::Args, Debug)]
struct PipelineSelectorArgs {
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: String,
}

#[derive(clap::Args, Debug)]
struct PipelineCompileArgs {
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: String,
    /// Canonical id or unambiguous shorthand for a stage within the selected pipeline.
    #[arg(long)]
    stage: String,
    /// Render compile proof instead of the stage payload.
    #[arg(long)]
    explain: bool,
}

#[derive(clap::Args, Debug)]
struct PipelineCaptureArgs {
    #[command(subcommand)]
    command: Option<PipelineCaptureCommand>,
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: Option<String>,
    /// Canonical id or unambiguous shorthand for a stage within the selected pipeline.
    #[arg(long)]
    stage: Option<String>,
    /// Validate and cache the capture plan without writing declared outputs.
    #[arg(long)]
    preview: bool,
}

#[derive(Subcommand, Debug)]
enum PipelineCaptureCommand {
    /// Apply one cached preview by capture id.
    Apply(PipelineCaptureApplyArgs),
}

#[derive(clap::Args, Debug)]
struct PipelineCaptureApplyArgs {
    /// Deterministic capture id returned by `pipeline capture --preview`.
    #[arg(long)]
    capture_id: String,
}

#[derive(clap::Args, Debug)]
struct PipelineStateSetArgs {
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: String,
    /// Route-state routing assignment in name=value form.
    #[arg(long, conflicts_with = "field", required_unless_present = "field")]
    var: Option<String>,
    /// Route-state field assignment in field.path=value form.
    #[arg(long, conflicts_with = "var", required_unless_present = "var")]
    field: Option<String>,
    /// Expected route-state revision. Defaults to the currently loaded revision.
    #[arg(long)]
    expected_revision: Option<u64>,
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
        if std::fs::symlink_metadata(candidate.join(".system")).is_ok() {
            return Some(candidate.to_path_buf());
        }
    }

    None
}

fn discover_managed_repo_root(start: &Path) -> PathBuf {
    if let Some(git_root) = discover_enclosing_git_root(start) {
        return git_root;
    }

    if let Some(managed_root) = discover_nearest_managed_root(start) {
        return managed_root;
    }

    start.to_path_buf()
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
    let cwd = match std::env::current_dir() {
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

    let repo_root = discover_managed_repo_root(&cwd);

    let compiler_root = match packet_id {
        PacketId::Planning | PacketId::ExecutionLive => repo_root.clone(),
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
            fixture_set_dir
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

    let model = match system_compiler::build_output_model(&result) {
        Ok(model) => model,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    let ready = model.packet_status == system_compiler::PacketSelectionStatus::Selected
        && model.refusal.is_none()
        && model.blockers.is_empty();

    println!("{}", system_compiler::render_markdown(&model));
    if ready {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn author(args: AuthorArgs) -> ExitCode {
    match args.command {
        Some(AuthorCommand::Charter(args)) => author_charter_command(args),
        None => print_subcommand_help(&["author"]),
    }
}

#[derive(Clone)]
struct EnvVarCharterSynthesizer {
    response: Result<String, String>,
}

impl system_compiler::CharterSynthesizer for EnvVarCharterSynthesizer {
    fn synthesize(
        &self,
        _repo_root: &Path,
        _request: system_compiler::CharterSynthesisRequest,
    ) -> Result<String, system_compiler::CharterSynthesisError> {
        match &self.response {
            Ok(markdown) => Ok(markdown.clone()),
            Err(message) => Err(system_compiler::CharterSynthesisError {
                message: message.clone(),
            }),
        }
    }
}

fn author_charter_command(args: AuthorCharterArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!(
                "{}",
                render_author_custom_refusal(
                    "REFUSED",
                    "WorkingDirectoryUnavailable",
                    &format!("failed to determine repo root: {err}"),
                    "current working directory",
                    "repair the current working directory and retry `system author charter`",
                )
            );
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let (input, input_mode, input_source) = match args.from_inputs.as_deref() {
        Some(path_or_dash) => {
            let yaml = match read_author_inputs_source(path_or_dash) {
                Ok(yaml) => yaml,
                Err(rendered) => {
                    println!("{rendered}");
                    return ExitCode::from(1);
                }
            };
            let input = match system_compiler::parse_charter_structured_input_yaml(&yaml) {
                Ok(input) => input,
                Err(refusal) => {
                    println!("{}", render_author_charter_refusal(&refusal));
                    return ExitCode::from(1);
                }
            };
            let input_mode = if path_or_dash == "-" {
                "structured_inputs_stdin"
            } else {
                "structured_inputs_file"
            };
            (input, input_mode, path_or_dash.to_string())
        }
        None => {
            if !interactive_authoring_is_allowed() {
                println!(
                    "{}",
                    render_author_custom_refusal(
                        "REFUSED",
                        "NonInteractiveRefusal",
                        "`system author charter` is a TTY-only guided interview",
                        "interactive terminal",
                        "run `system author charter --from-inputs <path|->`",
                    )
                );
                return ExitCode::from(1);
            }
            let input = match collect_guided_charter_input() {
                Ok(input) => input,
                Err(rendered) => {
                    println!("{rendered}");
                    return ExitCode::from(1);
                }
            };
            (
                input,
                "guided_interview",
                "interactive terminal".to_string(),
            )
        }
    };

    match run_author_charter_with_cli_synthesizer(&repo_root, &input) {
        Ok(result) => {
            println!(
                "{}",
                render_author_charter_success(&result, input_mode, &input_source)
            );
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!("{}", render_author_charter_refusal(&refusal));
            ExitCode::from(1)
        }
    }
}

fn interactive_authoring_is_allowed() -> bool {
    if let Ok(value) = std::env::var("SYSTEM_AUTHOR_FORCE_TTY") {
        if matches!(value.trim(), "1" | "true" | "TRUE" | "yes" | "YES") {
            return true;
        }
    }

    io::stdin().is_terminal() && io::stdout().is_terminal()
}

fn run_author_charter_with_cli_synthesizer(
    repo_root: &Path,
    input: &system_compiler::CharterStructuredInput,
) -> Result<system_compiler::AuthorCharterResult, system_compiler::AuthorCharterRefusal> {
    if let Ok(message) = std::env::var("SYSTEM_AUTHOR_TEST_SYNTHESIS_ERROR") {
        let synthesizer = EnvVarCharterSynthesizer {
            response: Err(message),
        };
        return system_compiler::author_charter_with_synthesizer(repo_root, input, &synthesizer);
    }

    if let Ok(markdown) = std::env::var("SYSTEM_AUTHOR_TEST_SYNTHESIS_OUTPUT") {
        let synthesizer = EnvVarCharterSynthesizer {
            response: Ok(markdown),
        };
        return system_compiler::author_charter_with_synthesizer(repo_root, input, &synthesizer);
    }

    system_compiler::author_charter(repo_root, input)
}

fn render_author_charter_success(
    result: &system_compiler::AuthorCharterResult,
    input_mode: &str,
    input_source: &str,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: AUTHORED\n");
    out.push_str("OBJECT: author charter\n");
    out.push_str("NEXT SAFE ACTION: run `system doctor`\n");
    out.push_str("## CANONICAL ARTIFACT\n");
    out.push_str(&format!("PATH: {}\n", result.canonical_repo_relative_path));
    out.push_str(&format!("BYTES WRITTEN: {}\n", result.bytes_written));
    out.push_str("## INPUT MODE\n");
    out.push_str(&format!("MODE: {input_mode}\n"));
    out.push_str(&format!("SOURCE: {input_source}\n"));
    out.trim_end().to_string()
}

fn render_author_charter_refusal(refusal: &system_compiler::AuthorCharterRefusal) -> String {
    render_author_custom_refusal(
        author_refusal_outcome_name(refusal.kind),
        author_refusal_kind_name(refusal.kind),
        refusal.summary.trim(),
        refusal.broken_subject.trim(),
        refusal.next_safe_action.trim(),
    )
}

fn render_author_custom_refusal(
    outcome: &str,
    category: &str,
    summary: &str,
    broken_subject: &str,
    next_safe_action: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("OUTCOME: {outcome}\n"));
    out.push_str("OBJECT: author charter\n");
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.push_str("## REFUSAL\n");
    out.push_str(&format!("CATEGORY: {category}\n"));
    out.push_str(&format!("SUMMARY: {summary}\n"));
    out.push_str(&format!("BROKEN SUBJECT: {broken_subject}\n"));
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.trim_end().to_string()
}

fn author_refusal_outcome_name(kind: system_compiler::AuthorCharterRefusalKind) -> &'static str {
    match kind {
        system_compiler::AuthorCharterRefusalKind::MissingSystemRoot
        | system_compiler::AuthorCharterRefusalKind::InvalidSystemRoot
        | system_compiler::AuthorCharterRefusalKind::MutationRefused
        | system_compiler::AuthorCharterRefusalKind::SynthesisFailed => "BLOCKED",
        system_compiler::AuthorCharterRefusalKind::MalformedStructuredInput
        | system_compiler::AuthorCharterRefusalKind::IncompleteStructuredInput
        | system_compiler::AuthorCharterRefusalKind::ExistingCanonicalTruth => "REFUSED",
    }
}

fn author_refusal_kind_name(kind: system_compiler::AuthorCharterRefusalKind) -> &'static str {
    match kind {
        system_compiler::AuthorCharterRefusalKind::MissingSystemRoot => "MissingSystemRoot",
        system_compiler::AuthorCharterRefusalKind::InvalidSystemRoot => "InvalidSystemRoot",
        system_compiler::AuthorCharterRefusalKind::MalformedStructuredInput => {
            "MalformedStructuredInput"
        }
        system_compiler::AuthorCharterRefusalKind::IncompleteStructuredInput => {
            "IncompleteStructuredInput"
        }
        system_compiler::AuthorCharterRefusalKind::ExistingCanonicalTruth => {
            "ExistingCanonicalTruth"
        }
        system_compiler::AuthorCharterRefusalKind::MutationRefused => "MutationRefused",
        system_compiler::AuthorCharterRefusalKind::SynthesisFailed => "SynthesisFailed",
    }
}

fn read_author_inputs_source(path_or_dash: &str) -> Result<String, String> {
    if path_or_dash == "-" {
        return read_stdin().map_err(|err| {
            render_author_custom_refusal(
                "REFUSED",
                "InputReadFailure",
                &format!("failed to read structured inputs from stdin: {err}"),
                "structured charter input source",
                "repair stdin and retry `system author charter --from-inputs -`",
            )
        });
    }

    fs::read_to_string(path_or_dash).map_err(|err| {
        render_author_custom_refusal(
            "REFUSED",
            "InputReadFailure",
            &format!(
                "failed to read structured inputs from `{path_or_dash}`: {err}"
            ),
            "structured charter input source",
            "repair the structured input file and retry `system author charter --from-inputs <path|->`",
        )
    })
}

fn collect_guided_charter_input() -> Result<system_compiler::CharterStructuredInput, String> {
    println!("Guided charter interview");
    println!("Answer with the documented value form. Comma-separated prompts accept `a, b, c`.");

    let project_name = prompt_required_concrete(
        "Project name",
        "Project name needs a concrete system name, not a placeholder",
        "project name",
    )?;
    let classification = prompt_choice(
        "Project classification [greenfield|brownfield|integration|modernization|hardening]",
        parse_project_classification,
    )?;
    let team_size = prompt_u32("Team size (> 0)")?;
    let users = prompt_choice("Audience [internal|external|mixed]", parse_audience)?;
    let expected_lifetime = prompt_choice(
        "Expected lifetime [days|weeks|months|years]",
        parse_expected_lifetime,
    )?;
    let surfaces = prompt_csv_choice(
        "Surfaces [web_app, api, cli, lib, infra, ml]",
        parse_surface,
    )?;
    let runtime_environments = prompt_csv_choice(
        "Runtime environments [browser, server, cloud, on_prem, edge]",
        parse_runtime_environment,
    )?;
    let deadline = prompt_optional("Deadline or delivery window")?;
    let budget = prompt_optional("Budget notes")?;
    let experience_notes = prompt_required_concrete(
        "Experience notes",
        "Experience notes need a concrete summary of team experience or delivery constraints",
        "experience notes",
    )?;
    let must_use_tech = prompt_csv_optional("Must-use tech (comma-separated, optional)")?;
    let in_production_today = prompt_bool("In production today? [yes|no]")?;
    let prod_users_or_data = prompt_optional("Production users or data notes")?;
    let external_contracts_to_preserve =
        prompt_csv_optional("External contracts to preserve (comma-separated, optional)")?;
    let uptime_expectations = prompt_optional("Uptime expectations")?;
    let baseline_level = prompt_u8_in_range("Baseline rubric level [1-5]", 1, 5)?;
    let baseline_rationale = prompt_csv_non_empty_concrete(
        "Baseline rationale (comma-separated, at least one)",
        "Baseline rationale needs concrete reasons, not placeholders",
        "baseline rationale",
    )?;
    let backward_compatibility = prompt_choice(
        "Backward compatibility [required|not_required|boundary_only]",
        parse_backward_compatibility,
    )?;
    let migration_planning = prompt_choice(
        "Migration planning [required|not_required]",
        parse_requiredness,
    )?;
    let rollout_controls = prompt_choice(
        "Rollout controls [none|lightweight|required]",
        parse_rollout_controls,
    )?;
    let deprecation_policy = prompt_choice(
        "Deprecation policy [required|not_required_yet]",
        parse_deprecation_policy,
    )?;
    let observability_threshold = prompt_choice(
        "Observability threshold [minimal|standard|high|regulated]",
        parse_observability_threshold,
    )?;
    let primary_domain_name = prompt_optional("Primary domain name (optional)")?;
    let domains = if primary_domain_name.trim().is_empty() {
        Vec::new()
    } else {
        let blast_radius = prompt_required_concrete(
            "Primary domain blast radius",
            "Primary domain blast radius needs a concrete impact or failure description",
            "primary domain blast radius",
        )?;
        let touches = prompt_csv_optional("Primary domain touches (comma-separated, optional)")?;
        let constraints =
            prompt_csv_optional("Primary domain constraints (comma-separated, optional)")?;
        vec![system_compiler::CharterDomainInput {
            name: primary_domain_name,
            blast_radius,
            touches,
            constraints,
        }]
    };
    let approvers = prompt_csv_non_empty_concrete(
        "Exception approvers (comma-separated, at least one)",
        "Exception approvers need concrete owners or roles",
        "exception approvers",
    )?;
    let record_location =
        prompt_with_default("Exception record location", "CHARTER.md#exceptions")?;
    let minimum_fields_input = prompt_optional(
        "Exception minimum fields (comma-separated; press enter for standard fields)",
    )?;
    let minimum_fields = if minimum_fields_input.trim().is_empty() {
        default_exception_minimum_fields()
    } else {
        split_csv_required(&minimum_fields_input)?
    };
    let debt_tracking_system = prompt_required_concrete(
        "Debt tracking system",
        "Debt tracking system needs a concrete tracker or repository location",
        "debt tracking system",
    )?;
    let debt_tracking_labels =
        prompt_csv_optional("Debt tracking labels (comma-separated, optional)")?;
    let debt_tracking_review_cadence = prompt_required_concrete(
        "Debt tracking review cadence",
        "Debt tracking review cadence needs a concrete cadence such as weekly or monthly",
        "debt tracking review cadence",
    )?;
    let decision_records_enabled = prompt_bool("Decision records enabled? [yes|no]")?;
    let (decision_records_path, decision_records_format) = if decision_records_enabled {
        (
            prompt_required_concrete(
                "Decision records path",
                "Decision records path needs a concrete folder path",
                "decision records path",
            )?,
            prompt_required_concrete(
                "Decision records format",
                "Decision records format needs a concrete format such as md",
                "decision records format",
            )?,
        )
    } else {
        (String::new(), String::new())
    };

    Ok(system_compiler::CharterStructuredInput {
        schema_version: "0.1.0".to_string(),
        project: system_compiler::CharterProjectInput {
            name: project_name.clone(),
            classification,
            team_size,
            users,
            expected_lifetime,
            surfaces,
            runtime_environments,
            constraints: system_compiler::CharterProjectConstraintsInput {
                deadline,
                budget,
                experience_notes: experience_notes.clone(),
                must_use_tech,
            },
            operational_reality: system_compiler::CharterOperationalRealityInput {
                in_production_today,
                prod_users_or_data,
                external_contracts_to_preserve,
                uptime_expectations,
            },
            default_implications: system_compiler::CharterDefaultImplicationsInput {
                backward_compatibility,
                migration_planning,
                rollout_controls,
                deprecation_policy,
                observability_threshold,
            },
        },
        posture: system_compiler::CharterPostureInput {
            rubric_scale: "1-5".to_string(),
            baseline_level,
            baseline_rationale,
        },
        domains,
        dimensions: default_dimension_inputs(baseline_level, &project_name, in_production_today),
        exceptions: system_compiler::CharterExceptionsInput {
            approvers,
            record_location,
            minimum_fields,
        },
        debt_tracking: system_compiler::CharterDebtTrackingInput {
            system: debt_tracking_system,
            labels: debt_tracking_labels,
            review_cadence: debt_tracking_review_cadence,
        },
        decision_records: system_compiler::CharterDecisionRecordsInput {
            enabled: decision_records_enabled,
            path: decision_records_path,
            format: decision_records_format,
        },
    })
}

fn prompt_line(prompt: &str) -> Result<String, String> {
    print!("{prompt}: ");
    io::stdout().flush().map_err(|err| {
        render_author_custom_refusal(
            "REFUSED",
            "PromptWriteFailure",
            &format!("failed to render guided prompt: {err}"),
            "interactive terminal",
            "repair the interactive terminal and retry `system author charter`",
        )
    })?;

    let mut value = String::new();
    let bytes_read = io::stdin().read_line(&mut value).map_err(|err| {
        render_author_custom_refusal(
            "REFUSED",
            "PromptReadFailure",
            &format!("failed to read guided answer: {err}"),
            "interactive terminal",
            "repair the interactive terminal and retry `system author charter`",
        )
    })?;

    if bytes_read == 0 {
        return Err(render_author_custom_refusal(
            "REFUSED",
            "InterviewIncomplete",
            "guided charter interview ended before all required answers were collected",
            "structured charter input",
            "restart `system author charter` or use `system author charter --from-inputs <path|->`",
        ));
    }

    Ok(value.trim().to_string())
}

fn prompt_required_concrete(
    prompt: &str,
    follow_up_prompt: &str,
    field_name: &str,
) -> Result<String, String> {
    let value = prompt_line(prompt)?;
    if let Some(normalized) = normalize_required_free_text(&value) {
        return Ok(normalized);
    }

    let follow_up = prompt_line(follow_up_prompt)?;
    if let Some(normalized) = normalize_required_free_text(&follow_up) {
        return Ok(normalized);
    }

    Err(render_interview_incomplete_refusal(&format!(
        "guided charter interview could not normalize a concrete answer for {field_name}"
    )))
}

fn prompt_optional(prompt: &str) -> Result<String, String> {
    prompt_line(prompt).map(|value| normalize_free_text_answer(&value))
}

fn prompt_with_default(prompt: &str, default_value: &str) -> Result<String, String> {
    let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
    if value.trim().is_empty() {
        Ok(default_value.to_string())
    } else {
        Ok(normalize_free_text_answer(&value))
    }
}

fn prompt_bool(prompt: &str) -> Result<bool, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" | "true" => return Ok(true),
            "n" | "no" | "false" => return Ok(false),
            _ => println!("Expected yes/no."),
        }
    }
}

fn prompt_u32(prompt: &str) -> Result<u32, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().parse::<u32>() {
            Ok(parsed) if parsed > 0 => return Ok(parsed),
            _ => println!("Expected an integer greater than 0."),
        }
    }
}

fn prompt_u8_in_range(prompt: &str, min: u8, max: u8) -> Result<u8, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().parse::<u8>() {
            Ok(parsed) if (min..=max).contains(&parsed) => return Ok(parsed),
            _ => println!("Expected an integer in the allowed range."),
        }
    }
}

fn prompt_choice<T>(prompt: &str, parser: fn(&str) -> Result<T, String>) -> Result<T, String> {
    loop {
        let value = prompt_line(prompt)?;
        match parser(&value) {
            Ok(parsed) => return Ok(parsed),
            Err(err) => println!("{err}"),
        }
    }
}

fn prompt_csv_choice<T>(
    prompt: &str,
    parser: fn(&str) -> Result<T, String>,
) -> Result<Vec<T>, String> {
    loop {
        let value = prompt_line(prompt)?;
        match split_csv_required(&value) {
            Ok(items) => {
                let mut parsed = Vec::new();
                let mut error = None;
                for item in items {
                    match parser(&item) {
                        Ok(value) => parsed.push(value),
                        Err(err) => {
                            error = Some(err);
                            break;
                        }
                    }
                }
                if let Some(err) = error {
                    println!("{err}");
                    continue;
                }
                return Ok(parsed);
            }
            Err(err) => println!("{err}"),
        }
    }
}

fn prompt_csv_optional(prompt: &str) -> Result<Vec<String>, String> {
    let value = prompt_line(prompt)?;
    if value.trim().is_empty() {
        Ok(Vec::new())
    } else {
        split_csv_required(&value)
    }
}

fn prompt_csv_non_empty_concrete(
    prompt: &str,
    follow_up_prompt: &str,
    field_name: &str,
) -> Result<Vec<String>, String> {
    let value = prompt_line(prompt)?;
    if let Some(items) = normalize_required_csv(&value) {
        return Ok(items);
    }

    let follow_up = prompt_line(follow_up_prompt)?;
    if let Some(items) = normalize_required_csv(&follow_up) {
        return Ok(items);
    }

    Err(render_interview_incomplete_refusal(&format!(
        "guided charter interview could not normalize a concrete answer for {field_name}"
    )))
}

fn split_csv_required(value: &str) -> Result<Vec<String>, String> {
    let items = value
        .split(',')
        .map(normalize_free_text_answer)
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>();
    if items.is_empty() {
        Err("Provide at least one comma-separated value.".to_string())
    } else {
        Ok(items)
    }
}

fn normalize_required_free_text(value: &str) -> Option<String> {
    let normalized = normalize_free_text_answer(value);
    if normalized.is_empty() || is_unusably_vague_text(&normalized) {
        None
    } else {
        Some(normalized)
    }
}

fn normalize_required_csv(value: &str) -> Option<Vec<String>> {
    let items = split_csv_required(value).ok()?;
    if items.iter().any(|item| is_unusably_vague_text(item)) {
        None
    } else {
        Some(items)
    }
}

fn normalize_free_text_answer(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_unusably_vague_text(value: &str) -> bool {
    let normalized = normalize_free_text_answer(value);
    if normalized.is_empty() {
        return true;
    }

    let lower = normalized.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "idk"
            | "i don't know"
            | "dont know"
            | "unknown"
            | "n/a"
            | "na"
            | "tbd"
            | "todo"
            | "unsure"
            | "not sure"
            | "good quality"
            | "good"
            | "quality"
            | "standard"
            | "normal"
            | "stuff"
            | "things"
            | "misc"
            | "various"
            | "whatever"
    )
}

fn render_interview_incomplete_refusal(summary: &str) -> String {
    render_author_custom_refusal(
        "REFUSED",
        "InterviewIncomplete",
        summary,
        "structured charter input",
        "restart `system author charter` or use `system author charter --from-inputs <path|->`",
    )
}

fn parse_project_classification(
    value: &str,
) -> Result<system_compiler::CharterProjectClassification, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "greenfield" => Ok(system_compiler::CharterProjectClassification::Greenfield),
        "brownfield" => Ok(system_compiler::CharterProjectClassification::Brownfield),
        "integration" => Ok(system_compiler::CharterProjectClassification::Integration),
        "modernization" => Ok(system_compiler::CharterProjectClassification::Modernization),
        "hardening" => Ok(system_compiler::CharterProjectClassification::Hardening),
        _ => Err(
            "Expected one of greenfield, brownfield, integration, modernization, or hardening."
                .to_string(),
        ),
    }
}

fn parse_audience(value: &str) -> Result<system_compiler::CharterAudience, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "internal" => Ok(system_compiler::CharterAudience::Internal),
        "external" => Ok(system_compiler::CharterAudience::External),
        "mixed" => Ok(system_compiler::CharterAudience::Mixed),
        _ => Err("Expected one of internal, external, or mixed.".to_string()),
    }
}

fn parse_expected_lifetime(
    value: &str,
) -> Result<system_compiler::CharterExpectedLifetime, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "days" => Ok(system_compiler::CharterExpectedLifetime::Days),
        "weeks" => Ok(system_compiler::CharterExpectedLifetime::Weeks),
        "months" => Ok(system_compiler::CharterExpectedLifetime::Months),
        "years" => Ok(system_compiler::CharterExpectedLifetime::Years),
        _ => Err("Expected one of days, weeks, months, or years.".to_string()),
    }
}

fn parse_surface(value: &str) -> Result<system_compiler::CharterSurface, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "web_app" => Ok(system_compiler::CharterSurface::WebApp),
        "api" => Ok(system_compiler::CharterSurface::Api),
        "cli" => Ok(system_compiler::CharterSurface::Cli),
        "lib" => Ok(system_compiler::CharterSurface::Lib),
        "infra" => Ok(system_compiler::CharterSurface::Infra),
        "ml" => Ok(system_compiler::CharterSurface::Ml),
        _ => Err("Expected one of web_app, api, cli, lib, infra, or ml.".to_string()),
    }
}

fn parse_runtime_environment(
    value: &str,
) -> Result<system_compiler::CharterRuntimeEnvironment, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "browser" => Ok(system_compiler::CharterRuntimeEnvironment::Browser),
        "server" => Ok(system_compiler::CharterRuntimeEnvironment::Server),
        "cloud" => Ok(system_compiler::CharterRuntimeEnvironment::Cloud),
        "on_prem" => Ok(system_compiler::CharterRuntimeEnvironment::OnPrem),
        "edge" => Ok(system_compiler::CharterRuntimeEnvironment::Edge),
        _ => Err("Expected one of browser, server, cloud, on_prem, or edge.".to_string()),
    }
}

fn parse_backward_compatibility(
    value: &str,
) -> Result<system_compiler::CharterBackwardCompatibility, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "required" => Ok(system_compiler::CharterBackwardCompatibility::Required),
        "not_required" => Ok(system_compiler::CharterBackwardCompatibility::NotRequired),
        "boundary_only" => Ok(system_compiler::CharterBackwardCompatibility::BoundaryOnly),
        _ => Err("Expected one of required, not_required, or boundary_only.".to_string()),
    }
}

fn parse_requiredness(value: &str) -> Result<system_compiler::CharterRequiredness, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "required" => Ok(system_compiler::CharterRequiredness::Required),
        "not_required" => Ok(system_compiler::CharterRequiredness::NotRequired),
        _ => Err("Expected one of required or not_required.".to_string()),
    }
}

fn parse_rollout_controls(value: &str) -> Result<system_compiler::CharterRolloutControls, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "none" => Ok(system_compiler::CharterRolloutControls::None),
        "lightweight" => Ok(system_compiler::CharterRolloutControls::Lightweight),
        "required" => Ok(system_compiler::CharterRolloutControls::Required),
        _ => Err("Expected one of none, lightweight, or required.".to_string()),
    }
}

fn parse_deprecation_policy(
    value: &str,
) -> Result<system_compiler::CharterDeprecationPolicy, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "required" => Ok(system_compiler::CharterDeprecationPolicy::Required),
        "not_required_yet" => Ok(system_compiler::CharterDeprecationPolicy::NotRequiredYet),
        _ => Err("Expected one of required or not_required_yet.".to_string()),
    }
}

fn parse_observability_threshold(
    value: &str,
) -> Result<system_compiler::CharterObservabilityThreshold, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "minimal" => Ok(system_compiler::CharterObservabilityThreshold::Minimal),
        "standard" => Ok(system_compiler::CharterObservabilityThreshold::Standard),
        "high" => Ok(system_compiler::CharterObservabilityThreshold::High),
        "regulated" => Ok(system_compiler::CharterObservabilityThreshold::Regulated),
        _ => Err("Expected one of minimal, standard, high, or regulated.".to_string()),
    }
}

fn default_exception_minimum_fields() -> Vec<String> {
    [
        "what",
        "why",
        "scope",
        "risk",
        "owner",
        "expiry_or_revisit_date",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn default_dimension_inputs(
    baseline_level: u8,
    project_name: &str,
    in_production_today: bool,
) -> Vec<system_compiler::CharterDimensionInput> {
    all_dimension_names()
        .iter()
        .copied()
        .map(|name| {
            default_dimension_input(name, baseline_level, project_name, in_production_today)
        })
        .collect()
}

fn all_dimension_names() -> [system_compiler::CharterDimensionName; 9] {
    [
        system_compiler::CharterDimensionName::SpeedVsQuality,
        system_compiler::CharterDimensionName::TypeSafetyStaticAnalysis,
        system_compiler::CharterDimensionName::TestingRigor,
        system_compiler::CharterDimensionName::ScalabilityPerformance,
        system_compiler::CharterDimensionName::ReliabilityOperability,
        system_compiler::CharterDimensionName::SecurityPrivacy,
        system_compiler::CharterDimensionName::Observability,
        system_compiler::CharterDimensionName::DxToolingAutomation,
        system_compiler::CharterDimensionName::UxPolishApiUsability,
    ]
}

fn default_dimension_input(
    name: system_compiler::CharterDimensionName,
    baseline_level: u8,
    project_name: &str,
    in_production_today: bool,
) -> system_compiler::CharterDimensionInput {
    let dimension_label = dimension_label(name);
    let production_trigger = if in_production_today {
        "changes touching live users, data, or uptime"
    } else {
        "changes that create irreversible migration or trust-boundary cost"
    };

    system_compiler::CharterDimensionInput {
        name,
        level: Some(baseline_level),
        default_stance: format!(
            "{project_name} defaults to level {baseline_level} on {dimension_label}; raise the bar whenever blast radius, trust boundaries, or recovery cost increases."
        ),
        raise_the_bar_triggers: vec![
            production_trigger.to_string(),
            "new external interfaces or contracts".to_string(),
        ],
        allowed_shortcuts: vec![
            "time-boxed exploration before merge".to_string(),
            "fixture-backed or local-only iteration with explicit follow-up".to_string(),
        ],
        red_lines: vec![
            format!("do not waive {dimension_label} expectations on shipped work"),
            "do not hide known risk without recording an exception".to_string(),
        ],
        domain_overrides: Vec::new(),
    }
}

fn dimension_label(name: system_compiler::CharterDimensionName) -> &'static str {
    match name {
        system_compiler::CharterDimensionName::SpeedVsQuality => "speed vs quality",
        system_compiler::CharterDimensionName::TypeSafetyStaticAnalysis => {
            "type safety and static analysis"
        }
        system_compiler::CharterDimensionName::TestingRigor => "testing rigor",
        system_compiler::CharterDimensionName::ScalabilityPerformance => {
            "scalability and performance"
        }
        system_compiler::CharterDimensionName::ReliabilityOperability => {
            "reliability and operability"
        }
        system_compiler::CharterDimensionName::SecurityPrivacy => "security and privacy",
        system_compiler::CharterDimensionName::Observability => "observability",
        system_compiler::CharterDimensionName::DxToolingAutomation => {
            "developer tooling and automation"
        }
        system_compiler::CharterDimensionName::UxPolishApiUsability => {
            "ux polish and api usability"
        }
    }
}

fn print_subcommand_help(path: &[&str]) -> ExitCode {
    let mut command = Cli::command();
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

fn setup(args: SetupArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let (request, routed_from_auto) = match args.command {
        None => (
            system_compiler::SetupRequest {
                mode: system_compiler::SetupMode::Auto,
                ..system_compiler::SetupRequest::default()
            },
            true,
        ),
        Some(SetupCommand::Init) => (
            system_compiler::SetupRequest {
                mode: system_compiler::SetupMode::Init,
                ..system_compiler::SetupRequest::default()
            },
            false,
        ),
        Some(SetupCommand::Refresh(refresh)) => (
            system_compiler::SetupRequest {
                mode: system_compiler::SetupMode::Refresh,
                rewrite: refresh.rewrite,
                reset_state: refresh.reset_state,
            },
            false,
        ),
    };

    match system_compiler::run_setup(&repo_root, &request) {
        Ok(outcome) => {
            println!("{}", render_setup_success(&outcome, routed_from_auto));
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!("{}", render_setup_refusal(&refusal));
            ExitCode::from(1)
        }
    }
}

fn render_setup_success(outcome: &system_compiler::SetupOutcome, routed_from_auto: bool) -> String {
    let mut out = String::new();
    let starter_actions = outcome
        .plan
        .actions
        .iter()
        .filter(|action| action.label != system_compiler::SetupActionLabel::Reset)
        .collect::<Vec<_>>();
    let state_updates = outcome
        .plan
        .actions
        .iter()
        .filter(|action| action.label == system_compiler::SetupActionLabel::Reset)
        .collect::<Vec<_>>();
    out.push_str(&format!(
        "OUTCOME: {}\n",
        setup_success_outcome_name(outcome.disposition)
    ));
    out.push_str(&format!(
        "OBJECT: {}\n",
        setup_object_name(outcome.plan.resolved_mode)
    ));
    out.push_str(&format!("NEXT SAFE ACTION: {}\n", outcome.next_safe_action));
    out.push_str("## CANONICAL ROOT\n");
    out.push_str(match outcome.plan.resolved_mode {
        system_compiler::SetupMode::Init => "STATUS: established canonical `.system/` root\n",
        system_compiler::SetupMode::Refresh => "STATUS: reused canonical `.system/` root\n",
        system_compiler::SetupMode::Auto => unreachable!("setup mode should resolve before render"),
    });
    out.push_str("## STARTER FILES\n");
    for action in starter_actions {
        out.push_str(&format!(
            "{} {}\n",
            setup_action_label_name(action.label),
            setup_action_path(action)
        ));
    }
    out.push_str("## STATE UPDATES\n");
    if state_updates.is_empty() {
        out.push_str("<none>\n");
    } else {
        for action in state_updates {
            out.push_str(&format!(
                "{} {}\n",
                setup_action_label_name(action.label),
                action.path
            ));
        }
    }
    out.push_str("## MODE NOTES\n");
    if routed_from_auto {
        out.push_str("ROUTED FROM: system setup -> ");
        out.push_str(setup_command_name(outcome.plan.resolved_mode));
        out.push('\n');
    }
    if outcome.disposition == system_compiler::SetupDisposition::Scaffolded {
        out.push_str(
            "Required starter files still contain shipped scaffold text; replace canonical truth before running `system doctor` or packet work.\n",
        );
    }
    out.push_str(
        "`PROJECT_CONTEXT.md` remains optional semantically for planning packets but is still setup-owned.\n",
    );

    out.trim_end().to_string()
}

fn setup_success_outcome_name(disposition: system_compiler::SetupDisposition) -> &'static str {
    match disposition {
        system_compiler::SetupDisposition::Ready => "READY",
        system_compiler::SetupDisposition::Scaffolded => "SCAFFOLDED",
    }
}

fn render_setup_refusal(refusal: &system_compiler::SetupRefusal) -> String {
    let mut out = String::new();
    let next_safe_action = refusal.next_safe_action.trim();
    out.push_str(&format!(
        "OUTCOME: {}\n",
        setup_refusal_outcome_name(refusal.kind)
    ));
    out.push_str("OBJECT: setup\n");
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.push_str("## REFUSAL\n");
    out.push_str(&format!(
        "CATEGORY: {}\n",
        setup_refusal_kind_name(refusal.kind)
    ));
    out.push_str(&format!("SUMMARY: {}\n", refusal.summary.trim()));
    out.push_str(&format!(
        "BROKEN SUBJECT: {}\n",
        refusal.broken_subject.trim()
    ));
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.trim_end().to_string()
}

fn setup_command_name(mode: system_compiler::SetupMode) -> &'static str {
    match mode {
        system_compiler::SetupMode::Auto => "system setup",
        system_compiler::SetupMode::Init => "system setup init",
        system_compiler::SetupMode::Refresh => "system setup refresh",
    }
}

fn setup_object_name(mode: system_compiler::SetupMode) -> &'static str {
    match mode {
        system_compiler::SetupMode::Auto => "setup",
        system_compiler::SetupMode::Init => "setup init",
        system_compiler::SetupMode::Refresh => "setup refresh",
    }
}

fn setup_action_label_name(label: system_compiler::SetupActionLabel) -> &'static str {
    match label {
        system_compiler::SetupActionLabel::Created => "created",
        system_compiler::SetupActionLabel::Preserved => "preserved",
        system_compiler::SetupActionLabel::Rewritten => "rewritten",
        system_compiler::SetupActionLabel::Reset => "reset",
    }
}

fn setup_action_path(action: &system_compiler::SetupAction) -> String {
    if action.label == system_compiler::SetupActionLabel::Created
        && action.path == ".system/project_context/PROJECT_CONTEXT.md"
    {
        format!("{} (optional)", action.path)
    } else {
        action.path.clone()
    }
}

fn setup_refusal_outcome_name(kind: system_compiler::SetupRefusalKind) -> &'static str {
    match kind {
        system_compiler::SetupRefusalKind::AlreadyInitialized
        | system_compiler::SetupRefusalKind::InvalidRequest => "REFUSED",
        system_compiler::SetupRefusalKind::MissingCanonicalRoot
        | system_compiler::SetupRefusalKind::InvalidCanonicalRoot
        | system_compiler::SetupRefusalKind::MutationRefused => "BLOCKED",
    }
}

fn setup_refusal_kind_name(kind: system_compiler::SetupRefusalKind) -> &'static str {
    match kind {
        system_compiler::SetupRefusalKind::AlreadyInitialized => "AlreadyInitialized",
        system_compiler::SetupRefusalKind::MissingCanonicalRoot => "MissingCanonicalRoot",
        system_compiler::SetupRefusalKind::InvalidCanonicalRoot => "InvalidCanonicalRoot",
        system_compiler::SetupRefusalKind::InvalidRequest => "InvalidRequest",
        system_compiler::SetupRefusalKind::MutationRefused => "MutationRefused",
    }
}

fn doctor() -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

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
        println!(
            "CATEGORY: {}",
            system_compiler::render_blocker_category(blocker.category)
        );
        println!("SUMMARY: {}", blocker.summary);
        println!(
            "SUBJECT: {}",
            system_compiler::render_subject_ref(&blocker.subject)
        );
        println!(
            "NEXT SAFE ACTION: {}",
            system_compiler::render_next_safe_action_value(&blocker.next_safe_action)
        );
        println!();
    }

    ExitCode::from(1)
}

fn pipeline(args: PipelineArgs) -> ExitCode {
    match args.command {
        PipelineCommand::List => pipeline_list(),
        PipelineCommand::Show(args) => pipeline_show(args),
        PipelineCommand::Resolve(args) => pipeline_resolve(args),
        PipelineCommand::Compile(args) => pipeline_compile(args),
        PipelineCommand::Capture(args) => pipeline_capture(args),
        PipelineCommand::Handoff(args) => pipeline_handoff(args),
        PipelineCommand::State(args) => match args.command {
            PipelineStateCommand::Set(args) => pipeline_state_set(args),
        },
    }
}

fn pipeline_list() -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let catalog = match system_compiler::load_pipeline_catalog_metadata(&repo_root) {
        Ok(catalog) => catalog,
        Err(err) => {
            println!("REFUSED: pipeline catalog error: {err}");
            return ExitCode::from(1);
        }
    };

    println!("{}", system_compiler::render_pipeline_list(&catalog));
    ExitCode::SUCCESS
}

fn pipeline_show(args: PipelineShowArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let selection = match system_compiler::load_pipeline_selection_metadata(&repo_root, &args.id) {
        Ok(selection) => selection,
        Err(system_compiler::PipelineMetadataSelectionError::Catalog(err)) => {
            println!("REFUSED: pipeline catalog error: {err}");
            return ExitCode::from(1);
        }
        Err(system_compiler::PipelineMetadataSelectionError::Lookup(err)) => {
            println!("{}", render_pipeline_selector_refusal(err));
            return ExitCode::from(1);
        }
    };

    println!("{}", system_compiler::render_pipeline_show(&selection));
    ExitCode::SUCCESS
}

fn pipeline_resolve(args: PipelineSelectorArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let catalog = match system_compiler::load_pipeline_catalog(&repo_root) {
        Ok(catalog) => catalog,
        Err(err) => {
            println!("REFUSED: pipeline catalog error: {err}");
            return ExitCode::from(1);
        }
    };

    let pipeline = match system_compiler::resolve_pipeline_only_selector(&catalog, &args.id) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            println!("{}", render_pipeline_selector_refusal(err));
            return ExitCode::from(1);
        }
    };

    let supported_variables =
        system_compiler::supported_route_state_variables(&pipeline.definition);
    let state = match system_compiler::load_route_state_with_supported_variables(
        &repo_root,
        &pipeline.definition.header.id,
        &supported_variables,
    ) {
        Ok(state) => state,
        Err(err) => {
            println!("REFUSED: {err}");
            return ExitCode::from(1);
        }
    };

    let route_variables = match system_compiler::RouteVariables::new(state.routing.clone()) {
        Ok(variables) => variables,
        Err(err) => {
            println!("REFUSED: malformed route state variables: {err}");
            return ExitCode::from(1);
        }
    };

    let route =
        match system_compiler::resolve_pipeline_route(&pipeline.definition, &route_variables) {
            Ok(route) => route,
            Err(err) => {
                println!("REFUSED: route resolution error: {err}");
                return ExitCode::from(1);
            }
        };

    let route_basis = match system_compiler::build_route_basis(
        &repo_root,
        &pipeline.definition,
        &state,
        &route,
    ) {
        Ok(route_basis) => route_basis,
        Err(err) => {
            println!("REFUSED: route basis build error: {err}");
            return ExitCode::from(1);
        }
    };

    match system_compiler::persist_route_basis(
        &repo_root,
        &pipeline.definition.header.id,
        route_basis,
    ) {
        Ok(system_compiler::RouteBasisPersistOutcome::Applied(_)) => {}
        Ok(system_compiler::RouteBasisPersistOutcome::Refused(refusal)) => {
            println!("REFUSED: route basis persistence refused: {refusal}");
            return ExitCode::from(1);
        }
        Err(err) => {
            println!("REFUSED: route basis persistence error: {err}");
            return ExitCode::from(1);
        }
    }

    println!(
        "{}",
        render_pipeline_resolve_output(
            &pipeline.definition.header.id,
            &state,
            &system_compiler::effective_route_basis_run(&repo_root, &pipeline.definition, &state),
            &route,
        )
    );
    ExitCode::SUCCESS
}

fn pipeline_compile(args: PipelineCompileArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    match system_compiler::compile_pipeline_stage(&repo_root, &args.id, &args.stage) {
        Ok(result) => {
            if args.explain {
                println!(
                    "{}",
                    system_compiler::render_pipeline_compile_explain(&result)
                );
            } else {
                println!(
                    "{}",
                    system_compiler::render_pipeline_compile_payload(&result)
                );
            }
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!(
                "{}",
                render_pipeline_compile_refusal(refusal, &args.id, &args.stage)
            );
            ExitCode::from(1)
        }
    }
}

fn pipeline_capture(args: PipelineCaptureArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    match args.command {
        Some(PipelineCaptureCommand::Apply(apply_args)) => {
            match system_compiler::apply_pipeline_capture(&repo_root, &apply_args.capture_id) {
                Ok(result) => {
                    println!(
                        "{}",
                        system_compiler::render_pipeline_capture_apply_result(&result)
                    );
                    ExitCode::SUCCESS
                }
                Err(refusal) => {
                    println!(
                        "{}",
                        system_compiler::render_pipeline_capture_refusal(&refusal, None, None)
                    );
                    ExitCode::from(1)
                }
            }
        }
        None => {
            let Some(pipeline_id) = args.id.as_deref() else {
                println!("REFUSED: `pipeline capture` requires --id");
                return ExitCode::from(1);
            };
            let Some(stage_id) = args.stage.as_deref() else {
                println!("REFUSED: `pipeline capture` requires --stage");
                return ExitCode::from(1);
            };
            let stdin = match read_stdin() {
                Ok(value) => value,
                Err(err) => {
                    println!("REFUSED: failed to read capture input from stdin: {err}");
                    return ExitCode::from(1);
                }
            };
            let request = system_compiler::PipelineCaptureRequest {
                pipeline_selector: pipeline_id.to_string(),
                stage_selector: stage_id.to_string(),
                input: stdin,
            };

            if args.preview {
                match system_compiler::preview_pipeline_capture(&repo_root, &request) {
                    Ok(preview) => {
                        println!(
                            "{}",
                            system_compiler::render_pipeline_capture_preview(&preview)
                        );
                        ExitCode::SUCCESS
                    }
                    Err(refusal) => {
                        println!(
                            "{}",
                            system_compiler::render_pipeline_capture_refusal(
                                &refusal,
                                Some(pipeline_id),
                                Some(stage_id),
                            )
                        );
                        ExitCode::from(1)
                    }
                }
            } else {
                match system_compiler::capture_pipeline_output(&repo_root, &request) {
                    Ok(result) => {
                        println!(
                            "{}",
                            system_compiler::render_pipeline_capture_apply_result(&result)
                        );
                        ExitCode::SUCCESS
                    }
                    Err(refusal) => {
                        println!(
                            "{}",
                            system_compiler::render_pipeline_capture_refusal(
                                &refusal,
                                Some(pipeline_id),
                                Some(stage_id),
                            )
                        );
                        ExitCode::from(1)
                    }
                }
            }
        }
    }
}

fn pipeline_handoff(args: PipelineHandoffArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    match args.command {
        PipelineHandoffCommand::Emit(emit_args) => {
            let request = system_compiler::PipelineHandoffEmitRequest {
                pipeline_selector: emit_args.id,
                consumer_selector: emit_args.consumer,
                producer_command: "system pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer".to_string(),
                producer_version: RELEASE_VERSION.to_string(),
            };
            match system_compiler::emit_pipeline_handoff_bundle(&repo_root, &request) {
                Ok(result) => {
                    println!(
                        "{}",
                        system_compiler::render_pipeline_handoff_emit_result(&result)
                    );
                    ExitCode::SUCCESS
                }
                Err(refusal) => {
                    println!(
                        "{}",
                        system_compiler::render_pipeline_handoff_refusal(&refusal)
                    );
                    ExitCode::from(1)
                }
            }
        }
    }
}

fn pipeline_state_set(args: PipelineStateSetArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let catalog = match system_compiler::load_pipeline_catalog(&repo_root) {
        Ok(catalog) => catalog,
        Err(err) => {
            println!("REFUSED: pipeline catalog error: {err}");
            return ExitCode::from(1);
        }
    };

    let pipeline = match system_compiler::resolve_pipeline_only_selector(&catalog, &args.id) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            println!("{}", render_pipeline_selector_refusal(err));
            return ExitCode::from(1);
        }
    };

    let supported_variables =
        system_compiler::supported_route_state_variables(&pipeline.definition);
    let current_state = match system_compiler::load_route_state_with_supported_variables(
        &repo_root,
        &pipeline.definition.header.id,
        &supported_variables,
    ) {
        Ok(state) => state,
        Err(err) => {
            println!("REFUSED: {err}");
            return ExitCode::from(1);
        }
    };

    let mutation = match parse_route_state_mutation(&args) {
        Ok(mutation) => mutation,
        Err(err) => {
            println!("REFUSED: {err}");
            return ExitCode::from(1);
        }
    };

    let expected_revision = args.expected_revision.unwrap_or(current_state.revision);
    let outcome = match system_compiler::set_route_state(
        &repo_root,
        &pipeline.definition.header.id,
        supported_variables,
        mutation,
        expected_revision,
    ) {
        Ok(outcome) => outcome,
        Err(err) => {
            println!("REFUSED: route state mutation error: {err}");
            return ExitCode::from(1);
        }
    };

    match outcome {
        system_compiler::RouteStateMutationOutcome::Applied(state) => {
            println!(
                "{}",
                render_pipeline_state_set_output(
                    &pipeline.definition.header.id,
                    system_compiler::RouteStateMutationOutcome::Applied(state),
                )
            );
            ExitCode::SUCCESS
        }
        system_compiler::RouteStateMutationOutcome::Refused(refusal) => {
            println!(
                "{}",
                render_pipeline_state_set_output(
                    &pipeline.definition.header.id,
                    system_compiler::RouteStateMutationOutcome::Refused(refusal),
                )
            );
            ExitCode::from(1)
        }
    }
}

fn render_pipeline_selector_refusal(err: system_compiler::PipelineLookupError) -> String {
    match err {
        system_compiler::PipelineLookupError::AmbiguousSelector { selector, matches } => {
            format!(
                "REFUSED: ambiguous selector `{selector}` matched multiple canonical ids: {}\nNEXT SAFE ACTION: use the full canonical id or rename the conflicting ids",
                matches.join(", ")
            )
        }
        system_compiler::PipelineLookupError::UnknownSelector { selector } => format!(
            "REFUSED: unknown pipeline selector `{selector}`; use a canonical id or `pipeline list` to inspect available inventory\nNEXT SAFE ACTION: run `pipeline list` and retry with the full canonical id"
        ),
        system_compiler::PipelineLookupError::UnsupportedSelector { selector, reason } => {
            let next_safe_action = if reason.contains("raw file paths are evidence only") {
                "use `pipeline list` to inspect available inventory and retry with a canonical pipeline or stage id"
            } else {
                "retry with a canonical pipeline id"
            };

            format!(
                "REFUSED: unsupported selector `{selector}`: {reason}\nNEXT SAFE ACTION: {next_safe_action}"
            )
        }
    }
}

fn render_pipeline_compile_refusal(
    refusal: system_compiler::PipelineCompileRefusal,
    requested_pipeline_id: &str,
    requested_stage_id: &str,
) -> String {
    let pipeline_id = refusal
        .pipeline_id
        .as_deref()
        .unwrap_or(requested_pipeline_id.trim());
    let stage_id = refusal
        .stage_id
        .as_deref()
        .unwrap_or(requested_stage_id.trim());
    let mut out = String::new();
    out.push_str("OUTCOME: REFUSED\n");
    out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
    out.push_str(&format!("STAGE: {stage_id}\n"));
    out.push_str(&format!(
        "REASON: {}: {}\n",
        render_pipeline_compile_refusal_classification(refusal.classification),
        refusal.summary.trim()
    ));
    out.push_str(&format!(
        "BROKEN SUBJECT: pipeline `{pipeline_id}` stage `{stage_id}`\n"
    ));
    out.push_str(&format!(
        "NEXT SAFE ACTION: {}\n",
        render_pipeline_compile_next_safe_action(&refusal, pipeline_id, stage_id)
    ));
    out.trim_end().to_string()
}

fn render_pipeline_compile_refusal_classification(
    classification: system_compiler::PipelineCompileRefusalClassification,
) -> &'static str {
    match classification {
        system_compiler::PipelineCompileRefusalClassification::UnsupportedTarget => {
            "unsupported_target"
        }
        system_compiler::PipelineCompileRefusalClassification::InvalidDefinition => {
            "invalid_definition"
        }
        system_compiler::PipelineCompileRefusalClassification::InvalidState => "invalid_state",
        system_compiler::PipelineCompileRefusalClassification::MissingRouteBasis => {
            "missing_route_basis"
        }
        system_compiler::PipelineCompileRefusalClassification::MalformedRouteBasis => {
            "malformed_route_basis"
        }
        system_compiler::PipelineCompileRefusalClassification::StaleRouteBasis => {
            "stale_route_basis"
        }
        system_compiler::PipelineCompileRefusalClassification::InactiveStage => "inactive_stage",
        system_compiler::PipelineCompileRefusalClassification::MissingRequiredInput => {
            "missing_required_input"
        }
        system_compiler::PipelineCompileRefusalClassification::EmptyRequiredInput => {
            "empty_required_input"
        }
    }
}

fn render_pipeline_compile_next_safe_action(
    refusal: &system_compiler::PipelineCompileRefusal,
    pipeline_id: &str,
    stage_id: &str,
) -> String {
    match refusal.classification {
        system_compiler::PipelineCompileRefusalClassification::UnsupportedTarget => {
            if refusal
                .recovery
                .trim()
                .contains("confirm the selected stage is declared in the pipeline")
            {
                format!(
                    "run `system pipeline resolve --id {pipeline_id}` and confirm `{stage_id}` is declared in pipeline `{pipeline_id}` before retrying `system pipeline compile --id {pipeline_id} --stage {stage_id}`"
                )
            } else {
                refusal.recovery.trim().to_string()
            }
        }
        system_compiler::PipelineCompileRefusalClassification::MissingRouteBasis
        | system_compiler::PipelineCompileRefusalClassification::MalformedRouteBasis
        | system_compiler::PipelineCompileRefusalClassification::StaleRouteBasis => format!(
            "run `system pipeline resolve --id {pipeline_id}` and then retry `system pipeline compile --id {pipeline_id} --stage {stage_id}`"
        ),
        system_compiler::PipelineCompileRefusalClassification::InactiveStage => format!(
            "run `system pipeline resolve --id {pipeline_id}`, adjust route state if needed, and then retry `system pipeline compile --id {pipeline_id} --stage {stage_id}`"
        ),
        _ => format!(
            "{}; then retry `system pipeline compile --id {pipeline_id} --stage {stage_id}`",
            refusal.recovery.trim()
        ),
    }
}

fn parse_route_state_mutation(
    args: &PipelineStateSetArgs,
) -> Result<system_compiler::RouteStateMutation, String> {
    match (&args.var, &args.field) {
        (Some(value), None) => parse_route_state_var_assignment(value),
        (None, Some(value)) => parse_route_state_field_assignment(value),
        (Some(_), Some(_)) => Err("use exactly one of --var or --field".to_string()),
        (None, None) => Err("one of --var or --field is required".to_string()),
    }
}

fn read_stdin() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input)
}

fn parse_route_state_var_assignment(
    value: &str,
) -> Result<system_compiler::RouteStateMutation, String> {
    let trimmed = value.trim();
    let Some((name, raw_value)) = trimmed.split_once('=') else {
        return Err("expected --var in name=value form".to_string());
    };

    let name = name.trim();
    let raw_value = raw_value.trim();
    if name.is_empty() {
        return Err("--var name must not be empty".to_string());
    }

    let parsed_value = match raw_value {
        "true" => true,
        "false" => false,
        _ => {
            return Err(format!(
                "unsupported --var value `{raw_value}`; expected `true` or `false`"
            ));
        }
    };

    Ok(system_compiler::RouteStateMutation::RoutingVariable {
        variable: name.to_string(),
        value: parsed_value,
    })
}

fn parse_route_state_field_assignment(
    value: &str,
) -> Result<system_compiler::RouteStateMutation, String> {
    let trimmed = value.trim();
    let Some((field_path, raw_value)) = trimmed.split_once('=') else {
        return Err("expected --field in field.path=value form".to_string());
    };

    let field_path = field_path.trim();
    let raw_value = raw_value.trim();
    if field_path.is_empty() {
        return Err("--field path must not be empty".to_string());
    }
    if raw_value.is_empty() {
        return Err("--field value must not be empty".to_string());
    }

    match field_path {
        "run.runner" => Ok(system_compiler::RouteStateMutation::RunRunner {
            value: raw_value.to_string(),
        }),
        "run.profile" => Ok(system_compiler::RouteStateMutation::RunProfile {
            value: raw_value.to_string(),
        }),
        "refs.charter_ref" => Ok(system_compiler::RouteStateMutation::RefCharterRef {
            value: raw_value.to_string(),
        }),
        "refs.project_context_ref" => {
            Ok(system_compiler::RouteStateMutation::RefProjectContextRef {
                value: raw_value.to_string(),
            })
        }
        _ => Err(format!(
            "unsupported --field path `{field_path}`; expected one of `run.runner`, `run.profile`, `refs.charter_ref`, or `refs.project_context_ref`"
        )),
    }
}

fn render_pipeline_resolve_output(
    pipeline_id: &str,
    state: &system_compiler::RouteState,
    effective_run: &system_compiler::RouteStateRun,
    route: &system_compiler::ResolvedPipelineRoute,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: RESOLVED\n");
    out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
    out.push_str("ROUTE BASIS:\n");
    out.push_str(&format!("  revision = {}\n", state.revision));
    out.push_str("  routing:\n");
    if state.routing.is_empty() {
        out.push_str("    <empty>\n");
    } else {
        for (name, value) in &state.routing {
            out.push_str(&format!("    {} = {}\n", name, value));
        }
    }
    out.push_str("  refs:\n");
    render_optional_route_basis_field(&mut out, "charter_ref", state.refs.charter_ref.as_deref());
    render_optional_route_basis_field(
        &mut out,
        "project_context_ref",
        state.refs.project_context_ref.as_deref(),
    );
    out.push_str("  run:\n");
    render_optional_route_basis_field(&mut out, "runner", effective_run.runner.as_deref());
    render_optional_route_basis_field(&mut out, "profile", effective_run.profile.as_deref());
    render_optional_route_basis_field(&mut out, "repo_root", effective_run.repo_root.as_deref());
    out.push_str("ROUTE:\n");

    for (index, stage) in route.stages.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {}\n",
            index + 1,
            stage.stage_id,
            stage.status.as_str()
        ));
        if let Some(reason) = &stage.reason {
            out.push_str(&format!(
                "     REASON: {}\n",
                render_route_stage_reason(reason)
            ));
        }
    }

    out.trim_end().to_string()
}

fn render_optional_route_basis_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("    {} = {}\n", name, value)),
        None => out.push_str(&format!("    {} = <unset>\n", name)),
    }
}

fn render_route_stage_reason(reason: &system_compiler::RouteStageReason) -> String {
    match reason {
        system_compiler::RouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        } => format!(
            "activation evaluated false for variables: {}",
            unsatisfied_variables.join(", ")
        ),
        system_compiler::RouteStageReason::NextMissingRouteVariables {
            missing_variables, ..
        } => format!("missing route variables: {}", missing_variables.join(", ")),
        system_compiler::RouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => format!(
            "blocked by unresolved stage {} ({})",
            upstream_stage_id,
            upstream_status.as_str()
        ),
    }
}

fn render_pipeline_state_set_output(
    pipeline_id: &str,
    outcome: system_compiler::RouteStateMutationOutcome,
) -> String {
    let mut out = String::new();
    match outcome {
        system_compiler::RouteStateMutationOutcome::Applied(state) => {
            let state = *state;
            out.push_str("OUTCOME: APPLIED\n");
            out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
            out.push_str(&format!("REVISION: {}\n", state.revision));
            out.push_str("ROUTING:\n");
            if state.routing.is_empty() {
                out.push_str("  <empty>\n");
            } else {
                for (name, value) in state.routing {
                    out.push_str(&format!("  {} = {}\n", name, value));
                }
            }
            out.push_str("REFS:\n");
            render_optional_state_field(&mut out, "charter_ref", state.refs.charter_ref.as_deref());
            render_optional_state_field(
                &mut out,
                "project_context_ref",
                state.refs.project_context_ref.as_deref(),
            );
            out.push_str("RUN:\n");
            render_optional_state_field(&mut out, "runner", state.run.runner.as_deref());
            render_optional_state_field(&mut out, "profile", state.run.profile.as_deref());
            render_optional_state_field(&mut out, "repo_root", state.run.repo_root.as_deref());
        }
        system_compiler::RouteStateMutationOutcome::Refused(refusal) => {
            out.push_str("OUTCOME: REFUSED\n");
            out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
            out.push_str(&format!("REASON: {}\n", refusal));
        }
    }

    out.trim_end().to_string()
}

fn render_optional_state_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("  {} = {}\n", name, value)),
        None => out.push_str(&format!("  {} = <unset>\n", name)),
    }
}

fn inspect(args: RequestArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
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

    let repo_root = discover_managed_repo_root(&cwd);

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

    let ready = model.packet_status == system_compiler::PacketSelectionStatus::Selected
        && model.refusal.is_none()
        && model.blockers.is_empty();

    if ready {
        println!("{}", system_compiler::render_inspect(&model));
    } else {
        let rendered = system_compiler::render_inspect(&model);
        if let Some(fixture_set_id) = demo_fixture_set_id.as_deref() {
            let section = fixture_section_for_demo(&repo_root, fixture_set_id);
            println!("{}", inject_after_first_three_lines(&rendered, &section));
        } else {
            println!("{rendered}");
        }
    }

    if ready {
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
