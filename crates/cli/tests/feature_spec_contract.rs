use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let start = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in start.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if !cargo_toml.is_file() {
            continue;
        }
        let Ok(contents) = fs::read_to_string(&cargo_toml) else {
            continue;
        };
        if contents.contains("[workspace]") {
            return ancestor.to_path_buf();
        }
    }

    panic!(
        "failed to locate workspace root from CARGO_MANIFEST_DIR={}",
        env!("CARGO_MANIFEST_DIR")
    );
}

fn foundation_flow_demo_root() -> PathBuf {
    workspace_root().join("tests/fixtures/foundation_flow_demo")
}

fn read_demo_repo_file(relative_path: &str) -> String {
    fs::read_to_string(foundation_flow_demo_root().join("repo").join(relative_path))
        .unwrap_or_else(|err| panic!("read demo repo file {relative_path}: {err}"))
}

fn read_demo_expected(case: &str) -> String {
    fs::read_to_string(
        foundation_flow_demo_root()
            .join("expected")
            .join(case)
            .join("final_feature_spec.md"),
    )
    .unwrap_or_else(|err| panic!("read expected feature spec for {case}: {err}"))
}

fn section_body(document: &str, heading: &str) -> String {
    let lines = document.lines().collect::<Vec<_>>();
    let start = lines
        .iter()
        .position(|line| *line == heading)
        .unwrap_or_else(|| panic!("missing heading `{heading}`"));
    let level = heading.chars().take_while(|ch| *ch == '#').count();
    let end = lines[start + 1..]
        .iter()
        .position(|line| {
            let trimmed = line.trim_start();
            if !trimmed.starts_with('#') {
                return false;
            }
            trimmed.chars().take_while(|ch| *ch == '#').count() <= level
        })
        .map(|offset| start + 1 + offset)
        .unwrap_or(lines.len());
    lines[start + 1..end].join("\n")
}

fn ids_from_prefixed_bullets(section: &str, prefix: &str) -> BTreeSet<String> {
    section
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed
                .strip_prefix("- ")
                .filter(|entry| entry.starts_with(prefix))
                .map(|entry| {
                    entry
                        .split([':', ' '])
                        .next()
                        .expect("prefixed entry id")
                        .to_string()
                })
        })
        .collect()
}

fn extract_indented_bullets_after(section: &str, marker: &str) -> Vec<String> {
    let mut bullets = Vec::new();
    let mut in_block = false;

    for line in section.lines() {
        if !in_block {
            if line.trim() == marker {
                in_block = true;
            }
            continue;
        }

        if line.starts_with("  - ") {
            bullets.push(line.trim().trim_start_matches("- ").trim().to_string());
            continue;
        }

        if !line.trim().is_empty() {
            break;
        }
    }

    bullets
}

fn assert_feature_spec_matches_contract(feature_spec: &str, template: &str, directive: &str) {
    let required_headings = template
        .lines()
        .filter(|line| line.starts_with("## "))
        .collect::<Vec<_>>();
    for heading in required_headings {
        assert!(
            feature_spec.contains(heading),
            "feature spec missing template heading `{heading}`"
        );
    }

    let goals_section = section_body(feature_spec, "## 3) Goals");
    let goals = ids_from_prefixed_bullets(&goals_section, "G");
    assert!(!goals.is_empty(), "feature spec must declare goals");

    let ac_section = section_body(feature_spec, "## 8) Acceptance Criteria (testable)");
    let acceptance_criteria = ids_from_prefixed_bullets(&ac_section, "AC-");
    assert!(
        !acceptance_criteria.is_empty(),
        "feature spec must declare acceptance criteria"
    );
    for line in ac_section.lines() {
        let trimmed = line.trim_start();
        if let Some(entry) = trimmed.strip_prefix("- AC-") {
            assert!(
                entry.contains(':'),
                "acceptance criterion must include an objective label: {trimmed}"
            );
            assert!(
                !trimmed.contains('\u{2026}'),
                "acceptance criterion must not keep template ellipsis: {trimmed}"
            );
        }
    }

    let traceability_section = section_body(feature_spec, "## 15) Traceability Map");
    for goal in &goals {
        assert!(
            traceability_section.contains(&format!("{goal} -> AC-")),
            "goal `{goal}` must map to at least one acceptance criterion"
        );
    }
    let requirement_lines = traceability_section
        .lines()
        .filter(|line| line.trim_start().starts_with("- R"))
        .collect::<Vec<_>>();
    assert!(
        !requirement_lines.is_empty(),
        "traceability map must include requirement mappings"
    );

    assert!(
        directive.contains("At least one alternative is documented."),
        "directive self-check must require alternatives"
    );
    let alternatives_section = section_body(feature_spec, "## 10) Alternatives Considered");
    let alternative_count = alternatives_section
        .lines()
        .filter(|line| line.trim_start().starts_with("- Alt "))
        .count();
    assert!(
        alternative_count >= 1,
        "feature spec must document at least one alternative"
    );

    assert!(
        directive.contains("NFRs include security + performance + reliability"),
        "directive self-check must require core NFR coverage"
    );
    let nfr_section = section_body(feature_spec, "### Non-Functional Requirements (NFRs)");
    for prefix in ["- Security", "- Performance", "- Reliability"] {
        let matching_line = nfr_section
            .lines()
            .find(|line| line.trim_start().starts_with(prefix))
            .unwrap_or_else(|| panic!("feature spec missing required NFR line `{prefix}`"));
        assert!(
            matching_line.contains(':'),
            "required NFR line must include content: {matching_line}"
        );
    }

    assert!(
        directive.contains("Integration touchpoints are named"),
        "directive self-check must require named integration touchpoints"
    );
    let proposed_approach_section =
        section_body(feature_spec, "### Proposed Approach (recommended)");
    let touchpoints = extract_indented_bullets_after(
        &proposed_approach_section,
        "- Integration touchpoints (files/modules/services):",
    );
    assert!(
        !touchpoints.is_empty(),
        "feature spec must name at least one integration touchpoint"
    );
    for touchpoint in touchpoints {
        assert!(
            !touchpoint.ends_with(':'),
            "integration touchpoint must be named or marked TBD: {touchpoint}"
        );
    }
}

#[test]
fn foundation_flow_demo_feature_specs_match_directive_and_template_contract() {
    let directive =
        read_demo_repo_file("core/library/feature_spec/feature_spec_architect_directive.md");
    let template = read_demo_repo_file("core/library/feature_spec/FEATURE_SPEC.md.tmpl");

    for case in ["happy_path", "skip_path"] {
        let feature_spec = read_demo_expected(case);
        assert_feature_spec_matches_contract(&feature_spec, &template, &directive);
    }
}
