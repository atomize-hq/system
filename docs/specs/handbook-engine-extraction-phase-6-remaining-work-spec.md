# Spec: Handbook Engine Extraction — Phase 6 Remaining Work

## Objective

Produce the single navigable planning authority for the remaining Phase 6 handbook engine extraction work: two execution lanes and one optional lane, with explicit crate-by-crate import readiness posture.

The three live decisions are:
1. **handbook-engine** — import-ready now. No remaining technical blocker.
2. **handbook-pipeline** — boundary frozen (documented frozen subset of current public surface, closed 2026-06-17). Import-ready pending Lane D adoption plan.
3. **handbook-flow** — needs import-boundary proof (Lane B) before the stable consumer contract is formalized and frozen.

The two execution lanes are:
- **Lane B: Flow import-boundary proof** — prove that `resolve`, `ResolveRequest`, `ResolverResult`, `budget`, `packet_result` can be consumed by Substrate without dragging in CLI shell behavior, compiler rendering/refusal/error glue, or doctor/setup concerns. Then formalize and freeze the stable consumer contract.
- **Lane D: Final Substrate import plan** — write the actual import/adoption plan for engine + pipeline + flow. This is a planning artifact, not import execution.

The one optional lane:
- **Lane C: Engine optional boundary freeze** — only if a stricter publishable API is later desired. Not a blocker for any lane.

### Lane A status (closed)

Lane A (pipeline boundary cleanup) is fully closed:
- Compiler-backed dev-dependency removed (commit `2dfb9b7`).
- Full verification wall passed (2026-06-17): `pipeline_catalog` (14), `pipeline_compile` (21), `pipeline_capture` (45), `pipeline_handoff` (8), compiler `author` (59), `cargo check --workspace` clean.
- Durable boundary decision: **documented frozen subset of the current public surface**.
- In-boundary modules: `pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, plus `pipeline_contract_version()`.
- Out-of-boundary modules: `setup` (CLI/compiler only), `declarative_roots` (internal), and private `layout`/`repo_file_access`/`stage_10_feature_spec_provenance`.
- Triplet archived under `docs/specs/archive/phase-6-pipeline-boundary-cleanup/`.

Lane A is not a remaining lane. It is recorded here for completeness and to prevent reopening.

### Crate import-readiness posture (live repo truth, 2026-06-17)

| Crate | Dependencies | Import target? | Posture |
|-------|-------------|----------------|---------|
| `handbook-engine` | libc, serde, serde_yaml_bw | **Yes** | Import-ready. No handbook-* dependencies. Public surface is narrow and clean. |
| `handbook-pipeline` | handbook-engine, libc, serde, serde_json, serde_yaml_bw, sha2, time | **Yes** | Boundary frozen (documented frozen subset). No handbook-compiler dependency (runtime or dev). |
| `handbook-flow` | handbook-engine | **Yes** | Exports only `budget`, `packet_result`, `resolver`. Depends only on handbook-engine. **Import-boundary not yet formally proved or frozen.** |
| `handbook-cli` | clap + all extracted crates | **No** | Product shell. 574-line clap entrypoint. Not an import target. |
| `handbook-compiler` | all extracted crates | **No** | Transition glue. Re-exports author/setup/doctor/rendering/refusal/resolver adapters. Not an import target. |

## Tech Stack

- Rust 2021 edition workspace
- Crates: `crates/engine`, `crates/pipeline`, `crates/flow`, `crates/cli`, `crates/compiler`
- No external handbook-* dependencies in the three import-target crates beyond `handbook-engine`
- `#![forbid(unsafe_code)]` in engine and flow

## Commands

```bash
# Full workspace check
cargo check --workspace

# Per-crate verification
cargo test -p handbook-engine
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-flow
cargo test -p handbook-compiler --test author

# Dependency verification
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow

# Coupling verification (should return nothing)
rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/

# Full lint wall
cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings
```

## Project Structure

```
crates/
  engine/          → Import-ready: artifact_manifest, author, baseline_validation, canonical_artifacts, freshness
  pipeline/        → Boundary frozen: pipeline, pipeline_capture, pipeline_compile, pipeline_handoff, pipeline_route, route_state
  flow/            → Needs boundary proof: budget, packet_result, resolver
  cli/             → Product shell (not an import target)
  compiler/        → Transition glue (not an import target)
docs/specs/
  handbook-engine-extraction-phase-6-remaining-work-spec.md      → This file
  handbook-engine-extraction-phase-6-remaining-work-plan.md      → Implementation plan
  handbook-engine-extraction-phase-6-remaining-work-tasks.md     → Task breakdown
  archive/                                                         → All historical Phase 1–6 artifacts
```

## Code Style

Existing crate conventions (no style changes expected from this work):

```rust
// engine: explicit re-exports, forbid unsafe, workspace_contract_version() function
#![forbid(unsafe_code)]
pub fn workspace_contract_version() -> &'static str { "C-02" }

// flow: three modules, clean re-exports, flow_contract_version() delegates to engine
pub mod budget;
pub mod packet_result;
pub mod resolver;
pub fn flow_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}

// pipeline: frozen subset re-exports, pipeline_contract_version() delegates to engine
pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
```

## Testing Strategy

- **Lane B (flow boundary proof):** Proof is primarily evidence-gathering + contract formalization, not new code. Verification is:
  - `cargo tree -p handbook-flow` confirms only `handbook-engine` dependency.
  - `rg` confirms zero imports from `handbook_compiler`, `handbook_cli`, `handbook_pipeline` in `crates/flow/src/` and `crates/flow/tests/`.
  - `cargo test -p handbook-flow` confirms existing tests pass.
  - The formalized consumer contract is a doc artifact recording: which symbols Substrate consumes, what types they depend on (transitively from engine only), and what is explicitly excluded.
- **Lane D (import plan):** No code changes. Verification is human review of the plan against the three import-target crate surfaces and the frozen pipeline boundary.
- **Lane C (optional):** If activated, follows the same proof-and-freeze pattern as Lane B but for engine. Not currently activated.

## Boundaries

- **Always:**
  - Ground every claim in live repo truth (cargo tree, rg, cargo test) before recording it.
  - Keep Lane B and Lane D as separate lanes — do not collapse them.
  - Preserve the Phase 6 naming family (no Phase 7).
  - Reference archived artifacts by their archive paths, not their historical pre-archive paths.
- **Ask first:**
  - If Lane B proof reveals an actual coupling that requires a code change (not just formalization).
  - If Lane D plan needs to introduce a narrower public facade for any crate before import.
  - If Lane C should be activated (it is currently optional/closed).
- **Never:**
  - Execute the actual Substrate import — that is beyond Phase 6 scope and requires separate authority.
  - Reopen Lane A (pipeline boundary cleanup is closed).
  - Widen into CLI shell redesign, compiler retirement, publication, or crates.io work.
  - Make `substrate-context` become handbook.
  - Introduce compatibility aliases as a long-term architecture substitute.

## Success Criteria

1. **Lane B — Flow import-boundary proof:**
   - Evidence proves `handbook-flow`'s public surface (`resolve`, `ResolveRequest`, `ResolverResult`, `budget` types, `packet_result` types) depends only on `handbook-engine` types transitively.
   - Evidence proves zero coupling to CLI shell behavior, compiler rendering/refusal/error glue, or doctor/setup concerns.
   - A formalized consumer contract document records the frozen import boundary (in-boundary symbols, their transitive type dependencies, and explicit exclusions).
   - `cargo test -p handbook-flow` passes.
   - `cargo check --workspace` passes.

2. **Lane D — Final Substrate import plan:**
   - A written import/adoption plan for `handbook-engine` + `handbook-pipeline` + `handbook-flow` that:
     - States the import order and rationale.
     - Records the frozen boundary for each crate (engine: current surface; pipeline: documented frozen subset; flow: Lane B consumer contract).
     - Identifies any adapter or facade requirements (or explicitly states none are needed).
     - States the verification gate for the import step itself.
   - The plan is a planning artifact — it does not execute the import.

3. **Lane C — Engine optional boundary freeze:**
   - Remains optional. If not activated, the spec explicitly records that engine's current surface is the working boundary and Lane C is deferred.

4. **Triplet completeness:**
   - The spec, plan, and tasks docs are the single navigable authority for remaining Phase 6 work.
   - Archived Phase 6 families are referenced as provenance, not active authority.
   - No contradictions between this triplet and the root plan, slice map, or closeout map.

## Substrate-Side Constraints (resolved from live repo inspection, 2026-06-17)

Substrate repo: `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate`

| Constraint | Handbook value | Substrate value | Impact |
|-----------|---------------|-----------------|--------|
| Edition | `2021` | `2021` | ✅ Compatible |
| Rust version | Not pinned | `1.89.0` (rust-toolchain.toml + rust-version) | ✅ Compatible — handbook crates build on 1.89 |
| Resolver | `2` | `2` | ✅ Compatible |
| License | Not specified | `MIT` | ⚠️ Handbook crates need a license field added before import (Substrate's `deny.toml` checks licenses) |
| `serde_yaml` | `serde_yaml_bw = "2.5.4"` (different crate) | `serde_yaml = "0.9"` | ⚠️ Different YAML implementations. Not a conflict (different crate names), but Substrate will carry both. `deny.toml` has `multiple-versions = "warn"` — won't block. |
| `sha2` | `0.10` | `0.10` (workspace dep) | ✅ Same version |
| `libc` | `0.2` | `0.2` | ✅ Same version |
| `serde` | `1` with derive | `1.0` with derive | ✅ Compatible |
| `deny.toml` bans | N/A | No crate bans; `unknown-registry = "warn"`, `unknown-git = "warn"` | ✅ Permissive |
| `#![forbid(unsafe_code)]` | Engine + flow | Lift crate also forbids; others use unsafe via nix/libc | ✅ No conflict — imported crates bring their own posture |
| Path deps | `path = "../engine"` etc. | `path = "../sibling"` pattern | ⚠️ Handbook crates must be added as workspace members or path deps in Substrate's workspace |
| Existing handbook refs | N/A | Zero references to "handbook" anywhere in Substrate | ✅ Clean slate |

**Constraints Lane D must address:**

1. **License field**: Add `license = "MIT"` (or matching) to the three import-target crate `Cargo.toml` files before import. Substrate's `deny.toml` runs license checks.
2. **Workspace integration**: Decide whether handbook crates become Substrate workspace members (like `crates/common`, `crates/shell`) or external path/git dependencies. Current Substrate pattern is workspace members with `path = "../sibling"`.
3. **YAML crate divergence**: `serde_yaml_bw` (handbook) vs `serde_yaml` (substrate) — both will coexist. Not a blocker but Lane D should record the decision: keep both, or migrate handbook crates to `serde_yaml` as a follow-up.
4. **No feature flags needed**: Handbook crates have no Cargo features. No feature-gate compatibility concern.

## Resolved Open Questions

1. ✅ **Consumer contract as standalone doc** — approved by human. Will be `docs/specs/handbook-flow-import-boundary-consumer-contract.md`.
2. ✅ **Phased rollout (engine first)** — approved by human. Engine has no intra-workspace deps, so it imports cleanly first.
3. ✅ **Substrate-side constraints** — resolved above. The main action items are: add license fields, decide workspace integration pattern, and record the YAML crate divergence decision.
