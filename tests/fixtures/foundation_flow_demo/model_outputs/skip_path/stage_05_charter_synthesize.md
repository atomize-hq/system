# Project Charter
- Charter Ref: `CH-M4-FOUNDATION-SKIP-2026-04`
- Project: System
- Owner: Platform Foundations
- Team: Build Systems

## Mission
Ship the M4 skip-path proof for `pipeline.foundation_inputs` so operators can see a truthful
route where the charter already contains the planning context required to proceed directly from
stage 05 to stage 07 and then to the stage-10 external-model handoff.

## Baseline Posture
- Baseline level: 4 - High rigor with delivery pragmatism
- Speed vs Quality: prefer correctness over speed whenever route state and docs disagree
- Type safety / static analysis: strict for compiler-owned and CLI-owned route contracts
- Testing rigor: deterministic fixture corpus plus focused integration tests for happy and skip paths
- Scalability & performance: optimize for local CLI repeatability, not distributed orchestration
- Reliability & operability: refuse stale route basis and keep skip reasons explicit
- Security & privacy: fixtures and captured outputs remain secret-free and local
- Observability: prove the skipped stage, the false predicates, and the unchanged stage-10 boundary
- DX & automation: keep the operator surface small, explicit, and aligned with docs/help

## Planning Context Already Established
- Users are engineers maintaining the Rust compiler and CLI proof surfaces.
- The route is local-only, repo-scoped, and requires no external service integration.
- Data handling is limited to committed text fixtures and route-state files in temp repos.
- Backward compatibility risk is limited to tests and docs because no new product surface is added.
- Rollout is a single fixture/test/doc packet with deterministic rerun expectations.
- Manual decisions that remain after M4 are limited to future M5 workflow scope, not this skip proof.

## Red Lines
- Do not activate stage 06 when both `needs_project_context=false` and `charter_gaps_detected=false`.
- Do not teach or prove a raw `compile | capture` stage-10 handoff.
- Do not merge the M4 demo corpus into other fixture surfaces.

## Success Criteria
- The skip path proves stage 06 is skipped for the correct explicit reason.
- Stage 07 still produces the foundation pack needed for stage 10 compile context.
- Stage 10 capture writes a completed external `FEATURE_SPEC.md`, not compile payload.
