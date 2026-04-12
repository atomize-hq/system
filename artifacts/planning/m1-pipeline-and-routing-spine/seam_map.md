# Seam Map - M1 Pipeline And Routing Spine

This seam map treats `M1` as a control-plane milestone:
first make the compiler authoritative for pipeline route truth and route-state truth,
then expose that truth through the supported `pipeline` operator surface,
then freeze the compile handoff contract and conformance rails that keep the surface honest.

## Horizon policy

- Active seam: `SEAM-1` because the compiler-owned route/state core is the critical-path dependency for every visible `pipeline` behavior.
- Next seam: `SEAM-2` because the operator-facing `pipeline` family can be planned next once the route/state core has a concrete contract and touch surface.
- Future seams: `SEAM-3` and `SEAM-4`.
- Only the active seam is eligible for authoritative deep planning by default.
- The next seam may later receive seam-local review and slices, but only provisional deeper planning until it revalidates against the landed route/state contract from `SEAM-1`.
- Future seams remain seam briefs only.

## Seam summary

| Seam | Name | Type | Horizon | Primary value | Key contracts | Primary touch surface |
| --- | --- | --- | --- | --- | --- | --- |
| `SEAM-1` | Compiler Pipeline Core and Routing State | `capability` | `active` | Makes the compiler the source of truth for pipeline loading, activation, resolved-route truth, and narrow route-state mutation semantics | `C-08` | `crates/compiler/`, `pipelines/`, `core/stages/`, `.system/state/pipeline/` |
| `SEAM-2` | Pipeline Operator Surface and ID Resolution | `platform` | `next` | Ships the supported `pipeline` command family, canonical-id lookup, shorthand ambiguity refusal, and normalized CLI render contract | `C-09` | `crates/cli/`, help snapshots, docs/help surfaces |
| `SEAM-3` | Stage Compile Boundary and Route Freshness Handoff | `integration` | `future` | Freezes the M2 compile boundary so route truth, stage metadata, and freshness checks connect cleanly without smuggling compile into M1 | `C-10` | future compile contract docs, `crates/compiler` compile boundary, stage front matter |
| `SEAM-4` | Validation Rails, Proof Corpus, and Docs Realignment | `conformance` | `future` | Locks the shipped M1 surface with realistic proof corpus, goldens, malformed-state rails, performance/security rules, and docs/help parity | `C-11` | `tests/`, proof fixtures, `README.md`, `DESIGN.md`, `docs/`, contract docs |

## Why these seams are decomposable

- `SEAM-1` is bounded to compiler-owned route/state truth; it does not own public help/docs cutover or compile payload generation.
- `SEAM-2` is bounded to the operator-visible `pipeline` surface and CLI lookup/render behavior; it consumes route/state truth instead of redefining it.
- `SEAM-3` is bounded to the compile handoff contract and freshness rules; it does not ship compile implementation or artifact writes in `M1`.
- `SEAM-4` is a genuine conformance seam because the remaining work is cross-seam hardening: proof corpus, docs/help alignment, drift guards, and performance/security boundaries.

## Workstream posture

- `WS-Compiler-Core`: `SEAM-1`
- `WS-Operator-Surface`: `SEAM-2` and `SEAM-3`
- `WS-Conformance`: `SEAM-4`

The workstreams are not safely parallel at extraction time. The critical path is still serial through `SEAM-1`, then `SEAM-2`, then the future compile-handoff and conformance seams.
