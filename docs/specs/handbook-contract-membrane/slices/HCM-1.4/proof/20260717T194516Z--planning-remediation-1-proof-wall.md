# HCM-1.4 Planning Remediation 1 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_1`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T192357Z--HCM-1-4--fresh-planning-review-1.json`
- reviewed subject fingerprint:
  `sha256:6100edcd8491c3770cf870de3eb62d6692bd897b9bd6bea777712ae463604e5d`
- review result: `CHANGES_REQUIRED`
- findings: six Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

The first dispatch and original planning wall remain immutable evidence. Their
pre-review/cross-document verdict is superseded for approval purposes by this
remediation and the next exact-manifest review; they are not rewritten to hide
the finding lineage.

## Finding dispositions

### `HCM-1.4-PR1-R001` — stale program conclusion

**Accepted.** `00-README.md` now states that HCM-1.1 through HCM-1.3 landed,
HCM-1.4 packet approval is the next planning boundary, and implementation
requires a separate review-clean closeout plus explicit session. It no longer
selects HCM-1.1 or describes the landed kind/profile/registry owners as absent.

### `HCM-1.4-PR1-R002` — unowned evidence/evaluator authority

**Accepted and narrowed rather than inferred.** Live control-pack authority
explicitly leaves the condition input binding, evidence types, freshness,
evaluator, transport, and migration as separate contracts. No landed producer
verifies an authoritative fact or admitted evidence ref. The packet now:

- accepts no observation/evidence/freshness/assertion/boolean/override input;
- binds a conditional descriptor only to its exact selected definition and
  fingerprint;
- emits only `unknown`/`evidence_contract_unavailable`/`indeterminate` plus a
  null evidence-closure fingerprint;
- preserves all six definition-owned outcome names as vocabulary but does not
  synthesize or implement the other five outcomes; and
- makes a separate reviewed admitted-evidence/evaluator packet the stop before
  any outcome can change.

This removes caller-selected authority, undefined semantic identity/
equivalence, and incomplete evidence fingerprints instead of pretending tests
can verify them.

### `HCM-1.4-PR1-R003` — reset atomicity versus exact scope

**Accepted; false claim removed without adjacent mutation repair.** Live
`route_state.rs` applies sequential deletion and has no rollback. HCM-1.4 does
not own that independent behavior. The packet now freezes complete profile
preflight before the unchanged reset call, prohibits mutation for invalid or
indeterminate readiness, records reset success only after the existing applier
returns, and reports a possible legacy partial-failure honestly. It does not
allow or edit `route_state.rs`, claim atomicity, or widen profile adoption into
transactional reset work. A reset repair requires its own packet.

### `HCM-1.4-PR1-R004` — compiler public contract underspecified

**Accepted.** The SPEC now freezes concrete engine and compiler types, private
engine field meanings plus accessors, owned compiler row fields, exact enum and
reason-code variants, default and injected setup/doctor signatures, a sole new
shared compiler projection owner, setup action/status precedence, old-type
removal posture, doctor schema constants/fields, JSON serialization rule, and
the exhaustive setup/doctor exit table. Custom-profile proof passes one exact
`&ResolvedProfileDecisions` to both injected entry points; default entry points
use one explicit package-owned shipped-profile resolver with no ambient input.

### `HCM-1.4-PR1-R005` — impossible compiler package proof

**Accepted.** Compiler `.crate` packaging/extraction was removed from the
mandatory wall because its path-only workspace dependencies are not publishable
without forbidden Cargo version changes. The substitute is exact compiler
source-tree/workspace proof: unchanged manifests/lockfile, workspace metadata,
focused/compiler/workspace tests, exact changed-path equality, and fixture/
absolute-path/untracked scans. Engine package extraction and the literal
29-member definition set/hash/size/byte proof remain mandatory. Compiler
publication stays separately gated.

### `HCM-1.4-PR1-R006` — Windows compile-only proof

**Accepted.** Cross compilation remains supplementary. The regression wall now
names two focused commands that must execute on an actual Windows MSVC host:
engine non-Unix repository-read refusal and compiler proof that refusal starts
no setup mutation. The proof must record host/target/test/exit evidence and the
parent must stop when no Windows runtime is available; no CI workflow edit is
silently inferred.

## Cross-document reconciliation

- `00` carries current landed-dependency and planning-only truth.
- `04` states the exact conditional-unknown boundary and reserves evidence/
  evaluator authority plus unrelated reset repair.
- `06` requires no condition input, exact unknown/unavailable truth, honest
  unchanged reset behavior, engine-package plus compiler-source-tree proof, and
  actual Windows runtime execution.
- SPEC, plan, and unchecked todo carry the same API, scope, condition, reset,
  package, Windows, review, and two-commit contracts.

## Replayed checks

```text
git diff --check
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS: 3 record schemas, 2 internal-dispatch schemas, 2 templates, 39 records,
149 current internal dispatches, 8 admitted legacy dispatches, 39 ledger entries

unchecked todo scan
PASS: no checked implementation item

candidate path scan
PASS: all planning changes remain under
docs/specs/handbook-contract-membrane/
```

No Rust, Cargo, definition asset, schema, runtime, product, reset owner, CI,
HCM-2, staging, or commit occurred during remediation.

## Remediation verdict

All six Required findings have bounded documentation-only remedies. Approval
remains pending a different fresh isolated read-only reviewer over the new
complete manifest and fingerprint. Any new valid finding requires another
bounded remediation and another different fresh reviewer.
