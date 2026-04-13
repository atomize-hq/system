# Project Charter
- Charter Ref: `CH-FOUNDATION-2026-04`
- Project: System
- Owner: Platform Foundations
- Team: Build Systems

## Mission
Ship a deterministic planning compiler and CLI that replace the legacy harness with a
small, auditable Rust surface. Every supported command must either produce a truthful
result or refuse with compact proof that explains why the command is unsafe to honor.

## Baseline Posture
- Baseline level: 4 - High rigor with delivery pragmatism
- Speed vs Quality: prefer correctness over speed when contracts or evidence disagree
- Type safety / static analysis: strict for compiler-owned modules
- Testing rigor: shared proof corpus plus focused unit/integration coverage
- Scalability & performance: optimize for predictable local CLI latency, not distributed scale
- Reliability & operability: refusal-first behavior for stale or malformed state
- Security & privacy: no secret capture in fixtures, logs, or proof surfaces
- Observability: proof output must explain route basis, stage selection, and missing inputs
- DX & automation: operator surface should stay small and consistent with docs/help

## Red Lines
- Do not claim success without repo-truth evidence.
- Do not silently mutate route state during compile.
- Do not expose unsupported CLI flags or alternate output formats in `M2`.
- Do not fork separate proof corpora for CLI and compiler suites.

## Success Criteria
- Canonical pipeline declarations match the shipped compile target.
- Shared fixtures contain the exact compile-time dependencies referenced by stage metadata.
- Refusals for missing required inputs, inactive stages, and stale route basis are regression-locked.
- Future stages can extend the compiler without weakening the proof-first operator contract.
