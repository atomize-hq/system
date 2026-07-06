use std::cell::RefCell;
use std::io::{self, Write};

struct GuidedPromptContext {
    object: &'static str,
    interview_name: &'static str,
    broken_subject: &'static str,
    retry_command: &'static str,
    restart_or_from_inputs: &'static str,
}

const CHARTER_PROMPT_CONTEXT: GuidedPromptContext = GuidedPromptContext {
    object: "author charter",
    interview_name: "guided charter interview",
    broken_subject: "structured charter input",
    retry_command: "repair the interactive terminal and retry `handbook author charter`",
    restart_or_from_inputs:
        "restart `handbook author charter` or use `handbook author charter --from-inputs <path|->`",
};

const PROJECT_CONTEXT_PROMPT_CONTEXT: GuidedPromptContext = GuidedPromptContext {
    object: "author project-context",
    interview_name: "guided project-context interview",
    broken_subject: "structured project-context input",
    retry_command: "repair the interactive terminal and retry `handbook author project-context`",
    restart_or_from_inputs:
        "restart `handbook author project-context` or use `handbook author project-context --from-inputs <path|->`",
};

thread_local! {
    static GUIDED_PROMPT_CONTEXT: RefCell<&'static GuidedPromptContext> =
        const { RefCell::new(&CHARTER_PROMPT_CONTEXT) };
}

pub(crate) struct GuidedPromptContextGuard {
    previous: &'static GuidedPromptContext,
}

impl GuidedPromptContextGuard {
    fn push(next: &'static GuidedPromptContext) -> Self {
        let previous = GUIDED_PROMPT_CONTEXT.with(|slot| {
            let previous = *slot.borrow();
            *slot.borrow_mut() = next;
            previous
        });
        Self { previous }
    }
}

impl Drop for GuidedPromptContextGuard {
    fn drop(&mut self) {
        GUIDED_PROMPT_CONTEXT.with(|slot| {
            *slot.borrow_mut() = self.previous;
        });
    }
}

pub(crate) fn project_context_prompt_guard() -> GuidedPromptContextGuard {
    GuidedPromptContextGuard::push(&PROJECT_CONTEXT_PROMPT_CONTEXT)
}

fn current_guided_prompt_context() -> &'static GuidedPromptContext {
    GUIDED_PROMPT_CONTEXT.with(|slot| *slot.borrow())
}

pub(crate) fn prompt_project_context_required_concrete(
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

    Err(render_project_context_interview_incomplete_refusal(
        &format!(
            "guided project-context interview could not normalize a concrete answer for {field_name}"
        ),
    ))
}

pub(crate) fn prompt_project_context_required_concrete_with_default(
    prompt: &str,
    default_value: &str,
    field_name: &str,
) -> Result<String, String> {
    loop {
        let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value.to_string());
        }
        if let Some(normalized) = normalize_required_free_text(&value) {
            return Ok(normalized);
        }
        println!("Provide a concrete answer or press enter to keep the default.");
        println!(
            "guided project-context interview needs a concrete answer for {field_name} when customizing"
        );
    }
}

pub(crate) fn prompt_project_context_csv_non_empty_concrete(
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

    Err(render_project_context_interview_incomplete_refusal(
        &format!(
            "guided project-context interview could not normalize a concrete answer for {field_name}"
        ),
    ))
}

pub(crate) fn prompt_required_concrete(
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

pub(crate) fn prompt_optional(prompt: &str) -> Result<String, String> {
    prompt_line(prompt).map(|value| normalize_free_text_answer(&value))
}

pub(crate) fn prompt_with_default(prompt: &str, default_value: &str) -> Result<String, String> {
    let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
    if value.trim().is_empty() {
        Ok(default_value.to_string())
    } else {
        Ok(normalize_free_text_answer(&value))
    }
}

pub(crate) fn prompt_bool(prompt: &str) -> Result<bool, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" | "true" => return Ok(true),
            "n" | "no" | "false" => return Ok(false),
            _ => println!("Expected yes/no."),
        }
    }
}

pub(crate) fn prompt_bool_with_default(prompt: &str, default_value: bool) -> Result<bool, String> {
    let default_label = if default_value { "yes" } else { "no" };
    loop {
        let value = prompt_line(&format!("{prompt} [yes|no] [{default_label}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value);
        }
        match value.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" | "true" => return Ok(true),
            "n" | "no" | "false" => return Ok(false),
            _ => println!("Expected yes/no."),
        }
    }
}

pub(crate) fn prompt_u32(prompt: &str) -> Result<u32, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().parse::<u32>() {
            Ok(parsed) if parsed > 0 => return Ok(parsed),
            _ => println!("Expected an integer greater than 0."),
        }
    }
}

pub(crate) fn prompt_u8_in_range(prompt: &str, min: u8, max: u8) -> Result<u8, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().parse::<u8>() {
            Ok(parsed) if (min..=max).contains(&parsed) => return Ok(parsed),
            _ => println!("Expected an integer in the allowed range."),
        }
    }
}

pub(crate) fn prompt_usize_in_range_with_default(
    prompt: &str,
    min: usize,
    max: usize,
    default_value: usize,
) -> Result<usize, String> {
    loop {
        let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value);
        }
        match value.trim().parse::<usize>() {
            Ok(parsed) if (min..=max).contains(&parsed) => return Ok(parsed),
            _ => println!("Expected an integer in the allowed range."),
        }
    }
}

pub(crate) fn prompt_u8_in_range_with_default(
    prompt: &str,
    min: u8,
    max: u8,
    default_value: u8,
) -> Result<u8, String> {
    loop {
        let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value);
        }
        match value.trim().parse::<u8>() {
            Ok(parsed) if (min..=max).contains(&parsed) => return Ok(parsed),
            _ => println!("Expected an integer in the allowed range."),
        }
    }
}

pub(crate) fn prompt_choice<T>(
    prompt: &str,
    parser: fn(&str) -> Result<T, String>,
) -> Result<T, String> {
    loop {
        let value = prompt_line(prompt)?;
        match parser(&value) {
            Ok(parsed) => return Ok(parsed),
            Err(err) => println!("{err}"),
        }
    }
}

pub(crate) fn prompt_csv_choice<T>(
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

pub(crate) fn prompt_csv_optional(prompt: &str) -> Result<Vec<String>, String> {
    let value = prompt_line(prompt)?;
    if value.trim().is_empty() {
        Ok(Vec::new())
    } else {
        split_csv_required(&value)
    }
}

pub(crate) fn prompt_csv_non_empty_concrete(
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

pub(crate) fn prompt_required_concrete_with_default(
    prompt: &str,
    default_value: &str,
    field_name: &str,
) -> Result<String, String> {
    loop {
        let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value.to_string());
        }
        if let Some(normalized) = normalize_required_free_text(&value) {
            return Ok(normalized);
        }
        println!("Provide a concrete answer or press enter to keep the baseline.");
        println!(
            "guided charter interview needs a concrete answer for {field_name} when customizing"
        );
    }
}

pub(crate) fn prompt_csv_non_empty_concrete_with_default(
    prompt: &str,
    default_value: &[String],
    field_name: &str,
) -> Result<Vec<String>, String> {
    let default_display = join_csv_default(default_value);
    loop {
        let value = prompt_line(&format!("{prompt} [{default_display}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value.to_vec());
        }
        if let Some(items) = normalize_required_csv(&value) {
            return Ok(items);
        }
        println!("Provide concrete comma-separated values or press enter to keep the baseline.");
        println!(
            "guided charter interview needs a concrete answer for {field_name} when customizing"
        );
    }
}

pub(crate) fn prompt_csv_optional_with_default(
    prompt: &str,
    default_value: &[String],
) -> Result<Vec<String>, String> {
    let default_display = join_csv_default(default_value);
    let value = prompt_line(&format!("{prompt} [{default_display}]"))?;
    if value.trim().is_empty() {
        Ok(default_value.to_vec())
    } else {
        split_csv_required(&value)
    }
}

fn prompt_line(prompt: &str) -> Result<String, String> {
    let context = current_guided_prompt_context();
    print!("{prompt}: ");
    io::stdout().flush().map_err(|err| {
        render_author_custom_refusal(
            context.object,
            "REFUSED",
            "PromptWriteFailure",
            &format!("failed to render guided prompt: {err}"),
            "interactive terminal",
            context.retry_command,
        )
    })?;

    let mut value = String::new();
    let bytes_read = io::stdin().read_line(&mut value).map_err(|err| {
        render_author_custom_refusal(
            context.object,
            "REFUSED",
            "PromptReadFailure",
            &format!("failed to read guided answer: {err}"),
            "interactive terminal",
            context.retry_command,
        )
    })?;

    if bytes_read == 0 {
        return Err(render_author_custom_refusal(
            context.object,
            "REFUSED",
            "InterviewIncomplete",
            &format!(
                "{} ended before all required answers were collected",
                context.interview_name
            ),
            context.broken_subject,
            context.restart_or_from_inputs,
        ));
    }

    Ok(value.trim().to_string())
}

pub(crate) fn split_csv_required(value: &str) -> Result<Vec<String>, String> {
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
    handbook_engine::author::normalize_charter_free_text(value)
}

fn join_csv_default(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

fn is_unusably_vague_text(value: &str) -> bool {
    handbook_engine::author::is_unusably_vague_charter_text(value)
}

fn render_author_custom_refusal(
    object: &str,
    outcome: &str,
    category: &str,
    summary: &str,
    broken_subject: &str,
    next_safe_action: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("OUTCOME: {outcome}\n"));
    out.push_str(&format!("OBJECT: {object}\n"));
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.push_str("## REFUSAL\n");
    out.push_str(&format!("CATEGORY: {category}\n"));
    out.push_str(&format!("SUMMARY: {summary}\n"));
    out.push_str(&format!("BROKEN SUBJECT: {broken_subject}\n"));
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.trim_end().to_string()
}

fn render_interview_incomplete_refusal(summary: &str) -> String {
    render_author_custom_refusal(
        "author charter",
        "REFUSED",
        "InterviewIncomplete",
        summary,
        "structured charter input",
        "restart `handbook author charter` or use `handbook author charter --from-inputs <path|->`",
    )
}

fn render_project_context_interview_incomplete_refusal(summary: &str) -> String {
    render_author_custom_refusal(
        "author project-context",
        "REFUSED",
        "InterviewIncomplete",
        summary,
        "structured project-context input",
        "restart `handbook author project-context` or use `handbook author project-context --from-inputs <path|->`",
    )
}
