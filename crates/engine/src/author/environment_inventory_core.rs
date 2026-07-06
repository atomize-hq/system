// These literals remain handbook-product authoring defaults for engine-owned
// markdown synthesis and validation. They are intentionally code-owned and do
// not describe the reusable import-layout contract.
const CANONICAL_ENVIRONMENT_INVENTORY_PATH: &str =
    ".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md";
const LEGACY_NON_CANONICAL_PATH_CLAIMS: [&str; 3] = [
    "${repo_root}/ENVIRONMENT_INVENTORY.md",
    "artifacts/foundation/ENVIRONMENT_INVENTORY.md",
    "repo/project root",
];
const PROJECT_CONTEXT_REF_PRESENT_LINE: &str =
    "> **Project Context Ref:** `.handbook/project_context/PROJECT_CONTEXT.md`";
const PROJECT_CONTEXT_REF_ABSENT_LINE: &str = "> **Project Context Ref:** None";

pub const REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS: [&str; 11] = [
    "## What this is",
    "## How to use",
    "## 1) Environment Variables (Inventory)",
    "## 2) External Services / Infrastructure Dependencies",
    "## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)",
    "## 4) Local Development Requirements",
    "## 5) CI Requirements",
    "## 6) Production / Deployment Requirements (even if not live yet)",
    "## 7) Dependency & Tooling Inventory (project-specific)",
    "## 8) Update Contract (non-negotiable)",
    "## 9) Known Unknowns",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvironmentInventoryValidationExpectations {
    expected_project_context_ref_line: &'static str,
}

impl EnvironmentInventoryValidationExpectations {
    pub fn for_optional_project_context(has_project_context: bool) -> Self {
        Self {
            expected_project_context_ref_line: if has_project_context {
                PROJECT_CONTEXT_REF_PRESENT_LINE
            } else {
                PROJECT_CONTEXT_REF_ABSENT_LINE
            },
        }
    }

    pub fn expected_project_context_ref_line(self) -> &'static str {
        self.expected_project_context_ref_line
    }
}

pub fn validate_environment_inventory_markdown(markdown: &str) -> Result<(), String> {
    let normalized = markdown.trim();
    if normalized.is_empty() {
        return Err("synthesized environment inventory markdown was empty".to_string());
    }
    if !markdown.starts_with("# Environment Inventory") {
        return Err(
            "synthesized environment inventory markdown must start with `# Environment Inventory`"
                .to_string(),
        );
    }
    if normalized.contains("{{") || normalized.contains("}}") {
        return Err(
            "synthesized environment inventory markdown contains unresolved template placeholders"
                .to_string(),
        );
    }
    if LEGACY_NON_CANONICAL_PATH_CLAIMS
        .iter()
        .any(|claim| normalized.contains(claim))
    {
        return Err(
            "synthesized environment inventory markdown still contains legacy non-canonical path claims"
                .to_string(),
        );
    }
    if !normalized.contains(&format!("`{CANONICAL_ENVIRONMENT_INVENTORY_PATH}`")) {
        return Err(format!(
            "synthesized environment inventory markdown must reference `{CANONICAL_ENVIRONMENT_INVENTORY_PATH}` as the canonical file"
        ));
    }
    validate_required_heading_order_result(normalized, &REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS)?;
    Ok(())
}

pub fn validate_required_heading_order_result(
    markdown: &str,
    required_headings: &[&str],
) -> Result<(), String> {
    let heading_lines = markdown
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim_end();
            required_headings
                .contains(&trimmed)
                .then_some((index, trimmed))
        })
        .collect::<Vec<_>>();

    let mut previous = 0usize;
    for heading in required_headings {
        let positions = heading_lines
            .iter()
            .filter_map(|(index, line)| (*line == *heading).then_some(*index))
            .collect::<Vec<_>>();
        if positions.is_empty() {
            return Err(format!("missing required heading `{heading}`"));
        }
        if positions.len() != 1 {
            return Err(format!(
                "required heading `{heading}` must appear exactly once"
            ));
        }
        let position = positions[0];
        if position < previous {
            return Err(format!("required heading `{heading}` is out of order"));
        }
        previous = position;
    }

    Ok(())
}

pub fn validate_synthesized_environment_inventory_markdown(
    markdown: &str,
    expectations: EnvironmentInventoryValidationExpectations,
) -> Result<(), String> {
    validate_environment_inventory_markdown(markdown)?;

    let expected_project_context_ref_line = expectations.expected_project_context_ref_line();
    if !markdown.contains(expected_project_context_ref_line) {
        return Err(format!(
            "synthesized environment inventory markdown must include the exact project context reference line `{expected_project_context_ref_line}`"
        ));
    }

    Ok(())
}
