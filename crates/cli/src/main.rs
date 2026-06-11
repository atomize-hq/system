mod author;
mod pipeline;
mod setup;
mod shell_shared;

use clap::{CommandFactory, Parser, Subcommand};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PACKET_PLANNING_ID: &str = "planning.packet";
const PACKET_EXECUTION_DEMO_ID: &str = "execution.demo.packet";
const PACKET_EXECUTION_LIVE_ID: &str = "execution.live.packet";
const RELEASE_VERSION: &str = env!("HANDBOOK_RELEASE_VERSION");

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
    name = "handbook",
    version = RELEASE_VERSION,
    disable_help_subcommand = true,
    about = "Rust CLI for the reduced v1 handbook: `setup` initializes or refreshes canonical repo-local `.handbook/` inputs, `author` is the baseline authoring surface for charter, project context, and environment inventory, `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations, planning packet generation uses canonical repo-local `.handbook/` inputs, fixture-backed execution demo flows through `execution.demo.packet`, live execution is explicitly refused, `inspect` is the packet proof surface, and `doctor` is the recovery surface.",
    long_about = "Rust CLI for the reduced v1 handbook. `setup` initializes or refreshes canonical repo-local `.handbook/` inputs. `author` is the baseline authoring surface for charter, project context, and environment inventory. `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations. planning packet generation uses canonical repo-local `.handbook/` inputs. fixture-backed execution demo flows through `execution.demo.packet`. live execution is explicitly refused. `inspect` is the packet proof surface. `doctor` is the recovery surface."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize or refresh canonical repo-local `.handbook/` inputs.
    Setup(SetupArgs),
    /// Human-guided and deterministic baseline authoring surfaces.
    Author(AuthorArgs),
    /// Pipeline operator surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations.
    Pipeline(PipelineArgs),
    /// Generate a reduced-v1 packet.
    Generate(RequestArgs),
    /// Inspect packet composition and decision evidence.
    Inspect(RequestArgs),
    /// Diagnose blockers and safe next actions.
    Doctor(DoctorArgs),
}

impl Command {
    fn run(self) -> ExitCode {
        match self {
            Command::Setup(args) => setup::run(args),
            Command::Author(args) => author::run(args),
            Command::Pipeline(args) => pipeline::run(args),
            Command::Generate(args) => generate(args),
            Command::Inspect(args) => inspect(args),
            Command::Doctor(args) => doctor(args),
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
    /// Create canonical `.handbook/` scaffold and starter files for first-run setup.
    Init,
    /// Preserve canonical files by default and optionally rewrite starter files or reset `.handbook/state/**`.
    Refresh(SetupRefreshArgs),
}

#[derive(clap::Args, Debug)]
struct SetupRefreshArgs {
    /// Rewrite setup-owned starter files in place.
    #[arg(long)]
    rewrite: bool,
    /// Reset only `.handbook/state/**`.
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
    /// Author canonical `.handbook/charter/CHARTER.md`.
    Charter(AuthorCharterArgs),
    /// Author canonical `.handbook/project_context/PROJECT_CONTEXT.md`.
    ProjectContext(AuthorProjectContextArgs),
    /// Author canonical `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`.
    EnvironmentInventory,
}

#[derive(clap::Args, Debug)]
struct AuthorCharterArgs {
    /// Read normalized structured inputs from a YAML file or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: Option<String>,
    /// Validate normalized structured inputs and repo write preconditions without mutation.
    #[arg(long)]
    validate: bool,
}

#[derive(clap::Args, Debug)]
struct AuthorProjectContextArgs {
    /// Read normalized structured inputs from a YAML file or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: Option<String>,
}

#[derive(clap::Args, Debug)]
struct DoctorArgs {
    /// Emit machine-readable JSON to stdout.
    #[arg(long)]
    json: bool,
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
        if std::fs::symlink_metadata(candidate.join(".handbook")).is_ok() {
            return Some(candidate.to_path_buf());
        }
    }

    None
}

fn discover_managed_repo_root(start: &Path) -> PathBuf {
    shell_shared::discover_managed_repo_root(start)
}

fn fixture_lineage_for_demo(repo_root: &Path, fixture_set_id: &str) -> Vec<String> {
    let base = execution_demo_fixture_set_dir(repo_root, fixture_set_id).join(".handbook");

    let project_context = base.join("project_context/PROJECT_CONTEXT.md");

    let mut out = Vec::new();
    out.push(format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/charter/CHARTER.md"
    ));
    if project_context.is_file() {
        out.push(format!(
            "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/project_context/PROJECT_CONTEXT.md"
        ));
    }
    out.push(format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/feature_spec/FEATURE_SPEC.md"
    ));
    out
}

fn fixture_section_for_demo(repo_root: &Path, fixture_set_id: &str) -> String {
    let mut out = String::new();
    out.push_str("MODE: fixture-backed execution demo\n");
    out.push_str("## FIXTURE DEMO\n");
    out.push_str(&format!("FIXTURE SET: {fixture_set_id}\n"));
    out.push_str(&format!(
        "FIXTURE BASIS ROOT: tests/fixtures/execution_demo/{fixture_set_id}/.handbook/\n"
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
            let basis_root = fixture_set_dir.join(".handbook");
            if let Err(err) = ensure_dir(&basis_root, "fixture basis root") {
                println!("REFUSED: {err}");
                return ExitCode::from(1);
            }
            fixture_set_dir
        }
    };

    let result = match handbook_flow::resolve(
        &compiler_root,
        handbook_flow::ResolveRequest {
            packet_id: packet_id.as_str(),
            ..handbook_flow::ResolveRequest::default()
        },
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("REFUSED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    let ready = result.selection.status == handbook_flow::PacketSelectionStatus::Selected
        && result.refusal.is_none()
        && result.blockers.is_empty();

    let compiler_result = flow_result_for_rendering(result);
    let model = match handbook_compiler::build_output_model(&compiler_result) {
        Ok(model) => model,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    println!("{}", handbook_compiler::render_markdown(&model));
    if ready {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn flow_result_for_rendering(
    result: handbook_flow::ResolverResult,
) -> handbook_compiler::ResolverResult {
    handbook_compiler::ResolverResult {
        c04_result_version: result.c04_result_version,
        c03_schema_version: result.c03_schema_version,
        c03_manifest_generation_version: result.c03_manifest_generation_version,
        c03_fingerprint_sha256: result.c03_fingerprint_sha256,
        packet_result: result.packet_result,
        decision_log: handbook_compiler::DecisionLog {
            entries: result.decision_log_entries,
        },
        budget_outcome: result.budget_outcome,
        selection: result.selection,
        refusal: result.refusal.map(flow_refusal_for_rendering),
        blockers: result
            .blockers
            .into_iter()
            .map(flow_blocker_for_rendering)
            .collect(),
    }
}

fn flow_refusal_for_rendering(
    refusal: handbook_flow::ResolverRefusal,
) -> handbook_compiler::Refusal {
    handbook_compiler::Refusal {
        category: flow_refusal_category_for_rendering(refusal.category),
        summary: refusal.summary,
        broken_subject: flow_subject_ref_for_rendering(refusal.broken_subject),
        next_safe_action: flow_next_safe_action_for_rendering(refusal.next_safe_action),
    }
}

fn flow_blocker_for_rendering(
    blocker: handbook_flow::ResolverBlocker,
) -> handbook_compiler::Blocker {
    handbook_compiler::Blocker {
        category: flow_blocker_category_for_rendering(blocker.category),
        subject: flow_subject_ref_for_rendering(blocker.subject),
        summary: blocker.summary,
        next_safe_action: flow_next_safe_action_for_rendering(blocker.next_safe_action),
    }
}

fn flow_refusal_category_for_rendering(
    category: handbook_flow::ResolverRefusalCategory,
) -> handbook_compiler::RefusalCategory {
    match category {
        handbook_flow::ResolverRefusalCategory::NonCanonicalInputAttempt => {
            handbook_compiler::RefusalCategory::NonCanonicalInputAttempt
        }
        handbook_flow::ResolverRefusalCategory::SystemRootMissing => {
            handbook_compiler::RefusalCategory::SystemRootMissing
        }
        handbook_flow::ResolverRefusalCategory::SystemRootNotDir => {
            handbook_compiler::RefusalCategory::SystemRootNotDir
        }
        handbook_flow::ResolverRefusalCategory::SystemRootSymlinkNotAllowed => {
            handbook_compiler::RefusalCategory::SystemRootSymlinkNotAllowed
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactMissing => {
            handbook_compiler::RefusalCategory::RequiredArtifactMissing
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactEmpty => {
            handbook_compiler::RefusalCategory::RequiredArtifactEmpty
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactStarterTemplate => {
            handbook_compiler::RefusalCategory::RequiredArtifactStarterTemplate
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactInvalid => {
            handbook_compiler::RefusalCategory::RequiredArtifactInvalid
        }
        handbook_flow::ResolverRefusalCategory::ArtifactReadError => {
            handbook_compiler::RefusalCategory::ArtifactReadError
        }
        handbook_flow::ResolverRefusalCategory::FreshnessInvalid => {
            handbook_compiler::RefusalCategory::FreshnessInvalid
        }
        handbook_flow::ResolverRefusalCategory::BudgetRefused => {
            handbook_compiler::RefusalCategory::BudgetRefused
        }
        handbook_flow::ResolverRefusalCategory::UnsupportedRequest => {
            handbook_compiler::RefusalCategory::UnsupportedRequest
        }
    }
}

fn flow_blocker_category_for_rendering(
    category: handbook_flow::ResolverBlockerCategory,
) -> handbook_compiler::BlockerCategory {
    match category {
        handbook_flow::ResolverBlockerCategory::SystemRootMissing => {
            handbook_compiler::BlockerCategory::SystemRootMissing
        }
        handbook_flow::ResolverBlockerCategory::SystemRootNotDir => {
            handbook_compiler::BlockerCategory::SystemRootNotDir
        }
        handbook_flow::ResolverBlockerCategory::SystemRootSymlinkNotAllowed => {
            handbook_compiler::BlockerCategory::SystemRootSymlinkNotAllowed
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactMissing => {
            handbook_compiler::BlockerCategory::RequiredArtifactMissing
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactEmpty => {
            handbook_compiler::BlockerCategory::RequiredArtifactEmpty
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactStarterTemplate => {
            handbook_compiler::BlockerCategory::RequiredArtifactStarterTemplate
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactInvalid => {
            handbook_compiler::BlockerCategory::RequiredArtifactInvalid
        }
        handbook_flow::ResolverBlockerCategory::ArtifactReadError => {
            handbook_compiler::BlockerCategory::ArtifactReadError
        }
        handbook_flow::ResolverBlockerCategory::FreshnessInvalid => {
            handbook_compiler::BlockerCategory::FreshnessInvalid
        }
        handbook_flow::ResolverBlockerCategory::BudgetRefused => {
            handbook_compiler::BlockerCategory::BudgetRefused
        }
        handbook_flow::ResolverBlockerCategory::UnsupportedRequest => {
            handbook_compiler::BlockerCategory::UnsupportedRequest
        }
    }
}

fn flow_subject_ref_for_rendering(
    subject: handbook_flow::ResolverSubjectRef,
) -> handbook_compiler::SubjectRef {
    match subject {
        handbook_flow::ResolverSubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        } => handbook_compiler::SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverSubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => handbook_compiler::SubjectRef::InheritedDependency {
            dependency_id,
            version,
        },
        handbook_flow::ResolverSubjectRef::Policy { policy_id } => {
            handbook_compiler::SubjectRef::Policy { policy_id }
        }
    }
}

fn flow_next_safe_action_for_rendering(
    action: handbook_flow::ResolverNextSafeAction,
) -> handbook_compiler::NextSafeAction {
    match action {
        handbook_flow::ResolverNextSafeAction::RunSetup => {
            handbook_compiler::NextSafeAction::RunSetup
        }
        handbook_flow::ResolverNextSafeAction::RunSetupInit => {
            handbook_compiler::NextSafeAction::RunSetupInit
        }
        handbook_flow::ResolverNextSafeAction::RunSetupRefresh => {
            handbook_compiler::NextSafeAction::RunSetupRefresh
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorCharter => {
            handbook_compiler::NextSafeAction::RunAuthorCharter
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorProjectContext => {
            handbook_compiler::NextSafeAction::RunAuthorProjectContext
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorEnvironmentInventory => {
            handbook_compiler::NextSafeAction::RunAuthorEnvironmentInventory
        }
        handbook_flow::ResolverNextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::RunGenerate { packet_id } => {
            handbook_compiler::NextSafeAction::RunGenerate { packet_id }
        }
        handbook_flow::ResolverNextSafeAction::RunDoctor => {
            handbook_compiler::NextSafeAction::RunDoctor
        }
    }
}

fn render_doctor_json(report: &handbook_compiler::DoctorReport) -> Result<String, String> {
    serde_json::to_string_pretty(report)
        .map(|mut output| {
            output.push('\n');
            output
        })
        .map_err(|err| format!("failed to serialize doctor json: {err}"))
}

fn doctor(args: DoctorArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let report = match handbook_compiler::doctor(&repo_root) {
        Ok(report) => report,
        Err(err) => {
            println!("INVALID_BASELINE");
            println!("SUMMARY: failed to inspect baseline truth: {err}");
            return ExitCode::from(1);
        }
    };

    if args.json {
        match render_doctor_json(&report) {
            Ok(json) => print!("{json}"),
            Err(err) => {
                println!("INVALID_BASELINE");
                println!("SUMMARY: {err}");
                return ExitCode::from(1);
            }
        }
    } else {
        println!("{}", doctor_status_name(report.status));
        println!(
            "ROOT STATUS: {}",
            doctor_root_status_name(report.system_root_status)
        );
        if let Some(next_safe_action) = &report.next_safe_action {
            println!(
                "NEXT SAFE ACTION: {}",
                handbook_compiler::render_next_safe_action_value(next_safe_action)
            );
        } else {
            println!("NEXT SAFE ACTION: <none>");
        }
        println!("## BASELINE CHECKLIST");
        for item in report.checklist.iter() {
            let subject_path = match &item.subject {
                handbook_compiler::SubjectRef::CanonicalArtifact {
                    canonical_repo_relative_path,
                    ..
                } => *canonical_repo_relative_path,
                _ => item.canonical_repo_relative_path,
            };
            println!(
                "{} [{}] STATUS: {} ACTION: {}",
                item.artifact_label,
                subject_path,
                doctor_artifact_status_name(item.status),
                item.author_command
            );
        }
    }

    if report.status == handbook_compiler::DoctorBaselineStatus::BaselineComplete {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn doctor_status_name(status: handbook_compiler::DoctorBaselineStatus) -> &'static str {
    match status {
        handbook_compiler::DoctorBaselineStatus::Scaffolded => "SCAFFOLDED",
        handbook_compiler::DoctorBaselineStatus::PartialBaseline => "PARTIAL_BASELINE",
        handbook_compiler::DoctorBaselineStatus::InvalidBaseline => "INVALID_BASELINE",
        handbook_compiler::DoctorBaselineStatus::BaselineComplete => "BASELINE_COMPLETE",
    }
}

fn doctor_root_status_name(status: handbook_compiler::SystemRootStatus) -> &'static str {
    match status {
        handbook_compiler::SystemRootStatus::Ok => "OK",
        handbook_compiler::SystemRootStatus::Missing => "MISSING",
        handbook_compiler::SystemRootStatus::NotDir => "NOT_DIR",
        handbook_compiler::SystemRootStatus::SymlinkNotAllowed => "SYMLINK_NOT_ALLOWED",
    }
}

fn doctor_artifact_status_name(status: handbook_compiler::DoctorArtifactStatus) -> &'static str {
    match status {
        handbook_compiler::DoctorArtifactStatus::Missing => "MISSING",
        handbook_compiler::DoctorArtifactStatus::Empty => "EMPTY",
        handbook_compiler::DoctorArtifactStatus::StarterOwned => "STARTER_OWNED",
        handbook_compiler::DoctorArtifactStatus::Invalid => "INVALID",
        handbook_compiler::DoctorArtifactStatus::ValidCanonicalTruth => "VALID_CANONICAL_TRUTH",
    }
}

fn read_stdin() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input)
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
            let basis_root = fixture_set_dir.join(".handbook");
            if let Err(err) = ensure_dir(&basis_root, "fixture basis root") {
                println!("BLOCKED: {err}");
                return ExitCode::from(1);
            }
            (fixture_set_dir, Some(fixture_set_id.to_string()))
        }
    };

    let result = match handbook_flow::resolve(
        &compiler_root,
        handbook_flow::ResolveRequest {
            packet_id: packet_id.as_str(),
            ..handbook_flow::ResolveRequest::default()
        },
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("BLOCKED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    let ready = result.selection.status == handbook_flow::PacketSelectionStatus::Selected
        && result.refusal.is_none()
        && result.blockers.is_empty();

    let compiler_result = flow_result_for_rendering(result);
    let model = match handbook_compiler::build_output_model(&compiler_result) {
        Ok(model) => model,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    if ready {
        println!("{}", handbook_compiler::render_inspect(&model));
    } else {
        let rendered = handbook_compiler::render_inspect(&model);
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
        std::mem::size_of::<handbook_compiler::DecisionLog>(),
        std::mem::size_of::<handbook_flow::PacketResult>(),
        std::mem::size_of::<handbook_compiler::CompilerError>(),
        std::mem::size_of::<handbook_compiler::Refusal>(),
    );
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::author::{execute_author_charter_command, render_author_charter_refusal};
    use std::{cell::Cell, fs};

    fn valid_structured_inputs_yaml() -> &'static str {
        r#"schema_version: "0.1.0"
project:
  name: "Handbook"
  classification: greenfield
  team_size: 2
  users: internal
  expected_lifetime: months
  surfaces:
    - cli
    - api
  runtime_environments:
    - server
  constraints:
    deadline: ""
    budget: ""
    experience_notes: "small team"
    must_use_tech:
      - rust
  operational_reality:
    in_production_today: false
    prod_users_or_data: ""
    external_contracts_to_preserve: []
    uptime_expectations: "best effort"
  default_implications:
    backward_compatibility: not_required
    migration_planning: not_required
    rollout_controls: lightweight
    deprecation_policy: not_required_yet
    observability_threshold: standard
posture:
  rubric_scale: "1-5"
  baseline_level: 3
  baseline_rationale:
    - "internal operators"
    - "moderate blast radius"
domains:
  - name: "planning"
    blast_radius: "medium"
    touches:
      - "internal operators"
    constraints:
      - "preserve trust boundaries"
dimensions:
  - name: speed_vs_quality
    level: 3
    default_stance: "optimize for durability over shortcuts"
    raise_the_bar_triggers: ["production data"]
    allowed_shortcuts: ["time-boxed exploration"]
    red_lines: ["ship without review"]
    domain_overrides: []
  - name: type_safety_static_analysis
    level: 3
    default_stance: "type-safe by default"
    raise_the_bar_triggers: ["cross-boundary interfaces"]
    allowed_shortcuts: ["fixture-backed exploration"]
    red_lines: ["unchecked public contracts"]
    domain_overrides: []
  - name: testing_rigor
    level: 3
    default_stance: "test the shipped path"
    raise_the_bar_triggers: ["regression risk"]
    allowed_shortcuts: ["manual validation for throwaway work"]
    red_lines: ["merge without exercising the path"]
    domain_overrides: []
  - name: scalability_performance
    level: 3
    default_stance: "track obvious bottlenecks"
    raise_the_bar_triggers: ["user-visible latency"]
    allowed_shortcuts: ["defer micro-optimizations"]
    red_lines: ["ignore known load cliffs"]
    domain_overrides: []
  - name: reliability_operability
    level: 3
    default_stance: "prefer recoverable changes"
    raise_the_bar_triggers: ["long-lived state changes"]
    allowed_shortcuts: ["local-only iteration"]
    red_lines: ["unrecoverable migrations without a plan"]
    domain_overrides: []
  - name: security_privacy
    level: 3
    default_stance: "protect boundaries by default"
    raise_the_bar_triggers: ["credentials or user data"]
    allowed_shortcuts: ["synthetic data in local dev"]
    red_lines: ["plaintext secrets"]
    domain_overrides: []
  - name: observability
    level: 3
    default_stance: "emit enough proof to debug production issues"
    raise_the_bar_triggers: ["background jobs"]
    allowed_shortcuts: ["manual logs for local-only work"]
    red_lines: ["silent failures"]
    domain_overrides: []
  - name: dx_tooling_automation
    level: 3
    default_stance: "prefer automation that pays for itself"
    raise_the_bar_triggers: ["frequent repeated workflows"]
    allowed_shortcuts: ["temporary local scripts"]
    red_lines: ["manual-only release steps"]
    domain_overrides: []
  - name: ux_polish_api_usability
    level: 3
    default_stance: "clear operator and API ergonomics"
    raise_the_bar_triggers: ["external users"]
    allowed_shortcuts: ["rough internal copy while iterating"]
    red_lines: ["unclear operator failure modes"]
    domain_overrides: []
exceptions:
  approvers:
    - project_owner
  record_location: ".handbook/charter/CHARTER.md#exceptions"
  minimum_fields:
    - what
    - why
    - scope
    - risk
    - owner
    - expiry_or_revisit_date
debt_tracking:
  system: "issues"
  labels:
    - debt
  review_cadence: "monthly"
decision_records:
  enabled: false
  path: ""
  format: ""
"#
    }

    #[test]
    fn execute_author_charter_command_renders_guided_success_with_injected_author() {
        let dir = tempfile::tempdir().expect("tempdir");
        let collect_called = Cell::new(false);
        let author_called = Cell::new(false);

        let rendered = execute_author_charter_command(
            AuthorCharterArgs {
                from_inputs: None,
                validate: false,
            },
            || Ok(dir.path().to_path_buf()),
            || true,
            |_| Ok(()),
            |_, _| panic!("guided mode should not run from-input preflight"),
            || {
                collect_called.set(true);
                handbook_compiler::parse_charter_structured_input_yaml(
                    valid_structured_inputs_yaml(),
                )
                .map_err(|refusal| render_author_charter_refusal(&refusal))
            },
            |repo_root, input| {
                author_called.set(true);
                assert_eq!(repo_root, dir.path());
                assert_eq!(input.project.name, "Handbook");
                Ok(handbook_compiler::AuthorCharterResult {
                    canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
                    bytes_written: 42,
                })
            },
            |_, _| panic!("guided mode should not run deterministic author"),
        );

        assert!(collect_called.get(), "guided input should be collected");
        assert!(author_called.get(), "authoring closure should be called");
        assert_eq!(rendered.exit_code, ExitCode::SUCCESS);
        assert!(rendered.output.contains("OUTCOME: AUTHORED"));
        assert!(rendered.output.contains("MODE: guided_interview"));
        assert!(rendered.output.contains("SOURCE: interactive terminal"));
    }

    #[test]
    fn execute_author_charter_command_renders_file_success_with_injected_author() {
        let dir = tempfile::tempdir().expect("tempdir");
        let inputs_path = dir.path().join("charter-inputs.yaml");
        fs::write(&inputs_path, valid_structured_inputs_yaml()).expect("write inputs");
        let author_called = Cell::new(false);

        let rendered = execute_author_charter_command(
            AuthorCharterArgs {
                from_inputs: Some(inputs_path.to_string_lossy().into_owned()),
                validate: false,
            },
            || Ok(dir.path().to_path_buf()),
            || panic!("file inputs should not check interactive tty state"),
            |_| panic!("file inputs should not run guided preflight"),
            |_, _| Ok(()),
            || panic!("file inputs should not run guided collection"),
            |_, _| panic!("file inputs should not run guided author"),
            |repo_root, input| {
                author_called.set(true);
                assert_eq!(repo_root, dir.path());
                assert_eq!(input.project.name, "Handbook");
                Ok(handbook_compiler::AuthorCharterResult {
                    canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
                    bytes_written: 24,
                })
            },
        );

        assert!(author_called.get(), "authoring closure should be called");
        assert_eq!(rendered.exit_code, ExitCode::SUCCESS);
        assert!(rendered.output.contains("MODE: structured_inputs_file"));
        assert!(rendered
            .output
            .contains(&format!("SOURCE: {}", inputs_path.display())));
    }

    #[test]
    fn execute_author_charter_command_refuses_without_tty_for_guided_mode() {
        let dir = tempfile::tempdir().expect("tempdir");

        let rendered = execute_author_charter_command(
            AuthorCharterArgs {
                from_inputs: None,
                validate: false,
            },
            || Ok(dir.path().to_path_buf()),
            || false,
            |_| panic!("guided non-tty refusal should happen before preflight"),
            |_, _| panic!("guided non-tty refusal should happen before from-input preflight"),
            || panic!("guided collection should not run without tty"),
            |_, _| panic!("authoring should not run without tty"),
            |_, _| panic!("deterministic author should not run without tty"),
        );

        assert_eq!(rendered.exit_code, ExitCode::from(1));
        assert!(rendered.output.contains("OUTCOME: REFUSED"));
        assert!(rendered.output.contains("CATEGORY: NonInteractiveRefusal"));
        assert!(rendered
            .output
            .contains("run `handbook author charter --from-inputs <path|->`"));
    }

    #[test]
    fn execute_author_charter_command_refuses_during_preflight_before_guided_collection() {
        let dir = tempfile::tempdir().expect("tempdir");
        let collect_called = Cell::new(false);

        let rendered = execute_author_charter_command(
            AuthorCharterArgs {
                from_inputs: None,
                validate: false,
            },
            || Ok(dir.path().to_path_buf()),
            || true,
            |_| {
                Err(handbook_compiler::AuthorCharterRefusal {
                    kind: handbook_compiler::AuthorCharterRefusalKind::ExistingCanonicalTruth,
                    summary: "canonical charter truth already exists".to_string(),
                    broken_subject: ".handbook/charter/CHARTER.md".to_string(),
                    next_safe_action:
                        "inspect `.handbook/charter/CHARTER.md` instead of rerunning `handbook author charter`"
                            .to_string(),
                })
            },
            |_, _| panic!("guided preflight refusal should happen before from-input preflight"),
            || {
                collect_called.set(true);
                panic!("guided collection should not run after preflight refusal")
            },
            |_, _| panic!("authoring should not run after preflight refusal"),
            |_, _| panic!("deterministic author should not run after preflight refusal"),
        );

        assert!(
            !collect_called.get(),
            "guided input should not be collected"
        );
        assert_eq!(rendered.exit_code, ExitCode::from(1));
        assert!(rendered.output.contains("OUTCOME: REFUSED"));
        assert!(rendered.output.contains("CATEGORY: ExistingCanonicalTruth"));
    }
}
