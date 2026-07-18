# HCM-1.4 Planning Remediation 4 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_4`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T202552Z--HCM-1-4--fresh-planning-review-4.json`
- reviewed subject fingerprint:
  `sha256:d85e52b6263a3e3d38aafe80773ef2e1a908579c71db8cf8e9c8f0e5468f65b7`
- review result: `CHANGES_REQUIRED`
- findings: one Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

Review 4 independently accepted every Review 1-3 remedy and found one final
representation leak: the setup error table was exact in prose, but its three
public fields allowed arbitrary downstream combinations and paths.

## Finding disposition

### `HCM-1.4-PR4-R001` — publicly constructible invalid SetupError rows

**Accepted.** The compiler contract now:

- defines one closed `SetupErrorCode` with exactly thirteen variants and an
  ordered `ALL` list;
- stores only that code in `SetupError` behind a private field;
- exposes read-only `code`, derived `kind`, derived `reason_code`, and derived
  `repo_relative_path` accessors;
- restricts construction to crate-private `from_code`;
- maps every code through one exact table to `None`, `.handbook`, or
  `.handbook/state`; and
- requires exhaustive unit replay of all thirteen rows plus a downstream
  `compile_fail` rustdoc example proving neither struct literals nor the
  crate-private constructor are accessible.

An arbitrary kind/reason pair or absolute/unbounded path is now
unrepresentable by the frozen API. The plan, todo, regression matrix, and `06`
proof gate all require the same closure and `cargo test -p handbook-compiler
--doc`.

## Replayed checks

```text
git diff --check
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS: 3 record schemas, 2 internal-dispatch schemas, 2 templates, 39 records,
152 current internal dispatches, 8 admitted legacy dispatches, 39 ledger entries

checked-todo scan
PASS: zero checked implementation items
```

No implementation, Cargo, production test, staging, commit, handoff, ledger,
or HCM-2 work occurred.

## Remediation verdict

The sole Required finding has a bounded documentation-only remedy. Approval
remains pending a fifth different fresh isolated read-only reviewer over the
new complete manifest/fingerprint.
