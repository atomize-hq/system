# Plan: Handbook Engine Extraction Phase 4 Slice 5 (Set 3 / Slice 4.5 Refresh) - Direct Caller Rewires + Compiler Narrowing Closeout

## Objective

Refresh and close the remaining Phase 4 caller/dependency boundary gap by removing stale compiler-facade imports, preserving already-landed direct-owner surfaces, and keeping `handbook-compiler` intentionally narrow and truthfully documented.

Spec reference: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md)

## Major Artifacts

1. Residual caller inventory and owner classification
   - identifies every remaining `handbook_compiler::*` caller in CLI-adjacent code
   - classifies each usage as stale extracted-logic indirection or legitimate retained support seam

2. Direct-owner caller closeout
   - rewires stale convenience imports to `handbook-engine`, `handbook-pipeline`, and `handbook-flow`
   - preserves already-owner-rooted test and flow surfaces

3. Narrow compiler-seam truth
   - keeps `crates/compiler/src/lib.rs` limited to reviewed compatibility/support exports
   - keeps manifest and dependency posture aligned with that boundary

4. Ownership-doc and guard alignment
   - keeps README/contract/help truth aligned with the retained compiler seam
   - records explicit deferrals into Set 4 rather than silently widening into CLI-shell redesign

## Order

### Packet 4.5.1: Refresh the residual caller inventory and freeze the allowed compiler seam

Why first:

- live Slice 4.5 truth is no longer “compiler is still broad”; the refresh must begin by mapping what remains after earlier landings
- direct-owner rewires should happen only after the repo distinguishes stale convenience imports from legitimate retained compiler support seams
- a frozen inventory prevents the slice from drifting into speculative Phase 5 CLI reshaping

Output:

- one reviewed inventory of all remaining `handbook_compiler::*` callers in CLI-adjacent code
- one explicit classification for each caller: stale extracted-logic indirection or retained narrow support seam
- one preserved baseline that `crates/flow/src/lib.rs` and owner-rooted tests are already on the right side of the boundary

### Packet 4.5.2: Rewire stale extracted-logic callers to the real owner crates

Why second:

- once the inventory is frozen, the refresh can remove the stale facade uses without disturbing the legitimate retained compiler seam
- direct imports make the remaining compiler surface reviewable and honest
- keeping this packet focused on stale owner-crate indirection avoids turning it into broad CLI cleanup

Output:

- stale caller imports move to `handbook-engine`, `handbook-pipeline`, or `handbook-flow`
- `handbook-cli` manifest and dependency posture remain aligned with real owners plus the retained compiler seam
- targeted CLI and crate tests prove behavior stability after rewires

### Packet 4.5.3: Keep `handbook-compiler` intentionally narrow and align ownership truth

Why third:

- after stale caller rewires land, the remaining compiler surface can be reviewed as the true retained seam rather than a mixed bag of old and new ownership
- repo-facing docs and help guards should describe the boundary that now actually exists
- this packet closes the “support-surface narrowing” part of the root-plan gap without reopening retirement or Phase 5 shell work

Output:

- a reviewable compiler-root export surface that stays narrow and non-umbrella
- docs/contracts/help guards that describe the retained compiler seam and direct-owner graph honestly
- explicit notes about what remains deferred to Set 4 CLI shell closeout

### Packet 4.5.4: Final closeout proof and deferral ledger

Why last:

- Slice 4.5 should not be called complete until format, lint, tests, docs, and caller inventories all agree
- the closeout must leave a clear boundary between “Phase 4 ownership closeout” and “Phase 5 CLI shell finish pass”

Output:

- one final verification wall for caller rewires, compiler narrowing truth, and workspace health
- one explicit deferral ledger naming the remaining CLI shell work that belongs to Set 4

## Risks And Mitigations

### Risk: the refresh widens into a Phase 5 CLI refactor

Mitigation:

- keep the packet sequence centered on caller classification, rewires, compiler-boundary truth, and verification
- reject opportunistic `main.rs` decomposition or wording cleanup unless a tiny move is strictly required to complete a rewire
- use Set 4 as the explicit sink for remaining shell-finish work

### Risk: legitimate retained compiler support seams get misclassified as stale leftovers

Mitigation:

- freeze the residual caller inventory before rewiring
- require an explicit owner classification for every remaining compiler-root import
- preserve compiler-root usage where the live support seam still spans multiple owner crates

### Risk: the compiler seam silently broadens again while fixing a few callers

Mitigation:

- keep `crates/compiler/src/lib.rs` and `cargo tree -p handbook-compiler -e normal` as first-class review evidence
- reject changes that reintroduce umbrella exports or new convenience facades for engine/pipeline/flow-owned logic

### Risk: docs and help guards drift from live ownership truth

Mitigation:

- update README/docs/contracts/help guards only after the remaining caller graph is settled
- require targeted help/doc verification before the final workspace wall

### Risk: the slice is called complete just because tests are green

Mitigation:

- require the residual caller inventory, dependency trees, and doc truth as closeout evidence in addition to tests
- keep success criteria tied to ownership honesty, not just passing verification

## Parallel Vs Sequential

Sequential:

- freeze the residual caller inventory before changing imports
- land stale caller rewires before reasserting final compiler-boundary truth
- align docs/help guards only after the retained compiler seam is known
- run the full workspace wall last

Parallel opportunities after Packet 4.5.1 lands:

- owner-crate tests can run in parallel while stale caller rewires are being finalized
- doc/help truth edits can be prepared in parallel with compiler-root export cleanup once the retained seam classification is stable

## Verification Checkpoints

### Checkpoint 1: Residual caller inventory is frozen and classified

```bash
rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli/src crates/cli/tests crates/flow crates/compiler/src/lib.rs
cargo tree -p handbook-cli -e normal
```

### Checkpoint 2: Stale extracted-logic callers are removed without behavior drift

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-engine
cargo test -p handbook-pipeline
cargo test -p handbook-flow
```

### Checkpoint 3: Retained compiler seam and ownership docs are aligned

```bash
cargo tree -p handbook-compiler -e normal
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-compiler --test author
cargo test -p handbook-compiler --test doctor
cargo test -p handbook-compiler --test setup
```

### Final checkpoint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Exit Conditions

Slice 4.5 refresh is ready for human review when:

- the remaining `handbook_compiler::*` callers are fully inventoried and each one is either removed or explicitly justified
- direct owner crates are the default import path for extracted logic
- `handbook-cli` manifests reflect real owners plus the retained compiler seam honestly
- `handbook-compiler` remains intentionally narrow and non-umbrella
- repo-facing docs and help guards tell the same ownership story as the live code
- the full workspace verification wall passes
- remaining Phase 5 shell-finish work is explicitly deferred instead of mixed into this slice
