# Spec: Handbook Published-Import Decoupling — Set 1: Pipeline Boundary Authority Reconciliation

## Assumptions I'm Making

1. This set is **docs-only** and exists to reconcile authority before any new `handbook-pipeline` public-boundary implementation begins.
2. `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` is now the root active authority for exact objective, exact intent, and top-level set sequencing, and this Set 1 triplet must conform to it.
3. `/Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md` and `/Users/spensermcconnell/__Active_Code/system/docs/ideas/handbook-substrate-packet-4-2-proof-findings.md` are the freshest planning inputs for this seam.
4. The older Set 1 / Set 3 parameterization docs and the published-crate consumption docs remain useful provenance, but they currently disagree about whether `handbook-pipeline` layout/declarative-root seams are part of the supported published boundary.
5. The `9b83` Substrate docs under `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/` are **non-canonical** and slightly stale; they may inform boundary thinking, but they must not overrule live `system` repo truth.
6. The top-level decomposition is now three sets, per `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`:
   - **Set 1:** authority reconciliation + objective lock
   - **Set 2:** minimal public capability boundary for `handbook-pipeline`
   - **Set 3:** published-consumer proof + Substrate proof + guard rails

## Objective

Create the first active planning set required after the 2026-06-23 audit invalidated the claim that the full `handbook-engine` / `handbook-flow` / `handbook-pipeline` decoupling story is already honest.

This set does **not** widen the published Rust API or edit production code. Its purpose is to lock one prerequisite truth before implementation:

> Substrate must receive the full reusable handbook capability it actually requires, including the reusable `handbook-pipeline` capability set, but Set 2 must expose only the minimum reviewed public surface needed to provide that capability.

The set must produce an active authority that:

1. reconciles conflicting prior docs,
2. distinguishes **published-consumption proof** from **pipeline public-contract proof**,
3. records the exact objective and exact intent from `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`,
4. states that full reusable pipeline capability for Substrate is a requirement rather than an optional future,
5. defines the Set 2 implementation wall, and
6. defines the Set 3 proof and guard-rail wall.

## Tech Stack

- Rust 2021 workspace in `/Users/spensermcconnell/__Active_Code/system`
- Published crates under review:
  - `handbook-engine = 0.1.1`
  - `handbook-pipeline = 0.1.1`
  - `handbook-flow = 0.1.1`
- Primary code surfaces inspected:
  - `/Users/spensermcconnell/__Active_Code/system/crates/engine/src/lib.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/engine/src/canonical_paths.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/flow/src/lib.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/flow/src/resolver.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/lib.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/declarative_roots.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/layout.rs`
- Primary authority/provenance docs:
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`
  - `/Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-published-crates-and-substrate-consumption-spec.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-import-adoption-plan.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md`
- Non-canonical context inputs:
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-program.md`
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-research/overlays/program-wide/handbook-context-integration.md`
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-research/crates/context/implications.md`

## Commands

```bash
# Read the root active authority plus the freshest audit/proof memo
sed -n '1,260p' docs/specs/MAP.md
sed -n '1,260p' HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md
sed -n '1,260p' docs/ideas/handbook-substrate-packet-4-2-proof-findings.md

# Inspect live crate-boundary truth
sed -n '1,120p' crates/pipeline/src/lib.rs
sed -n '1,220p' crates/pipeline/src/declarative_roots.rs
sed -n '1,260p' crates/pipeline/src/layout.rs
sed -n '1,120p' crates/flow/src/lib.rs
rg -n "pub fn resolve\(|pub fn resolve_with_contract" crates/flow/src/resolver.rs
sed -n '452,520p' crates/flow/src/resolver.rs

# Inspect archived authority claims side by side
rg -n "declarative_roots|layout|frozen subset|private layout|public/import-facing|published" \
  docs/specs/archive/handbook-published-crates-and-substrate-consumption-* \
  docs/specs/archive/handbook-substrate-import-adoption-plan.md \
  docs/specs/archive/handbook-substrate-parameterization-set-1-pipeline-import-layout-* \
  docs/specs/archive/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-*

# Reconfirm published packageability / repo verification truth
cargo package -p handbook-engine --allow-dirty
cargo package -p handbook-pipeline --allow-dirty
cargo package -p handbook-flow --allow-dirty
cargo check --workspace

# Positive external-consumer proof: engine + flow compile from crates.io =0.1.1
TMPDIR=$(mktemp -d)
mkdir -p "$TMPDIR/src"
cat > "$TMPDIR/Cargo.toml" <<'EOC'
[package]
name = "engine_flow_ok"
version = "0.1.0"
edition = "2021"

[dependencies]
handbook-engine = "=0.1.1"
handbook-flow = "=0.1.1"
EOC
cat > "$TMPDIR/src/main.rs" <<'EOC'
use handbook_engine::CanonicalLayoutContract;
use handbook_flow::{resolve_with_contract, ResolveRequest};
use std::path::Path;

fn main() {
    let contract = CanonicalLayoutContract::from_paths(
        ".substrate/handbook",
        "charter",
        "charter/CHARTER.md",
        "project_context",
        "project_context/PROJECT_CONTEXT.md",
        "environment_inventory",
        "environment_inventory/ENVIRONMENT_INVENTORY.md",
        "feature_spec",
        "feature_spec/FEATURE_SPEC.md",
    );
    let _ = resolve_with_contract(Path::new("."), ResolveRequest::default(), contract);
}
EOC
cargo check --manifest-path "$TMPDIR/Cargo.toml"

# Negative external-consumer proof: pipeline layout contract is still private
TMPDIR=$(mktemp -d)
mkdir -p "$TMPDIR/src"
cat > "$TMPDIR/Cargo.toml" <<'EOC'
[package]
name = "pipeline_fail"
version = "0.1.0"
edition = "2021"

[dependencies]
handbook-pipeline = "=0.1.1"
EOC
cat > "$TMPDIR/src/main.rs" <<'EOC'
use handbook_pipeline::layout::PipelineStorageLayoutContract;

fn main() {
    let _ = std::mem::size_of::<PipelineStorageLayoutContract>();
}
EOC
cargo check --manifest-path "$TMPDIR/Cargo.toml"
```

## Project Structure

```text
/Users/spensermcconnell/__Active_Code/system/
  docs/specs/MAP.md
    → root active authority for exact objective, exact intent, and top-level set map
  HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md
    → current root audit that invalidates the "full decoupling already proven" claim
  docs/ideas/handbook-substrate-packet-4-2-proof-findings.md
    → Packet 4.2 proof memo showing the actually-proven published seam
  docs/specs/
    handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
      → this file
    handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
      → Set 1 plan
    handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
      → Set 1 packets/tasks
  docs/specs/archive/
    handbook-published-crates-and-substrate-consumption-*.md
      → prior published-consumption authority; provenance only for this set
    handbook-substrate-import-adoption-plan.md
      → prior import plan; provenance only for this set
    handbook-substrate-parameterization-set-{1,3}-*.md
      → prior parameterization claims; provenance only for this set
  crates/engine/src/
    lib.rs, canonical_paths.rs
      → proven published engine seam
  crates/flow/src/
    lib.rs, resolver.rs
      → proven published flow seam
  crates/pipeline/src/
    lib.rs, declarative_roots.rs, layout.rs
      → disputed pipeline public-boundary seam

/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054/
  crates/shell/src/execution/prompt_fulfillment.rs
    → dedicated downstream proof seam; proves engine + flow consumption, not pipeline adoption

/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/
  code-intelligence-program.md
  code-intelligence-research/overlays/program-wide/handbook-context-integration.md
  code-intelligence-research/crates/context/implications.md
    → stale but useful non-authority context on provider boundaries and why reusable capability should flow through a reviewed boundary instead of broad direct internal coupling
```

## Code Style

Use an explicit contradiction-and-decision style anchored to the MAP objective and intent.

```md
## MAP Alignment

- Exact objective: full reusable handbook capability for Substrate through a reviewed published boundary
- Exact intent: maximum capability, minimum unnecessary public surface

## Authority Reconciliation Matrix

| Source | Claim | Live truth | Status |
|---|---|---|---|
| Packet 4.2 proof memo | published `engine + flow` consumption is proven | true | keep |
| archived Set 1 tasks | pipeline storage layout is public/import-facing | false for crates.io `0.1.1` | supersede |
| published-consumption spec | first-wave published boundary excludes `layout` and `declarative_roots` | true for `0.1.1` | keep, but narrow in scope |

## Set 1 conclusion

Set 2 must implement the full reusable `handbook-pipeline` capability Substrate needs, but it must expose only the smallest reviewed API that provides that capability.
```

Conventions for this set:

- prefer **side-by-side claim matrices** over broad prose
- separate **proven published seam** from **required future capability seam**
- separate **required capability** from **optional overexposure**
- label stale Substrate notes as **non-canonical** when cited
- do not rewrite archive history; instead create active superseding authority

## Testing Strategy

This set is docs-only, but it still has a proof wall. The verification strategy has five levels:

1. **Live source inspection**
   - confirm what `crates/pipeline/src/lib.rs` actually exports
   - confirm `PipelineDeclarativeRootsContract` and `PipelineStorageLayoutContract` remain non-public in the live/published seam

2. **External consumer proof**
   - positive compile for `handbook-engine + handbook-flow`
   - negative compile for `handbook-pipeline::layout::PipelineStorageLayoutContract`

3. **Downstream proof reclassification**
   - confirm Packet 4.2 proves only the narrow published seam it actually uses
   - explicitly reject any reading that treats Packet 4.2 as pipeline-adoption proof

4. **Authority reconciliation proof**
   - final Set 1 docs must name which prior claims remain valid, which are superseded, and what Sets 2 and 3 must do next

5. **MAP alignment proof**
   - final Set 1 docs must show that:
     - full reusable pipeline capability for Substrate is required
     - minimum unnecessary public surface is the governing intent
     - future reviews will use those exact standards

## Boundaries

- **Always:**
  - keep Set 1 docs-only
  - treat `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` as the root authority for this workstream
  - ground every decision in live code, published-crate behavior, and exact source/doc claims
  - treat `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/` as provenance, not as the only active authority location
  - preserve the difference between:
    - published `engine + flow` proof
    - required future `pipeline` capability proof
  - preserve the MAP rule: expose capabilities, not guts
  - call out stale Substrate notes as non-canonical when used

- **Ask first:**
  - if Set 1 evidence suggests the required capability cannot be delivered without exposing broader internals than expected
  - if the cleanest reconciliation would require changing published crate names, ownership, or versioning policy
  - if the likely Set 2 shape would intentionally contradict the MAP intent of minimum unnecessary public surface

- **Never:**
  - edit Rust production code in this set
  - claim Packet 4.2 proved `handbook-pipeline` public-contract usability
  - treat archived Set 1 / Set 3 completion notes as stronger than live crates.io behavior
  - silently rely on stale `9b83` Substrate docs as if they were current authority
  - frame full reusable `handbook-pipeline` capability for Substrate as optional future nice-to-have
  - widen into CLI/compiler/product-shell redesign

## Success Criteria

1. A new active Set 1 authority exists under `/Users/spensermcconnell/__Active_Code/system/docs/specs/`.
2. That active authority explicitly aligns to `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`.
3. The active authority explicitly states that Substrate requires the full reusable `handbook-pipeline` capability set through a reviewed published boundary.
4. The active authority explicitly states that Set 2 must deliver that capability with minimum unnecessary public surface.
5. The active authority explicitly resolves the contradiction between:
   - archived parameterization docs that imply a public/import-facing pipeline seam, and
   - published-consumption docs plus live crates.io behavior that still keep the key seams private.
6. Packet 4.2 is correctly reclassified as proof of feasible published `engine + flow` consumption, not proof of full pipeline-capability readiness.
7. Set 2 and Set 3 receive concrete acceptance walls:
   - Set 2 = capability boundary implementation
   - Set 3 = external consumer proof + downstream Substrate proof + guard rails
8. No Rust code or archived history is changed as part of Set 1 execution.

## Open Questions

1. Should Set 2 expose the required `handbook-pipeline` capability through direct module/type promotion, or through a narrower façade that preserves the same capability with less public surface?
2. Which currently private typed seams are truly required for full Substrate capability, and which are merely one current implementation route that should stay private behind a narrower API?
3. Is the intended downstream consumer shape a direct Substrate call site, a reviewed provider/context boundary, or both?
4. Does the team want the archived parameterization docs to be formally superseded by the new active Set 1 triplet, or should Set 1 also produce a short explicit supersession note?
