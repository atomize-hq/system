use super::model::RenderOutputModel;
use super::shared::{
    json_string, render_blocker_category, render_budget_disposition,
    render_budget_next_safe_action, render_budget_reason, render_canonical_artifact_kind,
    render_packet_status, render_refusal_category,
};
use crate::{Blocker, Refusal, SubjectRef};
use std::fmt::Write;

pub fn render_json(model: &RenderOutputModel) -> String {
    let mut output = String::new();
    output.push_str("{\n");
    write_line(
        &mut output,
        1,
        "\"c04_result_version\":",
        &json_string(&model.c04_result_version),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"c03_schema_version\":",
        &json_string(&model.c03_schema_version),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"c03_manifest_generation_version\":",
        &model.c03_manifest_generation_version.to_string(),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"c03_fingerprint_sha256\":",
        &json_string(&model.c03_fingerprint_sha256),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"packet_id\":",
        &json_string(&model.packet_id),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"packet_status\":",
        &json_string(render_packet_status(model.packet_status)),
        true,
    );
    output.push_str("  \"budget_outcome\": ");
    output.push_str(&render_budget_outcome_json(model));
    output.push_str(",\n");
    output.push_str("  \"decision_log_entries\": [\n");
    for (index, entry) in model.decision_log_entries.iter().enumerate() {
        writeln!(
            &mut output,
            "    {}{}",
            json_string(entry),
            if index + 1 == model.decision_log_entries.len() {
                ""
            } else {
                ","
            }
        )
        .expect("decision log json");
    }
    output.push_str("  ],\n");
    output.push_str("  \"refusal\": ");
    output.push_str(&render_refusal_json(model.refusal.as_ref()));
    output.push_str(",\n");
    output.push_str("  \"blockers\": ");
    output.push_str(&render_blockers_json(&model.blockers));
    output.push_str("\n}\n");
    output
}

fn render_budget_outcome_json(model: &RenderOutputModel) -> String {
    let mut output = String::from("{\n");
    write_line(
        &mut output,
        2,
        "\"disposition\":",
        &json_string(render_budget_disposition(model.budget_outcome.disposition)),
        true,
    );
    write_line(
        &mut output,
        2,
        "\"reason\":",
        &json_string(render_budget_reason(&model.budget_outcome.reason)),
        true,
    );
    output.push_str("    \"targets\": [\n");
    for (index, target) in model.budget_outcome.targets.iter().enumerate() {
        write!(
            &mut output,
            "      {{\n        \"canonical_repo_relative_path\": {},\n        \"byte_len\": {}\n      }}{}\n",
            json_string(target.canonical_repo_relative_path),
            target.byte_len,
            if index + 1 == model.budget_outcome.targets.len() {
                ""
            } else {
                ","
            }
        )
        .expect("budget target json");
    }
    output.push_str("    ],\n");
    output.push_str("    \"next_safe_action\": ");
    match render_budget_next_safe_action(model.budget_outcome.next_safe_action.as_ref()) {
        Some(action) => output.push_str(&json_string(&action)),
        None => output.push_str("null"),
    }
    output.push_str("\n  }");
    output
}

fn render_refusal_json(refusal: Option<&Refusal>) -> String {
    match refusal {
        Some(refusal) => {
            let mut output = String::from("{\n");
            write_line(
                &mut output,
                2,
                "\"category\":",
                &json_string(render_refusal_category(refusal.category)),
                true,
            );
            write_line(
                &mut output,
                2,
                "\"summary\":",
                &json_string(&refusal.summary),
                true,
            );
            output.push_str("    \"broken_subject\": ");
            output.push_str(&render_subject_json(&refusal.broken_subject));
            output.push_str(",\n");
            output.push_str("    \"next_safe_action\": ");
            output.push_str(&json_string(&super::shared::render_next_safe_action_value(
                &refusal.next_safe_action,
            )));
            output.push_str("\n  }");
            output
        }
        None => "null".to_string(),
    }
}

fn render_blockers_json(blockers: &[Blocker]) -> String {
    let mut output = String::from("[\n");
    for (index, blocker) in blockers.iter().enumerate() {
        write!(
            &mut output,
            "    {{\n      \"category\": {},\n      \"summary\": {},\n      \"subject\": {},\n      \"next_safe_action\": {}\n    }}{}\n",
            json_string(render_blocker_category(blocker.category)),
            json_string(&blocker.summary),
            render_subject_json(&blocker.subject),
            json_string(&super::shared::render_next_safe_action_value(&blocker.next_safe_action)),
            if index + 1 == blockers.len() { "" } else { "," }
        )
        .expect("blockers json");
    }
    output.push_str("  ]");
    output
}

fn render_subject_json(subject: &SubjectRef) -> String {
    match subject {
        SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        } => format!(
            "{{\n      \"kind\": {},\n      \"canonical_repo_relative_path\": {}\n    }}",
            json_string(render_canonical_artifact_kind(*kind)),
            json_string(canonical_repo_relative_path)
        ),
        SubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => format!(
            "{{\n      \"kind\": \"InheritedDependency\",\n      \"dependency_id\": {},\n      \"version\": {}\n    }}",
            json_string(dependency_id),
            match version {
                Some(version) => json_string(version),
                None => "null".to_string(),
            }
        ),
        SubjectRef::Policy { policy_id } => format!(
            "{{\n      \"kind\": \"Policy\",\n      \"policy_id\": {}\n    }}",
            json_string(policy_id)
        ),
    }
}

fn write_line(output: &mut String, indent: usize, key: &str, value: &str, trailing_comma: bool) {
    for _ in 0..indent {
        output.push_str("  ");
    }
    output.push_str(key);
    output.push(' ');
    output.push_str(value);
    if trailing_comma {
        output.push(',');
    }
    output.push('\n');
}
