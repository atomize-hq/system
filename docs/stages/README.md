# Stage Reference (Implemented)

## Legacy Scope

This reference documents stages implemented in the current Python harness scaffold.

- It is useful for understanding what exists today.
- It is **not** proof that the reviewed Rust-first v1 already supports those same flows.
- In particular, slice and execution stages are still scaffolds, not live product capability.

This directory documents the stages that are **implemented and runnable today**.

All stages are executed via the harness:
- compile → paste into LLM → capture

See [Harness](../HARNESS.md) for the mechanical workflow.

## Implemented stages

- [`stage.00_base`](stage.00_base.md) — initialize run variables and write `BASE_CONTEXT.md`
- [`stage.05_charter_interview`](stage.05_charter_interview.md) — produce `CHARTER.md`
- [`stage.06_project_context_interview`](stage.06_project_context_interview.md) — optional; produce `PROJECT_CONTEXT.md`
- [`stage.07_foundation_pack`](stage.07_foundation_pack.md) — synthesize foundation artifacts + quality gates + environment inventory
- [`stage.01_release_plan`](stage.01_release_plan.md) — define a release queue + multi-sprint intent (grounded in Work Catalog)
- [`stage.02_sprint_plan`](stage.02_sprint_plan.md) — turn release intent + prior sprint reality into a sprint plan + typed tasks
- [`stage.10_feature_spec`](stage.10_feature_spec.md) — produce a single `FEATURE_SPEC.md`

Other stage files may exist in `core/stages/`, but if they are empty placeholders they are not documented here.
