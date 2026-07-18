# HCM-1.4 Planning Remediation 2 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_2`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T194705Z--HCM-1-4--fresh-planning-review-2.json`
- reviewed subject fingerprint:
  `sha256:b7c93398ed497c3910e402ce4070a3e5a990cfba9eb15e9fe630af4141db4fbd`
- review result: `CHANGES_REQUIRED`
- findings: four Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

Review 2 independently confirmed the prior condition-authority narrowing,
honest legacy-reset boundary, compiler source-tree substitute, Windows runtime
gate, program status, 29-member package manifest, validator modes, and docs-only
scope. It found four additional implementation-separability gaps, all remediated
below. Both predecessor dispatches and proof walls remain immutable lineage.

## Finding dispositions

### `HCM-1.4-PR2-R001` — author test scaffolds depend on old setup

**Accepted.** Live compiler and CLI author suites call setup from their shared
temporary-repository helpers and then rely on fixed Markdown starter bytes. The
future setup cutover cannot preserve that behavior without violating the no-
artifact-write boundary. The packet now:

- allowlists only `crates/compiler/tests/author.rs` and
  `crates/cli/tests/author_cli.rs` in addition to the prior test files;
- replaces only each `scaffold_repo` helper/imports with a test-owned
  `legacy_authoring_fixture_repo` that writes retained sibling descriptors/
  starter bytes into a temp repo without calling setup;
- admits fixed descriptor/template symbols only in those exact test helper
  hunks; and
- requires all current 47 compiler and 22 CLI author tests while prohibiting
  any production authoring or substantive assertion change.

This decouples a legacy test prerequisite without making setup retain the fixed
universe or widening production authoring.

### `HCM-1.4-PR2-R002` — setup error/precedence map incomplete

**Accepted.** The SPEC now freezes:

- exact inspection -> auto-mode resolution -> init-flag -> root guard ->
  refresh-rewrite -> profile/inspection -> reset-plan -> mutation precedence;
- a total requested/resolved mode by missing/directory/non-directory/symlink
  table with exact root actions;
- exact request, root, rewrite, profile, and reset error-kind/reason pairs;
- exact inspect/repair/create failure categories and safe repo-relative path
  binding; and
- the prohibition on underlying absolute/I/O detail in domain errors.

`Init + rewrite/reset` is invalid-request before materializer handling; valid
refresh rewrite is materializer-unavailable after root guards. No combination
requires an implementation-time decision.

### `HCM-1.4-PR2-R003` — SetupMode serialization unspecified

**Accepted.** `SetupMode` keeps `Auto`/`Init`/`Refresh`, adds `Serialize`, and
freezes exact snake-case wire values `auto`/`init`/`refresh`. `SetupRequest`
retains its existing non-DTO shape. Serialized `SetupPlan` is now compilable
without inventing a wire mapping.

### `HCM-1.4-PR2-R004` — inspection status/reason pairs incomplete

**Accepted.** `ArtifactInspectionReason` now has distinct YAML syntax,
duplicate-key, non-object, structural, document-limit, aggregate-limit,
symlink, non-regular, unsafe-path, unsupported-platform, and read-failure
variants. A closed first-match table maps every required/optional/indeterminate
missing/present result and every parse/schema/size/path/platform/read result to
one exact `(ArtifactInspectionStatus, ArtifactInspectionReason)` pair. Present
invalid conditional data uses the concrete invalid pair; condition-unavailable-
present applies only after structural success. Aggregate exhaustion behavior is
also exact, and the future suite requires one named test per row.

## Cross-document and scope reconciliation

- SPEC owns the exact test allowlist, APIs, error/wire tables, inspection table,
  regression commands, diff exception, and stop boundaries.
- Plan orders compiler/CLI fixture decoupling after their corresponding cutover
  increments and requires the complete 47/22 suites.
- Todo remains entirely unchecked and names all four new proof obligations.
- `06` records the exact setup/error, inspection-pair, and test-helper gates.
- No production authoring, reset-owner, Cargo, CI, condition-evidence,
  canonical-content, or HCM-2 scope was admitted.

## Replayed checks

```text
git diff --check
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS: 3 record schemas, 2 internal-dispatch schemas, 2 templates, 39 records,
150 current internal dispatches, 8 admitted legacy dispatches, 39 ledger entries

unchecked todo scan
PASS: 80 unchecked implementation items; zero checked
```

Review 2 itself ran and passed all three validator modes, Markdown links, the
29-member engine definition manifest replay, Cargo metadata/source-tree checks,
and the live author baselines (47 compiler, 22 CLI). Remediation changed only
planning/control documentation and added no implementation byte.

## Remediation verdict

All four Required findings have bounded documentation-only remedies. Approval
remains pending a third different fresh isolated read-only reviewer over the new
complete manifest/fingerprint. Any valid finding requires another remediation
and another different fresh reviewer.
