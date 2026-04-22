use system_compiler::{
    doctor, setup_starter_template_bytes, BlockerCategory, CanonicalArtifactKind,
    DoctorArtifactStatus, DoctorBaselineStatus, NextSafeAction, SubjectRef, C03_SCHEMA_VERSION,
    C04_RESULT_VERSION, MANIFEST_GENERATION_VERSION,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn valid_charter_markdown() -> &'static str {
    "# Engineering Charter — System

## What this is
Body.

## How to use this charter
Use it.

## Rubric: 1–5 rigor levels
Levels.

## Project baseline posture
Baseline.

## Domains / areas (optional overrides)
None.

## Posture at a glance (quick scan)
Snapshot.

## Dimensions (details + guardrails)
Details.

## Cross-cutting red lines (global non-negotiables)
- Keep trust boundaries intact.

## Exceptions / overrides process
- **Approvers:** project_owner
- **Record location:** docs/exceptions.md
- **Minimum required fields:**
  - what
  - why
  - scope
  - risk
  - owner
  - expiry_or_revisit_date

## Debt tracking expectations
Tracked in issues.

## Decision Records (ADRs): how to use this charter
Use ADRs.

## Review & updates
Review monthly.
"
}

fn valid_project_context_markdown() -> &'static str {
    "# Project Context — System

> **File:** `PROJECT_CONTEXT.md`
> **Created (UTC):** 2026-04-21T00:00:00Z
> **Owner:** project-owner
> **Team:** system-team
> **Repo / Project:** /tmp/system
> **Charter Ref:** .system/charter/CHARTER.md

## What this is
Project reality.

## How to use this
Use this document to ground planning in reality.

## 0) Project Summary (factual, 3–6 bullets)
- Summary.

## 1) Operational Reality (the most important section)
- Operations.

## 2) Project Classification Implications (planning guardrails)
- Guardrails.

## 3) System Boundaries (what we own vs integrate with)
### What we own
- Canonical `.system/` truth.
### What we do NOT own (but may depend on)
- External delivery systems.

## 4) Integrations & Contracts (top 1–5)
- Integrations.

## 5) Environments & Delivery
- Delivery.

## 6) Data Reality
- Data.

## 7) Repo / Codebase Reality (brownfield-friendly, but safe for greenfield)
- Codebase.

## 8) Constraints
- Constraints.

## 9) Known Unknowns (explicitly tracked)
- Unknowns.

## 10) Update Triggers
- Update when reality changes.
"
}

fn valid_environment_inventory_markdown() -> &'static str {
    "# Environment Inventory

> **Canonical File:** `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
> **Project Context Ref:** `.system/project_context/PROJECT_CONTEXT.md`

## What this is
Canonical environment and runtime inventory.

## How to use
- Update this file when runtime assumptions change.

## 1) Environment Variables (Inventory)
- None yet.

## 2) External Services / Infrastructure Dependencies
- None yet.

## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)
- None yet.

## 4) Local Development Requirements
- None yet.

## 5) CI Requirements
- None yet.

## 6) Production / Deployment Requirements (even if not live yet)
- None yet.

## 7) Dependency & Tooling Inventory (project-specific)
- None yet.

## 8) Update Contract (non-negotiable)
- Update `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.

## 9) Known Unknowns
- None yet.
"
}

fn expected_artifact_label(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "CHARTER",
        CanonicalArtifactKind::ProjectContext => "PROJECT_CONTEXT",
        CanonicalArtifactKind::EnvironmentInventory => "ENVIRONMENT_INVENTORY",
        CanonicalArtifactKind::FeatureSpec => "FEATURE_SPEC",
    }
}

fn expected_author_command(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "run `system author charter`",
        CanonicalArtifactKind::ProjectContext => "run `system author project-context`",
        CanonicalArtifactKind::EnvironmentInventory => "run `system author environment-inventory`",
        CanonicalArtifactKind::FeatureSpec => {
            "fill canonical artifact at .system/feature_spec/FEATURE_SPEC.md"
        }
    }
}

fn assert_checklist_contract_fields(
    item: &system_compiler::DoctorChecklistItem,
    kind: CanonicalArtifactKind,
    canonical_repo_relative_path: &str,
) {
    assert_eq!(item.artifact_label, expected_artifact_label(kind));
    assert_eq!(item.author_command, expected_author_command(kind));
    match &item.subject {
        SubjectRef::CanonicalArtifact {
            kind: actual_kind,
            canonical_repo_relative_path: actual_path,
        } => {
            assert_eq!(*actual_kind, kind);
            assert_eq!(*actual_path, canonical_repo_relative_path);
        }
        other => panic!("expected canonical artifact subject, got {other:?}"),
    }
}

fn assert_empty_baseline_invalid(
    empty_path: &str,
    empty_kind: CanonicalArtifactKind,
    expected_action: NextSafeAction,
) {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".system/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );
    write_file(&repo_root.join(empty_path), b"");

    let report = doctor(repo_root).expect("doctor");
    assert_eq!(report.c04_result_version, C04_RESULT_VERSION);
    assert_eq!(report.c03_schema_version, C03_SCHEMA_VERSION);
    assert_eq!(
        report.c03_manifest_generation_version,
        MANIFEST_GENERATION_VERSION
    );
    assert_eq!(report.baseline_state, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(report.status, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(report.next_safe_action, Some(expected_action.clone()));

    let item = report
        .checklist
        .iter()
        .find(|item| item.kind == empty_kind)
        .expect("empty artifact");
    assert_checklist_contract_fields(item, empty_kind, empty_path);
    assert_eq!(item.status, DoctorArtifactStatus::Empty);
    assert_eq!(item.next_safe_action, Some(expected_action));
    assert_eq!(report.blockers.len(), 1);
    assert_eq!(
        report.blockers[0].category,
        BlockerCategory::RequiredArtifactEmpty
    );
    match &report.blockers[0].subject {
        SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        } => {
            assert_eq!(*kind, empty_kind);
            assert_eq!(*canonical_repo_relative_path, empty_path);
        }
        other => panic!("expected canonical artifact subject, got {other:?}"),
    }
}

#[test]
fn doctor_marks_only_project_context_invalid_for_matching_directory_ingest_issue() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    std::fs::create_dir_all(repo_root.join(".system/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context dir");
    write_file(
        &repo_root.join(".system/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );

    let report = doctor(repo_root).expect("doctor");
    assert_eq!(report.baseline_state, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(report.status, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(
        report.next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );
    assert_eq!(report.checklist.len(), 3);

    assert_eq!(report.checklist[0].kind, CanonicalArtifactKind::Charter);
    assert_checklist_contract_fields(
        &report.checklist[0],
        CanonicalArtifactKind::Charter,
        ".system/charter/CHARTER.md",
    );
    assert_eq!(
        report.checklist[0].status,
        DoctorArtifactStatus::ValidCanonicalTruth
    );
    assert_eq!(report.checklist[0].next_safe_action, None);

    assert_eq!(
        report.checklist[1].kind,
        CanonicalArtifactKind::ProjectContext
    );
    assert_checklist_contract_fields(
        &report.checklist[1],
        CanonicalArtifactKind::ProjectContext,
        ".system/project_context/PROJECT_CONTEXT.md",
    );
    assert_eq!(report.checklist[1].status, DoctorArtifactStatus::Invalid);
    assert_eq!(
        report.checklist[1].next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );

    assert_eq!(
        report.checklist[2].kind,
        CanonicalArtifactKind::EnvironmentInventory
    );
    assert_checklist_contract_fields(
        &report.checklist[2],
        CanonicalArtifactKind::EnvironmentInventory,
        ".system/environment_inventory/ENVIRONMENT_INVENTORY.md",
    );
    assert_eq!(
        report.checklist[2].status,
        DoctorArtifactStatus::ValidCanonicalTruth
    );
    assert_eq!(report.checklist[2].next_safe_action, None);
    assert_eq!(report.blockers.len(), 1);
    assert_eq!(
        report.blockers[0].category,
        BlockerCategory::ArtifactReadError
    );
    assert_eq!(
        report.blockers[0].subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::ProjectContext,
            canonical_repo_relative_path: ".system/project_context/PROJECT_CONTEXT.md",
        }
    );
    assert_eq!(
        report.blockers[0].summary,
        "failed to read canonical artifact"
    );
}

#[cfg(unix)]
#[test]
fn doctor_marks_only_environment_inventory_invalid_for_matching_symlink_ingest_issue() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    std::fs::create_dir_all(repo_root.join(".system/environment_inventory")).expect("mkdirs");
    let redirected = repo_root.join("redirected_environment_inventory.md");
    write_file(
        &redirected,
        valid_environment_inventory_markdown().as_bytes(),
    );
    symlink(
        &redirected,
        repo_root.join(".system/environment_inventory/ENVIRONMENT_INVENTORY.md"),
    )
    .expect("symlink environment inventory");

    let report = doctor(repo_root).expect("doctor");
    assert_eq!(report.baseline_state, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(report.status, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(
        report.next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );

    assert_eq!(
        report
            .checklist
            .iter()
            .find(|item| item.kind == CanonicalArtifactKind::Charter)
            .expect("charter")
            .status,
        DoctorArtifactStatus::ValidCanonicalTruth
    );
    let environment_inventory = report
        .checklist
        .iter()
        .find(|item| item.kind == CanonicalArtifactKind::EnvironmentInventory)
        .expect("environment inventory");
    assert_checklist_contract_fields(
        environment_inventory,
        CanonicalArtifactKind::EnvironmentInventory,
        ".system/environment_inventory/ENVIRONMENT_INVENTORY.md",
    );
    assert_eq!(environment_inventory.status, DoctorArtifactStatus::Invalid);
    assert_eq!(
        environment_inventory.next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );
    assert_eq!(report.blockers.len(), 1);
    assert_eq!(
        report.blockers[0].category,
        BlockerCategory::ArtifactReadError
    );
}

#[test]
fn doctor_treats_empty_charter_as_invalid_baseline() {
    assert_empty_baseline_invalid(
        ".system/charter/CHARTER.md",
        CanonicalArtifactKind::Charter,
        NextSafeAction::RunAuthorCharter,
    );
}

#[test]
fn doctor_treats_empty_project_context_as_invalid_baseline() {
    assert_empty_baseline_invalid(
        ".system/project_context/PROJECT_CONTEXT.md",
        CanonicalArtifactKind::ProjectContext,
        NextSafeAction::RunAuthorProjectContext,
    );
}

#[test]
fn doctor_treats_empty_environment_inventory_as_invalid_baseline() {
    assert_empty_baseline_invalid(
        ".system/environment_inventory/ENVIRONMENT_INVENTORY.md",
        CanonicalArtifactKind::EnvironmentInventory,
        NextSafeAction::RunAuthorEnvironmentInventory,
    );
}

#[test]
fn doctor_keeps_all_starter_owned_baseline_in_scaffolded() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter),
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::ProjectContext),
    );
    write_file(
        &repo_root.join(".system/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::EnvironmentInventory),
    );

    let report = doctor(repo_root).expect("doctor");
    assert_eq!(report.baseline_state, DoctorBaselineStatus::Scaffolded);
    assert_eq!(report.status, DoctorBaselineStatus::Scaffolded);
    assert_eq!(
        report.next_safe_action,
        Some(NextSafeAction::RunAuthorCharter)
    );
    assert!(report
        .checklist
        .iter()
        .all(|item| item.status == DoctorArtifactStatus::StarterOwned));
    assert_eq!(report.blockers.len(), 3);
    assert_eq!(
        report
            .blockers
            .iter()
            .map(|blocker| blocker.category)
            .collect::<Vec<_>>(),
        vec![
            BlockerCategory::RequiredArtifactStarterTemplate,
            BlockerCategory::RequiredArtifactStarterTemplate,
            BlockerCategory::RequiredArtifactStarterTemplate,
        ]
    );
}

#[test]
fn doctor_reports_root_missing_blocker_before_checklist_actions() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let report = doctor(repo_root).expect("doctor");

    assert_eq!(report.c04_result_version, C04_RESULT_VERSION);
    assert_eq!(report.c03_schema_version, C03_SCHEMA_VERSION);
    assert_eq!(
        report.c03_manifest_generation_version,
        MANIFEST_GENERATION_VERSION
    );
    assert_eq!(report.baseline_state, DoctorBaselineStatus::Scaffolded);
    assert_eq!(report.status, DoctorBaselineStatus::Scaffolded);
    assert_eq!(report.next_safe_action, Some(NextSafeAction::RunSetup));
    assert_eq!(report.blockers.len(), 1);
    assert_eq!(
        report.blockers[0].category,
        BlockerCategory::SystemRootMissing
    );
    assert_eq!(
        report.blockers[0].subject,
        SubjectRef::Policy {
            policy_id: "system_root"
        }
    );
}

#[test]
fn doctor_reports_complete_baseline_with_empty_blockers() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".system/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );

    let report = doctor(repo_root).expect("doctor");

    assert_eq!(report.c04_result_version, C04_RESULT_VERSION);
    assert_eq!(report.c03_schema_version, C03_SCHEMA_VERSION);
    assert_eq!(
        report.c03_manifest_generation_version,
        MANIFEST_GENERATION_VERSION
    );
    assert_eq!(
        report.baseline_state,
        DoctorBaselineStatus::BaselineComplete
    );
    assert_eq!(report.status, DoctorBaselineStatus::BaselineComplete);
    assert!(report.blockers.is_empty());
    assert_eq!(report.next_safe_action, None);
}
