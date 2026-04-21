use system_compiler::{
    doctor, CanonicalArtifactKind, DoctorArtifactStatus, DoctorBaselineStatus, NextSafeAction,
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
    assert_eq!(report.status, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(
        report.next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );
    assert_eq!(report.checklist.len(), 3);

    assert_eq!(report.checklist[0].kind, CanonicalArtifactKind::Charter);
    assert_eq!(
        report.checklist[0].status,
        DoctorArtifactStatus::ValidCanonicalTruth
    );
    assert_eq!(report.checklist[0].next_safe_action, None);

    assert_eq!(
        report.checklist[1].kind,
        CanonicalArtifactKind::ProjectContext
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
    assert_eq!(
        report.checklist[2].status,
        DoctorArtifactStatus::ValidCanonicalTruth
    );
    assert_eq!(report.checklist[2].next_safe_action, None);
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
    assert_eq!(environment_inventory.status, DoctorArtifactStatus::Invalid);
    assert_eq!(
        environment_inventory.next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );
}
