# Handbook Engine Extraction Closeout Four-Set Map

## Purpose

This file was the planning-entry map for the four closeout sets required to finish the handbook engine extraction honestly against the root objective in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`.

Those four closeout sets are now landed in live repo truth. This file should be read as the closeout-shape reference for how Phases 1 through 5 were finished, not as evidence that those phases remain open.

This file does **not** replace:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` as the root-plan and phase-order authority
- `docs/specs/handbook-engine-extraction-slice-map.md` as the existing Phase -> Slice -> Packet decomposition authority
- any already-landed slice triplets as the authority for their original slice boundaries

Instead, this file records the four closeout seams, the order they landed, the triplet stems they used, and the repo-truth assumptions that were required to close Phases 1 through 5 honestly before Phase 6.

---

## Global Repo-Truth Summary

The current repo truth is:

- the crate split is real (`engine`, `pipeline`, `flow`, `cli`, plus a narrowed `compiler`)
- the verification wall is green in the current repo (`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`)
- the four-set closeout is landed, so Phases 1 through 5 are complete against the root extraction-plan objective

The four seams that had to be closed were:

1. reusable layout/storage assumptions are still centralized but not fully parameterized
2. orchestration targets are still effectively one bounded runtime wedge
3. direct callers and compatibility ownership still lean too heavily on `handbook-compiler`
4. the CLI shell is improved but not yet the fully isolated product shell described in the plan

The residual truths that remain explicit after closeout are:

- generalized multi-consumer/customizable consumer-framework work is still deferred
- the current bounded consumer/runtime wedge is intentionally retained and code-owned
- handbook-product default layout contracts still exist as defaults, but reusable internals now consume reviewed typed contracts instead of scattered literals
- `handbook-compiler` remains a narrow compatibility/support seam rather than the implementation center
- Phase 6 in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` is now the next authoritative step
- All Phase 1–5 and Phase 6 slice/packet artifacts have been archived under `docs/specs/archive/`. See `docs/specs/archive/README.md` for the archive index. Historical file paths referenced in this map should be resolved relative to the archive subdirectories.

---

## How Future Agents Should Use This File

For future audits or regression repairs:

1. treat this file as the high-level record of the closed four-set reconciliation, not as an active backlog
2. re-read the listed live authority inputs before claiming one of the four seams regressed
3. start new forward-looking work from Phase 6 in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` unless the task is a narrow regression repair inside one landed set
4. do **not** use this file to justify widening one closed set into another

Recommended flow for a future agent:

1. read this file
2. read `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
3. read `docs/specs/handbook-engine-extraction-slice-map.md`
4. read the set-specific authority files listed below only if you are auditing or repairing that closed seam
5. if no regression exists, move on to Phase 6 authority instead of reopening Phase 1–5 planning

---

## Landed Order

The four sets landed in this order:

1. **Set 1 / Slice 1.5** — Layout / storage parameterization closeout
2. **Set 2 / Slice 2.4** — Orchestration target parameterization closeout
3. **Set 3 / Slice 4.5 refresh** — Direct caller rewires + compiler narrowing closeout
4. **Set 4 / Slice 5.3** — CLI shell closeout

Why this order:

- Set 1 and Set 2 close the two most important root-plan parameterization gaps
- Set 3 makes the crate split operationally real after those seams are clarified
- Set 4 is the final product-shell finish pass and should not be mixed into architectural remediation

---

## Set 1 — Layout / Storage Parameterization Closeout

### Triplet Status

- **Landed remediation triplet**

### Proposed Triplet Stem

- `handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout`

### Landed Files

- `docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-tasks.md`

### Seam

Finish the root-plan requirement that reusable internals are parameterized instead of merely centralized behind layout helper types.

### Why This Is Its Own Set

- separate acceptance story from orchestration-target parameterization
- touches reusable storage/layout seams
- should stay out of CLI shell cleanup and caller ownership cleanup

### Repo-Truth Problem Statement

The repo now has typed layout owners, but reusable internals still retain fixed handbook-product layout assumptions in places that the root plan expected to become parameterized.

### Root-Plan Gap Closed By This Set

- `path/storage assumptions are parameterized instead of being baked into reusable internals`

### Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- existing Phase 1 slice triplets:
  - `handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-*`
  - `handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-*`
  - `handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-*`
  - `handbook-engine-extraction-phase-1-slice-4-authoring-layout-*`
- likely live code surfaces:
  - `crates/engine/src/canonical_paths.rs`
  - `crates/pipeline/src/layout.rs`
  - any remaining reusable-internal callers that still assume fixed `.handbook/**`, `.handbook/state/**`, or `artifacts/**` roots

### Assumptions Future Agents Should Surface In `SPECIFY`

1. The goal is to finish root-plan parameterization, not to redesign the product layout model.
2. Fixed paths may remain in the CLI shell or explicitly product-owned edge surfaces if justified.
3. Reusable internals should not keep path assumptions that prevent later reuse or narrow extraction.
4. This set should avoid target-parameterization work except where layout ownership is inseparable from path semantics.

### Commands / Verifier Wall To Carry Forward

Suggested live evidence sweep:

```bash
rg -n "\.handbook|artifacts/handoff|artifacts/planning|feature_slice|state/pipeline" crates/engine crates/pipeline crates/flow crates/compiler/src crates/cli/src
```

Suggested implementation verification:

```bash
cargo test --workspace
```

### Project Structure Focus

- `crates/engine/**`
- `crates/pipeline/**`
- `crates/flow/**`
- adjacent `crates/compiler/src/**` only when proving remaining reusable-internal layout ownership

### Boundaries

- Always:
  - keep this set about reusable layout/storage parameterization
  - distinguish reusable-internal ownership from CLI/product-shell ownership
  - justify any remaining fixed layout assumptions explicitly
- Ask first:
  - any change that alters public CLI behavior only for naming/style reasons
  - any relocation of user-facing artifacts that changes product contract
- Never:
  - widen into orchestration-target generalization
  - widen into compiler narrowing or CLI shell wording cleanup

### Success Criteria To Carry Into `SPECIFY`

- reusable engine/pipeline/flow internals stop owning fixed handbook-product layout assumptions where parameterization is required
- any remaining fixed paths are either CLI-shell-only or explicitly justified
- the resulting spec names which fixed assumptions are acceptable and which are not

---

## Set 2 — Orchestration Target Parameterization Closeout

### Triplet Status

- **Landed remediation triplet**

### Proposed Triplet Stem

- `handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout`

### Landed Files

- `docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md`
- `docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md`
- `docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-tasks.md`

### Seam

Finish the biggest remaining root-plan miss: compile/capture/handoff/provenance behavior must be driven by declared targets instead of one effectively singleton hardcoded runtime wedge.

### Why This Is Its Own Set

- different seam from layout/storage parameterization
- different verifier wall from caller rewires or CLI shell closeout
- highest risk of accidental over-generalization if not kept tightly bounded

### Repo-Truth Problem Statement

The current `SupportedTargetRegistry` is real, but it still centers the approved wedge:

- `pipeline.foundation_inputs`
- `stage.10_feature_spec`
- `feature-slice-decomposer`

Pipeline and stage truth should become catalog-driven runtime truth. Consumer truth should be de-hardcoded into one code-owned validated default owner, but generalized multi-consumer/customizable consumer work should stay deferred.

### Root-Plan Gap Closed By This Set

- `orchestration targets are parameterized`
- `remove direct hardcoding of feature-slice-decomposer`

### Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- existing Phase 2 slice triplets:
  - `handbook-engine-extraction-phase-2-slice-1-supported-target-contract-*`
  - `handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-*`
  - `handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-*`
- declarative truth:
  - `core/pipelines/**`
  - `core/stages/**`
- live runtime surfaces:
  - `crates/pipeline/src/pipeline.rs`
  - `crates/pipeline/src/pipeline_handoff.rs`
  - `crates/pipeline/src/stage_10_feature_spec_provenance.rs`
  - any compile/capture/handoff/provenance adopters still carrying direct target literals

### Assumptions Future Agents Should Surface In `SPECIFY`

1. Pipeline ids should come from declarative pipeline catalog truth.
2. Stage ids should come from declarative stage catalog truth.
3. Consumer ids should move to one code-owned validated registry/default owner for now.
4. This set must **not** build a generic multi-consumer platform.
5. This set must **not** introduce free-form consumer config or a `core/consumers/**` schema unless the user explicitly changes scope.
6. The bounded current consumer `feature-slice-decomposer` remains the only supported consumer during this closeout, but should no longer be a scattered runtime literal.

### Commands / Verifier Wall To Carry Forward

Suggested live evidence sweep:

```bash
rg -n "pipeline\.foundation_inputs|stage\.10_feature_spec|feature-slice-decomposer" crates/pipeline crates/compiler/src crates/cli/src
```

Suggested runtime verification:

```bash
cargo test -p handbook-pipeline
cargo test -p handbook-compiler --test pipeline_handoff
cargo test -p handbook-cli --test cli_surface
```

### Project Structure Focus

- `core/pipelines/**`
- `core/stages/**`
- `crates/pipeline/**`
- adjacent `crates/compiler/src/**` or `crates/cli/src/**` only if still participating in supported-target ownership

### Boundaries

- Always:
  - de-hardcode the current consumer
  - keep pipeline/stage truth declarative
  - keep consumer truth code-owned and validated for now
  - keep runtime behavior bounded to the currently supported wedge unless explicitly expanded
- Ask first:
  - any new consumer schema or `core/consumers/**` tree
  - any new supported consumer, pipeline, or stage
  - any attempt to turn this into a broad artifact/endpoint factory
- Never:
  - widen into a generic multi-consumer framework
  - widen into template/library resolver work
  - treat de-hardcoding the consumer as permission to generalize the product surface

### Success Criteria To Carry Into `SPECIFY`

- compile/capture/handoff/provenance behavior is driven by declared targets
- `pipeline.foundation_inputs`, `stage.10_feature_spec`, and `feature-slice-decomposer` are no longer the baked-in runtime posture
- pipeline/stage truth comes from catalog inputs
- consumer truth lives in one code-owned validated registry/default owner
- generalized consumer-platform work remains explicitly deferred

---

## Set 3 — Direct Caller Rewires + Compiler Narrowing Closeout

### Triplet Status

- **Refreshed triplet landed against live repo truth**

### Existing Triplet Stem

- `handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing`

### Landed Files

- `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md`
- `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md`
- `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md`

### Seam

Make the crate split operationally real by stopping default reliance on `handbook-compiler` and making direct callers use `handbook-engine`, `handbook-pipeline`, and `handbook-flow` wherever the extracted ownership is already real.

### Why This Is Its Own Set

- this is dependency/ownership cleanup
- not target architecture
- not CLI shell finish work

### Repo-Truth Problem Statement

The compiler seam is narrower than it was, but it still remains a substantial compatibility/support layer in places where the root plan expected the extracted crates to become the real center.

### Root-Plan Gap Closed By This Set

- direct callers use the extracted crates as their real owners
- `handbook-compiler` becomes intentionally narrow instead of remaining the de facto center

### Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- existing Slice 4.5 triplet
- existing Phase 4 crate-migration triplets:
  - Slice 4.1 scaffold
  - Slice 4.2 engine migration
  - Slice 4.3 pipeline migration
  - Slice 4.4 flow migration
- live caller surfaces and dependency graph evidence

### Assumptions Future Agents Should Surface In `SPECIFY`

1. This is a refresh/closeout of an existing seam, not a brand-new architectural slice.
2. The extracted crates already own enough behavior that the remaining work is mostly caller/dependency cleanup and seam honesty.
3. The goal is not to delete `handbook-compiler` entirely; the goal is to narrow it intentionally.
4. This set should avoid reopening Phase 1 or Phase 2 architecture unless a caller is blocked by unresolved ownership truth.

### Commands / Verifier Wall To Carry Forward

Suggested live dependency sweep:

```bash
cargo tree -p handbook-compiler
rg -n "handbook_compiler|crate::compiler|use handbook_compiler" crates crates/cli/src
```

Suggested verification:

```bash
cargo test --workspace
```

### Project Structure Focus

- `crates/engine/**`
- `crates/pipeline/**`
- `crates/flow/**`
- `crates/compiler/**`
- direct callers in `crates/cli/**` and any workspace crates still routing through `handbook-compiler`

### Boundaries

- Always:
  - keep this set focused on caller rewires and compiler narrowing
  - distinguish “narrowed” from “deleted”
  - refresh the existing Slice 4.5 docs instead of inventing a different seam
- Ask first:
  - any new public API expansion just to simplify one caller
  - any attempt to merge this set into CLI shell cleanup
- Never:
  - widen into broad product-shell wording work
  - reopen Phase 2 target-parameterization as the main job of this set

### Success Criteria To Carry Into `SPECIFY`

- direct callers use `handbook-engine`, `handbook-pipeline`, and `handbook-flow` where ownership is already real
- `handbook-compiler` becomes intentionally narrow instead of remaining the default center
- the refreshed spec states clearly which compatibility/support seams still legitimately remain

---

## Set 4 — CLI Shell Closeout

### Triplet Status

- **Landed closeout triplet**

### Proposed Triplet Stem

- `handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout`

### Landed Files

- `docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-spec.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-plan.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-tasks.md`

### Seam

Finish the Phase 5 steady-state target: the CLI owns prompting, wording, rendering/help, and exit-code decisions as a thin product shell.

### Why This Is Its Own Set

- this is the product-shell finish pass
- should stay separate from parameterization and compatibility narrowing

### Repo-Truth Problem Statement

The CLI is much thinner than before, but it still is not yet the fully isolated shell the root plan describes. Some wording/rendering/help/exit behavior is still shared too broadly with reusable crates.

### Root-Plan Gap Closed By This Set

- prompting helpers isolated
- shell wording/rendering/help isolated
- exit-code decisions live in the CLI shell
- `main.rs` remains thin

### Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- existing Phase 5 docs:
  - `handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-*`
  - `handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-*`
- any temp planning artifacts for Slice 5.3 if still available
- live CLI shell files:
  - `crates/cli/src/main.rs`
  - `crates/cli/src/pipeline.rs`
  - other command modules under `crates/cli/src/**`
- reusable rendering or support surfaces still leaking product-shell ownership

### Assumptions Future Agents Should Surface In `SPECIFY`

1. The goal is not “make the CLI smaller” in the abstract; the goal is “make the CLI the clear owner of the product shell.”
2. This set should preserve already-landed Phase 5 thinning work.
3. This set should not be used to smuggle in architecture cleanup that belongs to Sets 1 through 3.
4. CLI shell isolation includes help/rendering/prompting/exit posture, not only `main.rs` line count.

### Commands / Verifier Wall To Carry Forward

Suggested CLI evidence sweep:

```bash
rg -n "render_|help|ExitCode|prompt|NEXT SAFE ACTION|OUTCOME:" crates/cli/src crates/pipeline/src crates/compiler/src
```

Suggested verification:

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
```

### Project Structure Focus

- `crates/cli/src/**`
- `crates/cli/tests/**`
- only the specific reusable surfaces still leaking shell ownership

### Boundaries

- Always:
  - keep this set focused on CLI shell ownership
  - preserve behavior while clarifying ownership
  - keep `main.rs` thin without turning submodules into trampoline-only shims
- Ask first:
  - any major public wording change that alters the documented product vocabulary
  - any change that would force neighboring architectural sets to land first
- Never:
  - widen into generic target-parameterization
  - widen into compiler narrowing as the main story
  - reopen earlier CLI slices unless live repo truth requires a narrow correction

### Success Criteria To Carry Into `SPECIFY`

- prompting helpers are isolated into CLI-owned shell surfaces
- wording/rendering/help ownership sits clearly in the CLI shell
- exit-code decisions live in the CLI shell
- `main.rs` remains thin and honest

---

## Cross-Set Boundaries

Future agents should preserve these boundaries between the four sets:

- **Set 1** closes layout/storage parameterization only
- **Set 2** closes orchestration-target parameterization only
- **Set 3** closes caller/dependency ownership only
- **Set 4** closes CLI product-shell ownership only

Do not collapse them into one umbrella closeout spec.

---

## Quick Decision Rules For Future Agents

If the question is:

- “Should this path/root assumption become parameterized?” -> likely **Set 1**
- “Should this pipeline/stage/consumer runtime literal stop being hardcoded?” -> likely **Set 2**
- “Should this caller stop depending on `handbook-compiler`?” -> likely **Set 3**
- “Should this help/rendering/prompt/exit behavior live in CLI?” -> likely **Set 4**

If the work crosses multiple questions at once, stop and split the seam before writing the triplet.

---

## Expected Outcome

After all four sets were specified, planned, task-decomposed, implemented, and honestly closed:

- reusable internals no longer carry unjustified fixed layout assumptions
- runtime target behavior is driven by declared targets instead of a scattered singleton wedge
- the extracted crates are the real operational center instead of `handbook-compiler`
- the CLI is the clear thin product shell

That closeout posture is now the landed Phase 1–5 end state. Phase 6 is the next authoritative step.
