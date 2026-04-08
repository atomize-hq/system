use super::json::render_json;
use super::model::RenderOutputModel;
use super::shared::{
    push_line, render_blocker_category, render_budget_disposition, render_budget_next_safe_action,
    render_budget_reason, render_next_safe_action_from_model, render_outcome,
    render_refusal_category, render_subject_ref,
};

pub fn render_inspect(model: &RenderOutputModel) -> String {
    let mut output = String::new();

    push_line(
        &mut output,
        format!(
            "OUTCOME: {}",
            render_outcome(model.packet_status, model.refusal.is_some())
        ),
    );
    push_line(&mut output, format!("OBJECT: {}", model.packet_id));
    push_line(
        &mut output,
        format!(
            "NEXT SAFE ACTION: {}",
            render_next_safe_action_from_model(
                &model.packet_result,
                model.refusal.as_ref(),
                &model.blockers
            )
        ),
    );

    output.push('\n');
    push_line(&mut output, "## DECISION LOG");
    for (index, entry) in model.decision_log_entries.iter().enumerate() {
        push_line(&mut output, format!("{}. {}", index + 1, entry));
    }

    output.push('\n');
    push_line(&mut output, "## BUDGET OUTCOME");
    push_line(
        &mut output,
        format!(
            "DISPOSITION: {}",
            render_budget_disposition(model.budget_outcome.disposition)
        ),
    );
    push_line(
        &mut output,
        format!(
            "REASON: {}",
            render_budget_reason(&model.budget_outcome.reason)
        ),
    );
    if model.budget_outcome.targets.is_empty() {
        push_line(&mut output, "TARGETS: NONE");
    } else {
        for (index, target) in model.budget_outcome.targets.iter().enumerate() {
            push_line(
                &mut output,
                format!(
                    "TARGET {}: {} ({} bytes)",
                    index + 1,
                    target.canonical_repo_relative_path,
                    target.byte_len
                ),
            );
        }
    }
    match render_budget_next_safe_action(model.budget_outcome.next_safe_action.as_ref()) {
        Some(next_safe_action) => push_line(
            &mut output,
            format!("NEXT SAFE ACTION: {}", next_safe_action),
        ),
        None => push_line(&mut output, "NEXT SAFE ACTION: NONE"),
    }

    output.push('\n');
    push_line(&mut output, "## REFUSAL");
    match model.refusal.as_ref() {
        Some(refusal) => {
            push_line(
                &mut output,
                format!("CATEGORY: {}", render_refusal_category(refusal.category)),
            );
            push_line(&mut output, format!("SUMMARY: {}", refusal.summary));
            push_line(
                &mut output,
                format!(
                    "BROKEN SUBJECT: {}",
                    render_subject_ref(&refusal.broken_subject)
                ),
            );
            push_line(
                &mut output,
                format!(
                    "NEXT SAFE ACTION: {}",
                    super::shared::render_next_safe_action_value(&refusal.next_safe_action)
                ),
            );
        }
        None => push_line(&mut output, "NONE"),
    }

    output.push('\n');
    push_line(&mut output, "## BLOCKERS");
    if model.blockers.is_empty() {
        push_line(&mut output, "NONE");
    } else {
        for (index, blocker) in model.blockers.iter().enumerate() {
            if index > 0 {
                output.push('\n');
            }
            push_line(
                &mut output,
                format!("CATEGORY: {}", render_blocker_category(blocker.category)),
            );
            push_line(&mut output, format!("SUMMARY: {}", blocker.summary));
            push_line(
                &mut output,
                format!("SUBJECT: {}", render_subject_ref(&blocker.subject)),
            );
            push_line(
                &mut output,
                format!(
                    "NEXT SAFE ACTION: {}",
                    super::shared::render_next_safe_action_value(&blocker.next_safe_action)
                ),
            );
        }
    }

    if model.packet_result.is_ready() {
        output.push('\n');
        push_line(&mut output, "## PACKET OVERVIEW");
        push_line(
            &mut output,
            format!(
                "PACKET VARIANT: {}",
                super::shared::render_packet_variant(model.packet_result.variant)
            ),
        );
        push_line(
            &mut output,
            format!(
                "SUMMARY: {}",
                model.packet_result.decision_summary.summary_line
            ),
        );
        output.push('\n');
        super::shared::render_packet_body(&mut output, &model.packet_result);
    }

    output.push('\n');
    push_line(&mut output, "## JSON FALLBACK");
    push_line(&mut output, render_json(model).trim_end());

    output
}
