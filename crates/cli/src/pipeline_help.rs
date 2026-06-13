use crate::shell_shared::discover_managed_repo_root;
use clap::Command;

pub(crate) const SUPPORTED_CAPTURE_HELP_SUMMARY: &str =
    "Capture one supported stage output and materialize declared artifact and repo-mirror files for the supported pipeline stages.";
pub(crate) const SUPPORTED_CAPTURE_HELP_EXAMPLES: &str =
    "Examples are loaded from the current managed repo when supported target metadata is available.";
pub(crate) const SUPPORTED_HANDOFF_HELP_SUMMARY: &str =
    "Emit one bounded handoff bundle for the supported pipeline and consumer.";
pub(crate) const SUPPORTED_HANDOFF_EMIT_HELP_SUMMARY: &str = SUPPORTED_HANDOFF_HELP_SUMMARY;
pub(crate) const SUPPORTED_HANDOFF_HELP_EXAMPLES: &str =
    "Examples are loaded from the current managed repo when supported target metadata is available.";

#[derive(Debug, Clone)]
struct PipelineHelpText {
    capture_summary: String,
    capture_examples: String,
    handoff_summary: String,
    handoff_examples: String,
}

pub(crate) fn apply_dynamic_pipeline_help(command: Command) -> Command {
    let Some(help) = load_dynamic_pipeline_help() else {
        return command;
    };

    command.mut_subcommand("pipeline", move |pipeline| {
        let capture_summary = help.capture_summary.clone();
        let capture_examples = help.capture_examples.clone();
        let handoff_summary = help.handoff_summary.clone();
        let handoff_examples = help.handoff_examples.clone();

        pipeline
            .mut_subcommand("capture", move |capture| {
                capture.about(capture_summary).after_help(capture_examples)
            })
            .mut_subcommand("handoff", move |handoff| {
                let emit_summary = handoff_summary.clone();
                let emit_examples = handoff_examples.clone();
                handoff
                    .about(handoff_summary)
                    .after_help(handoff_examples)
                    .mut_subcommand("emit", move |emit| {
                        emit.about(emit_summary).after_help(emit_examples)
                    })
            })
    })
}

pub(crate) fn render_supported_handoff_emit_command(
    target: &handbook_pipeline::pipeline::SupportedHandoffTarget,
) -> String {
    format!(
        "handbook pipeline handoff emit --id {} --consumer {}",
        target.pipeline_id, target.consumer_id
    )
}

fn load_dynamic_pipeline_help() -> Option<PipelineHelpText> {
    let cwd = std::env::current_dir().ok()?;
    let repo_root = discover_managed_repo_root(&cwd);
    let registry = handbook_pipeline::pipeline::SupportedTargetRegistry::load(&repo_root).ok()?;
    let compile_target = registry.compile_target();
    let pipeline_id = compile_target.pipeline.id.clone();
    let compile_stage_id = compile_target.stage.id.clone();
    let capture_stage_ids = registry
        .stages()
        .filter(|stage| registry.supports_capture_target(&pipeline_id, &stage.id))
        .map(|stage| stage.id.clone())
        .collect::<Vec<_>>();
    let first_capture_stage_id = capture_stage_ids.first()?.clone();
    let preview_stage_id = capture_stage_ids
        .iter()
        .rev()
        .find(|stage_id| *stage_id != &compile_stage_id)
        .cloned()
        .unwrap_or_else(|| first_capture_stage_id.clone());
    let rendered_stage_list = capture_stage_ids
        .iter()
        .map(|stage_id| format!("`{stage_id}`"))
        .collect::<Vec<_>>();
    let handoff_target = registry.handoff_target();
    let handoff_command = render_supported_handoff_emit_command(&handoff_target);

    Some(PipelineHelpText {
        capture_summary: format!(
            "Capture one supported stage output and materialize declared artifact and repo-mirror files for `{pipeline_id}` stages {}",
            render_human_stage_list(&rendered_stage_list)
        ),
        capture_examples: format!(
            "Examples:\n  handbook pipeline capture --id {pipeline_id} --stage {first_capture_stage_id}\n  handbook pipeline capture --id {pipeline_id} --stage {preview_stage_id} --preview\n  handbook pipeline capture --id {pipeline_id} --stage {compile_stage_id} < /tmp/FEATURE_SPEC.md\n  handbook pipeline capture apply --capture-id <capture-id>"
        ),
        handoff_summary: format!(
            "Emit one bounded handoff bundle for `{}` -> `{}`",
            handoff_target.pipeline_id, handoff_target.consumer_id
        ),
        handoff_examples: format!("Example:\n  {handoff_command}"),
    })
}

fn render_human_stage_list(stage_ids: &[String]) -> String {
    match stage_ids {
        [] => String::new(),
        [only] => only.clone(),
        [left, right] => format!("{left} and {right}"),
        _ => format!(
            "{}, and {}",
            stage_ids[..stage_ids.len() - 1].join(", "),
            stage_ids.last().expect("non-empty stage list")
        ),
    }
}
