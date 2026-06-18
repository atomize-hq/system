# Handbook → Substrate Parameterization Gap Map

## Purpose

This note maps the current live repo truth against the desired Substrate-owned handbook layout:

```text
.some-project/
  .substrate/
    handbook/
      core/
        pipelines/
        profiles/
        runners/
      artifacts/
        handoff/
          feature_slice/
      state/
        pipeline/
          capture/
```

The goal of this note is to separate four questions:

1. what is already parameterized today
2. what is parameterized internally but not exposed through public import-facing APIs
3. what is still fixed to handbook-product defaults
4. what must change before `.substrate/handbook/**` can work cleanly without compatibility shims or misleading docs

This is a repo-truth note, not an implementation plan.

## Exhaustiveness status

This note is now split into two layers:

1. **primary structural import gaps** — the main parameterization seams required for the import-target crates to live cleanly under `.substrate/handbook/**`
2. **secondary literal / product-shell gaps** — remaining hardcoded defaults, validation text, and product-shell assumptions that still encode handbook-product layout even after the main structural seams are separated

Current confidence level:

- **High** for the primary structural import gaps
- **Medium** for secondary literal and validation gaps inside the import-target crates
- **Low** for a claim that every handbook-product assumption across CLI/compiler/product-shell surfaces has already been enumerated exhaustively

So this file should be read as a **strong structural gap map**, not as a final “zero remaining literals anywhere in the repo” inventory.

## Already parameterized

### 1. Canonical handbook artifact layout in `handbook-engine`

Live code already exposes a public layout contract for the canonical handbook artifact tree:

- `crates/engine/src/canonical_paths.rs`
- public type: `CanonicalLayoutContract`
- public helper: `default_canonical_layout_contract()`
- public loader: `CanonicalArtifacts::load_with_contract(...)`

What this already parameterizes:

- handbook root directory
- charter namespace/path
- project-context namespace/path
- environment-inventory namespace/path
- feature-spec namespace/path

That means engine-level canonical artifacts can already be redirected away from the default `.handbook/**` tree and toward a Substrate-owned namespace such as:

- `.substrate/handbook/charter/**`
- `.substrate/handbook/project_context/**`
- `.substrate/handbook/environment_inventory/**`
- `.substrate/handbook/feature_spec/**`

### 2. Canonical artifact ingest can already consume a non-default root

`CanonicalArtifacts::load_with_contract(...)` is public, so a downstream integrator can already choose a non-default canonical layout contract instead of the default `.handbook/**` contract.

This is the strongest currently-landed parameterization seam that directly helps a future Substrate import.

## Parameterized internally but not public

### 1. Pipeline state / capture / handoff storage layout

`crates/pipeline/src/layout.rs` contains an internal typed storage contract:

- `PipelineStorageLayoutContract`
- `RepoLayoutRoot::with_contract(...)`
- `PipelineStorageLayoutContract::from_paths(...)`

That internal contract already models these locations:

- runtime state root
- pipeline state directory
- stage-capture provenance directory
- capture-cache directory
- handoff feature-slice bundle root

The current default values are still handbook-product paths:

- `.handbook/state`
- `.handbook/state/pipeline`
- `.handbook/state/pipeline/stage_capture`
- `.handbook/state/pipeline/capture`
- `artifacts/handoff/feature_slice`

But the type system shows those values are changeable in principle.

### Why this is still a gap

This parameterization is currently `pub(crate)`, not public API. Substrate cannot import `handbook-pipeline` and supply its own contract through the crate's supported public surface.

So, for the desired Substrate layout, the path model exists internally, but Substrate cannot honestly rely on it yet.

## Not parameterized yet

The sections below are the **primary structural gaps**.

### 1. Declarative roots are still fixed to repo-level `core/**`

`crates/pipeline/src/declarative_roots.rs` still hardcodes:

- `core/pipelines`
- `core/profiles`
- `core/runners`

with helpers such as:

- `pipeline_root()`
- `profile_root()`
- `runner_root()`
- `runner_file(...)`
- `profile_file(...)`

So the desired Substrate layout:

- `.substrate/handbook/core/pipelines`
- `.substrate/handbook/core/profiles`
- `.substrate/handbook/core/runners`

is **not** supported by current live repo truth.

### 2. Supported stage-source assumptions are still fixed to `core/stages/**`

`crates/pipeline/src/pipeline.rs` still owns bounded target/source-path assumptions such as:

- `core/stages/00_base.md`
- `core/stages/10_feature_spec.md`
- other expected `core/stages/**` paths for capture/compile support

Phase 2.4 centralized those assumptions behind `SupportedTargetRegistry`, but it did **not** make the stage-root location configurable.

So the code is better centralized than before, but it still assumes repo-level `core/stages/**` declarative truth.

This gap has two pieces:

- the bounded supported-target constants still encode `core/stages/**` source paths
- stage discovery and validation logic also still assumes repo-level `core/stages/**`, so the remaining gap is wider than just `SupportedTargetRegistry`

### 3. `handbook-flow` public resolution still uses the default canonical root

`crates/flow/src/resolver.rs` still exposes:

- `pub fn resolve(repo_root, request) -> Result<...>`

and that path currently calls:

- `CanonicalArtifacts::load(repo_root.as_ref())`

not:

- `CanonicalArtifacts::load_with_contract(...)`

So even though engine supports a public canonical layout contract, flow's public import-facing resolution path does not yet let Substrate inject a non-default handbook root.

### 4. Residual default `.handbook` assumptions still exist in live code

There are still default-root assumptions and wording tied to `.handbook` in a few places, including:

- flow refusal/blocker fallback behavior that defaults to `default_canonical_layout_contract().system_root_relative()`
- engine authoring/core constants and references that still mention `.handbook/...`
- various tests and diagnostic text that still encode handbook-product defaults

These are smaller than the structural gaps above, but they are still real if the goal is a clean `.substrate/handbook/**` posture.

## Secondary literal / validation gaps

The sections below are **not** the main architectural blockers, but they still matter if the goal is a clean and honest `.substrate/handbook/**` import story.

### 1. Additional `core/stages/**` assumptions exist outside the bounded-target constants

Live `handbook-pipeline` code still contains direct repo-level stage-root assumptions in places beyond the central supported-target constants, including:

- stage discovery rooted at `Path::new("core/stages")`
- stage validation and refusal text that says stages must live under `core/stages/`

So the remaining stage-root gap is not only “replace a few constants”; it also includes direct scanning and validation behavior that still assumes handbook's repo-level declarative stage tree.

### 2. Additional `core/pipelines/**` assumptions exist outside `declarative_roots`

Live `handbook-pipeline` code still contains direct validation text such as:

- pipeline YAML must live under `core/pipelines/`
- pipeline YAML must use the `.yaml` extension under `core/pipelines/`

So even after adding a root contract for `declarative_roots`, the related validation/refusal surface will still need cleanup or parameterized wording.

### 3. `feature-slice-decomposer` remains a bounded consumer default

Phase 2.4 centralized consumer ownership, but the supported consumer id is still a bounded code-owned default:

- `feature-slice-decomposer`

That is not the same kind of root-path gap as `core/pipelines/**`, but it is still a remaining bounded assumption in the orchestration-target model.

This note treats it as a **secondary target-ownership gap** rather than a root-layout gap.

### 4. Flow still carries handbook-root-specific refusal / blocker wording

`handbook-flow` still contains user-facing refusal/blocker text such as:

- missing canonical `.handbook` root
- canonical `.handbook` root is not a directory
- canonical `.handbook` root must not be a symlink

Even if the underlying root becomes injectable, these messages will remain misleading until they are parameterized or rewritten to derive from the active layout contract.

### 5. Engine still contains handbook-product default authoring references

Some engine-side authoring/core constants and references still point directly at handbook-product canonical paths such as:

- `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`
- `.handbook/project_context/PROJECT_CONTEXT.md`
- `.handbook/charter/CHARTER.md#exceptions`

These are smaller than the main structural layout gaps, but they still encode handbook-product assumptions and should be tracked explicitly.

## CLI / compiler product-shell gaps

These are outside the import-target crate boundary, but they are still important if the broader user story is “Substrate imports handbook and everything cleanly reflects `.substrate/handbook/**`”.

### 1. CLI setup / doctor / authoring / rendering still strongly assume `.handbook/**`

Live repo truth still shows many `.handbook`-bound product-shell surfaces in:

- `crates/cli/**`
- `crates/compiler/**`

including:

- setup text and recovery instructions
- doctor/blocker/refusal text
- authoring prompts and canonical file references
- rendering/help output
- product-shell wording around canonical root creation / repair

### 2. Compiler-owned layout still uses handbook-product defaults directly

`crates/compiler/src/layout.rs` still carries compiler-local `.handbook/**` defaults, including canonical artifact paths and authoring lock paths under `.handbook/state/**`.

This does not block the narrow import contract for engine/pipeline/flow by itself, but it does mean the full product-shell story is not yet cleanly substrate-owned.

### 3. CLI / compiler gaps are broader than the import-target gap

A future exhaustive repo-wide parameterization inventory should treat these as a separate class:

- **import-target crate parameterization**
- **product-shell parameterization**

This note is stronger on the first class than the second.

## Must change for `.substrate/handbook/**` to work cleanly

The items below are the **primary required changes** for the import-target crates, followed by secondary cleanup needed for a fully coherent repo story.

### 1. Declarative roots need a real contract

The code needs an explicit parameterization seam for:

- pipeline catalog root
- profile-pack root
- runner root
- likely stage catalog root as well

Without that, Substrate would need to mirror handbook's repo-level `core/**` tree or rely on compatibility tricks.

### 2. Stage-root assumptions need to stop being implicit repo literals

The current supported-target registry is an improvement over scattered literals, but it still assumes a fixed declarative stage layout under `core/stages/**`.

For `.substrate/handbook/**` to work cleanly, those source-path assumptions need to be driven by a contract or explicit imported-layout owner rather than fixed repo literals.

### 3. Pipeline storage layout must become import-facing, not only internal

The existing pipeline storage contract should be made usable through supported public import APIs so Substrate can choose:

- `.substrate/handbook/state/pipeline/**`
- `.substrate/handbook/artifacts/handoff/feature_slice/**`

without reaching into crate-private implementation details.

### 4. Flow-facing public APIs must accept a canonical layout contract

As long as `handbook-flow::resolve(...)` always uses the default canonical root, engine's layout parameterization is only partially useful for an actual downstream integrator.

Substrate needs a flow-facing entry point that can consume the same non-default canonical layout contract rather than always assuming `.handbook/**`.

### 5. Residual handbook-product default strings and fallbacks need cleanup or explicit bounding

Even after the structural seams above land, the remaining `.handbook`-specific text and fallback assumptions will need either:

- true parameterization
- or an explicit statement that they are product-owned defaults outside the reusable import contract

Otherwise the docs will overstate how cleanly handbook can live under a Substrate-owned namespace.

### 6. Stage and pipeline validation/refusal wording must follow the new roots

Even after the underlying roots are parameterized, the current validation and refusal text that explicitly mentions:

- `core/stages/`
- `core/pipelines/`
- `.handbook`

will need to derive from the active imported layout or be rewritten so the user-visible behavior stays honest.

### 7. Decide whether CLI/compiler are in scope for the Substrate-owned layout story

If the intended story is only “import-target crates can live under `.substrate/handbook/**`”, then the main required changes are the structural items above.

If the intended story is “the full handbook product surface should behave as a `.substrate/handbook/**` resident,” then additional CLI/compiler work is still required:

- setup/doctor messaging
- authoring prompts and file references
- compiler-local layout defaults
- shell-rendered recovery/action text

## Practical current-state summary

### Already compatible in principle

- canonical engine artifact layout can be redirected through a public contract

### Compatible in code shape but not yet as a public import contract

- pipeline state / capture / handoff storage roots

### Still incompatible with the target `.substrate/handbook/**` model

- `core/pipelines/**`
- `core/profiles/**`
- `core/runners/**`
- `core/stages/**`
- flow public APIs that assume the default canonical root

### Also still mismatched with a fully clean user-visible Substrate story

- stage/pipeline validation text that still names handbook-product roots
- flow refusal/blocker wording that still names `.handbook`
- CLI/compiler setup, doctor, authoring, and rendering surfaces that still name `.handbook/**`
- compiler-local layout defaults under `.handbook/**`

## Short answer

If Substrate wants handbook to live cleanly under:

- `.substrate/handbook/core/**`
- `.substrate/handbook/state/**`
- `.substrate/handbook/artifacts/**`

then the handbook root parameterization that already landed is only part of the job.

The main remaining **structural import** gaps are:

1. declarative root parameterization
2. stage-root parameterization / bounded target-source path ownership
3. making pipeline storage layout public and injectable
4. threading canonical-layout injection through flow-facing public APIs
5. cleaning or explicitly bounding residual `.handbook` defaults

Additional **secondary cleanup/product-shell** gaps still remain beyond that list, especially around:

6. validation/refusal wording that still names `core/**` or `.handbook/**`
7. CLI/compiler product-shell surfaces that still assume handbook-product layout
