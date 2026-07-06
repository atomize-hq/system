# Tasks: Handbook Published-Import Decoupling — Set 3: Published Consumer Proof + Substrate Proof + Guard Rails

Spec reference: [handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md](./handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md)
Plan reference: [handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md](./handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md)

## Active authority route for this seam

Use the following order during Set 3 execution:

1. `docs/specs/MAP.md` for exact objective, exact intent, success criteria, set sequencing, and the downstream worktree rule
2. the active Set 1 triplet for the Set 3 proof/guard-rail handoff wall
3. the active Set 2 triplet for the retained/dropped public boundary matrix and packaged-proof closeout line
4. this active Set 3 triplet for released-proof, downstream-proof, and guard-rail implementation scope
5. `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md` for current-state evidence only
6. `docs/specs/archive/` for provenance only, including the archived older Set 3 parameterization docs
7. the current Packet 4.2 downstream worktree and notes for baseline context only

If those sources disagree, the MAP plus the active Set 1 triplet plus the active Set 2 triplet plus this Set 3 triplet control active execution.

---

## Packet 3.1: Released-Proof Harness + Release Preparation

- [x] Task: Add an exact-published-version external consumer proof harness for `handbook-pipeline`
  - Acceptance: A new proof script plus fixture consume `handbook-pipeline` through an exact crates.io version pin, run outside the source tree, exercise retained public APIs only, and fail if proof falls back to sibling-path or source-tree dependency accidents.
  - Verify: source inspection of the proof script + fixture; `cargo package -p handbook-pipeline --allow-dirty`; `cargo publish --dry-run -p handbook-pipeline`
  - Files: `tools/proof/handbook_pipeline_released_boundary.sh`, `tests/fixtures/external_consumers/handbook_pipeline_released_boundary/Cargo.toml`, `tests/fixtures/external_consumers/handbook_pipeline_released_boundary/src/main.rs`

- [x] Task: Record the smallest honest publish target for the Set 2 boundary
  - Acceptance: The active Set 3 docs explicitly state which crate version(s) must be published for Set 3, why `handbook-pipeline 0.1.1` is insufficient, and whether the publish is `handbook-pipeline` only or a coordinated version train.
  - Verify: source inspection of the Set 3 triplet; `rg -n "0\.1\.1|publish target|exact published version|coordinated version" docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md`

### Packet 3.1 closeout note (2026-06-23)

- The released proof tier now lives at `tools/proof/handbook_pipeline_released_boundary.sh` plus `tests/fixtures/external_consumers/handbook_pipeline_released_boundary/`, separate from the Set 2 packaged proof tier.
- The released fixture pins `handbook-pipeline` with an exact crates.io version only, and the harness rejects path-dependency or source-tree fallback before any proof run can count.
- `handbook-pipeline 0.1.1` is insufficient because crates.io already contains `0.1.1` while the retained Set 2 boundary being proved is present only in the current unpublished source/package.
- The smallest honest publish target for Packet 3.2 is `handbook-pipeline 0.1.2` only; Packet 3.1 found no evidence that `handbook-engine` or `handbook-flow` need a coordinated version train.

---

## Packet 3.2: Published Crates.io Proof

- [x] Task: Publish the first crates.io version that actually contains the Set 2 `handbook-pipeline` boundary
  - Acceptance: The selected crate version is published to crates.io, `cargo publish --dry-run -p handbook-pipeline` passes first, and the released version is the one the proof harness will consume exactly.
  - Verify: `cargo publish --dry-run -p handbook-pipeline`; actual `cargo publish -p handbook-pipeline` run after approval; optional `cargo search handbook-pipeline --limit 5`
  - Files: `crates/pipeline/Cargo.toml`, any tightly required version-coupled workspace manifests only if needed for the publish target, and the active Set 3 docs if they must record the final published version

- [x] Task: Prove the released external consumer passes against the exact published version
  - Acceptance: The released external consumer proof passes against the real published version, constructs non-default contracts through public APIs only, and exercises the retained capability families without private-module imports.
  - Verify: `bash tools/proof/handbook_pipeline_released_boundary.sh --version <published_version>`
  - Files: `tools/proof/handbook_pipeline_released_boundary.sh`, `tests/fixtures/external_consumers/handbook_pipeline_released_boundary/Cargo.toml`, `tests/fixtures/external_consumers/handbook_pipeline_released_boundary/src/main.rs`, optionally a small README under the same proof directory if needed for exact-version usage

- [x] Task: Record honest released-proof classification and preserve Packet 4.2 boundaries
  - Acceptance: Active docs clearly distinguish Set 2 packaged proof from Set 3 released proof, and explicitly keep Packet 4.2 classified only as `engine + flow` proof.
  - Verify: `rg -n "packaged proof|released proof|engine \+ flow|Packet 4\.2" docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-*.md docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md`

### Packet 3.2 closeout status (2026-06-23)

- `cargo publish -p handbook-pipeline --allow-dirty` published `handbook-pipeline 0.1.2` to crates.io.
- `bash tools/proof/handbook_pipeline_released_boundary.sh --version 0.1.2` passed against the published registry artifact.
- Packet 3.2 now closes with released proof complete while keeping Set 2 packaged proof distinct and Packet 4.2 classified only as `engine + flow` proof.

---

## Packet 3.3: Downstream Substrate Published-Boundary Proof

- [x] Task: Choose one narrow downstream Substrate proof seam in a dedicated worktree and pin the exact published version
  - Acceptance: The downstream proof work happens only in a dedicated worktree under `/Users/spensermcconnell/.codex/worktrees/`, the main Substrate checkout remains untouched, and the worktree resolves the exact published `handbook-pipeline` version proved in Packet 3.2.
  - Verify: `git status --short --branch`; `cargo tree -p handbook-pipeline`; `cargo tree -p handbook-engine`; `cargo tree -p handbook-flow`
  - Files: the dedicated Substrate worktree `Cargo.toml` / manifest surfaces needed to pin the published version, plus the active Set 3 docs if they must record the selected seam/worktree path

- [x] Task: Implement only the smallest downstream seam needed to prove real `handbook-pipeline` capability
  - Acceptance: The chosen Substrate seam uses published `handbook-pipeline` public APIs only, does not rely on sibling-path fallback or private internals, and keeps final wording/runtime behavior owned by Substrate.
  - Verify: `cargo check --workspace` in the dedicated Substrate worktree, plus the targeted downstream proof command(s) selected for the seam, including an execution of the positive handbook-backed path rather than only the no-handbook branch
  - Files: one narrow downstream file group in the dedicated worktree, ideally no more than ~5 files, plus tightly related tests if needed for honest proof

- [x] Task: Record an explicit downstream capability map instead of overclaiming full family usage
  - Acceptance: Active docs state exactly which retained `handbook-pipeline` capability families the chosen Substrate seam uses now, which families remain externally proved but unused downstream, and why that still satisfies the MAP objective for Substrate's actual needs.
  - Verify: source inspection of the Set 3 triplet; `rg -n "capability map|externally proved|unused downstream|actual needs|Substrate-owned" docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md`

### Packet 3.3 closeout status (2026-06-23)

- Dedicated downstream worktree used: `/Users/spensermcconnell/.codex/worktrees/substrate-packet-3-3-20260623-213135` on branch `packet-3-3-20260623-213135`.
- Exact downstream proof seam chosen: `/Users/spensermcconnell/.codex/worktrees/substrate-packet-3-3-20260623-213135/crates/shell/src/execution/prompt_fulfillment.rs`
  - the Substrate-owned host-toolbox prompt composer contains the production handbook-backed advisory path; Packet 3.3 proves that path through targeted downstream tests rather than by claiming the checked-out worktree itself is handbook-backed
- Exact version pin result:
  - root worktree manifest now pins `handbook-pipeline = "=0.1.2"`
  - `cargo tree -p handbook-pipeline` resolves `handbook-pipeline v0.1.2`
  - `cargo tree -p handbook-engine` and `cargo tree -p handbook-flow` remain at `v0.1.1`
  - the worktree manifests contain no `[patch.crates-io]` override or sibling-path dependency for handbook crates
- Targeted downstream proof runs:
  - the checked-out worktree contains no `.handbook`, so ambient runtime in that checkout still takes the non-handbook branch
  - `cargo test -p shell compose_prompt_with_host_toolbox_contract_adds_ready_handbook_pipeline_advisory -- --nocapture` passed; it creates a temporary `.handbook/core/...` repo fixture and executes the production prompt-fulfillment seam through published `handbook-pipeline 0.1.2`
  - `cargo check --workspace` passed in the dedicated worktree
- Downstream capability map:
  - consumed now: declarative-root contract construction for `.handbook/core/...`; metadata browse; selector resolution; selected definition load
  - externally proved / unused downstream now: direct definition load by explicit repo-relative path; route-state storage-layout; capture storage-layout; handoff storage-layout
- Packet 3.3 claim boundary:
  - this packet proves a narrow Substrate planning-context seam only, under a handbook-backed repo fixture that executes the real downstream production path
  - Packet 4.2 remains `engine + flow` only
  - Packet 3.4 still owns release/update guard rails and final Set 3 closeout

---

## Packet 3.4: Guard Rails + Honest Closeout

- [x] Task: Add release/update guard rails that re-run the released-boundary proof and reject path-fallback proof
  - Acceptance: The repo has a rerunnable guard rail that verifies the released external consumer proof against the exact published version and fails if proof falls back to sibling-path or source-tree dependency accidents.
  - Verify: `bash tools/proof/handbook_pipeline_released_boundary.sh --version 0.1.2`; `just handbook_pipeline_released_proof`; `just handbook_published_import_set3_guardrails`
  - Files: `tools/proof/handbook_pipeline_released_boundary.sh`, `justfile`

- [x] Task: Add truth-classification guard rails so Packet 4.2 cannot be mistaken for `handbook-pipeline` proof
  - Acceptance: Active docs and any added verification rails explicitly distinguish `engine + flow` proof from `handbook-pipeline` proof and fail closeout if that distinction is lost.
  - Verify: `just handbook_published_import_set3_guardrails`; `rg -n "engine \+ flow|handbook-pipeline proof|Packet 4\.2|Set 2 packaged proof|Set 3 released external proof|Set 3 downstream Substrate proof" docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-*.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-*.md`
  - Files: `justfile`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md`

- [x] Task: Close Set 3 honestly against the MAP objective and exact intent
  - Acceptance: Closeout notes confirm a real published external proof passed, a real dedicated-worktree downstream Substrate proof passed, no new public surface was added outside the Set 2 matrix, Packet 4.2 remains `engine + flow` only, and the full MAP objective is now satisfied through a reviewed stable published boundary.
  - Verify: source inspection of the Set 3 triplet; `rg -n "published external proof|downstream Substrate proof|minimum unnecessary public surface|engine \+ flow|MAP objective|reviewed, stable, published boundary" docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md`, `docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md`, optionally `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md` only if a final audit addendum is required

### Packet 3.4 closeout status (2026-06-23)

- Added the rerunnable repo-local guard rails in `justfile`:
  - `just handbook_pipeline_released_proof` re-runs `bash tools/proof/handbook_pipeline_released_boundary.sh --version 0.1.2`
  - `just handbook_published_import_set3_guardrails` re-runs the same released proof and then checks that the active docs still name Set 2 packaged proof, Set 3 released external proof, Set 3 downstream Substrate proof, and Packet 4.2 as `engine + flow` only
- These rails remain honest because the underlying released-proof harness rejects path dependencies and source-tree fallback before any proof run can count.
- Final Set 3 closeout remains intentionally narrow:
  - released external proof passed against `handbook-pipeline 0.1.2`
  - downstream Substrate proof passed once, in the dedicated worktree seam `crates/shell/src/execution/prompt_fulfillment.rs`
  - no new public surface was added beyond the Set 2 retained/dropped matrix
  - Packet 4.2 remains `engine + flow` only
  - the MAP objective is satisfied through a reviewed, stable, published boundary rather than packaged-only or sibling-path-local proof

---

## Set-Level Guardrail

Stop after Set 3 lands:

- one released external consumer proof against an exact crates.io version,
- one dedicated-worktree downstream Substrate proof against that same published version,
- and the minimum release/update guard rails needed to keep those truths honest.

Do not:

- widen the public boundary beyond the Set 2 retained/dropped matrix without reopening authority first
- classify Set 2 packaged proof as if it were released-crate proof
- classify Packet 4.2 as if it were `handbook-pipeline` proof
- run downstream source-touching proof in the main Substrate checkout
- drift into broad Substrate redesign, unrelated release automation, or new handbook public API design

## Set-Level Completion Standard

Set 3 is complete only when:

- the exact published version required by Set 3 is real on crates.io,
- the released external consumer proof passes against that version,
- the dedicated-worktree downstream Substrate proof passes against that version,
- the active docs explicitly preserve the ownership split and proof classification boundaries,
- the Set 2 retained/dropped matrix still matches the live public surface,
- the guard rails can catch published-boundary drift and proof misclassification,
- and the full MAP objective is honestly satisfied without overclaiming.
