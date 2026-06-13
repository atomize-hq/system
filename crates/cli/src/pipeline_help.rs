pub(crate) const SUPPORTED_CAPTURE_HELP_SUMMARY: &str =
    "Capture one supported stage output and materialize declared artifact and repo-mirror files for `pipeline.foundation_inputs` stages `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, and `stage.10_feature_spec`";
pub(crate) const SUPPORTED_CAPTURE_HELP_EXAMPLES: &str = "Examples:
  handbook pipeline capture --id pipeline.foundation_inputs --stage stage.04_charter_inputs
  handbook pipeline capture --id pipeline.foundation_inputs --stage stage.07_foundation_pack --preview
  handbook pipeline capture --id pipeline.foundation_inputs --stage stage.10_feature_spec < /tmp/FEATURE_SPEC.md
  handbook pipeline capture apply --capture-id <capture-id>";
pub(crate) const SUPPORTED_HANDOFF_HELP_SUMMARY: &str =
    "Emit one bounded handoff bundle for `pipeline.foundation_inputs` -> `feature-slice-decomposer`";
pub(crate) const SUPPORTED_HANDOFF_EMIT_HELP_SUMMARY: &str = SUPPORTED_HANDOFF_HELP_SUMMARY;
pub(crate) const SUPPORTED_HANDOFF_HELP_EXAMPLES: &str = "Example:
  handbook pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer";

pub(crate) fn render_supported_handoff_emit_command(
    target: &handbook_pipeline::pipeline::SupportedHandoffTarget,
) -> String {
    format!(
        "handbook pipeline handoff emit --id {} --consumer {}",
        target.pipeline_id, target.consumer_id
    )
}
