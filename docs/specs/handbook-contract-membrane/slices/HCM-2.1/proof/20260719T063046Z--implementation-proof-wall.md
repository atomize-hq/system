# HCM-2.1 implementation proof wall

- entry HEAD: `fd6242fbab911eebb09a4d541a6e5e1e32dc5468`
- branch: `feat/handbook-contract-membrane`
- phase / slice: `HCM-2` / `HCM-2.1`
- selected implementation handoff:
  `20260718T224708Z--HCM-2-1--orchestration--implementation-packet-approved`
- parent orchestration:
  `20260718T204243Z--HCM-2-1--orchestration`
- evidence ceiling: `ContractCorrectAndProven` only for the exact selected
  Project Context canonical-YAML pilot

## Delivered boundary

The implementation makes selected instance `project_context` the sole content
authority for the pilot. The descriptor-owned
`.handbook/project/context.yaml` record has a closed schema-bound parser, typed
decode, exact canonical emitter, fixed in-memory Markdown renderer, separate
source and rendered SHA-256 domains, strict retained-observation stability
checks, and no-follow bounded loading. The old fixed Project Context Markdown
path and its public re-exports are removed; no legacy Markdown byte can select,
overwrite, render, fingerprint, or influence the canonical record.

Compiler and CLI authoring accept only the canonical `1.0` record and preserve
validation-only non-mutation, selected-path ownership, overwrite/root/lock
preflight, and atomic write behavior. Setup remains non-authoring. Doctor keeps
the HCM-1.4 envelope and adds only the bounded nullable Project Context
fingerprint row. Environment Inventory changes only its required Project
Context reference. Flow consumes the selected canonical record and its derived
view through temporary bridge `BR-HCM-2-PILOT-FLOW-01`; the C04 envelope advances
only from `reduced-v1-m8.1` to `reduced-v1-m8.2`.

No Cargo manifest, definition, schema, pipeline/stage, installed-skill, SDK,
Charter-authority, Feature-Spec-authority, or HCM-2.2 surface changed. The
literal HCM-1.2 29-member definition manifest remains byte-identical in source
and the engine package.

## Security and refusal proof

The active proof rows cover duplicate YAML keys at every nesting level,
multiple documents, non-object roots, unknown and missing fields, constants,
types, bounds, stable references, aliases/tags outside the admitted data model,
oversized input, traversal and unsafe paths, component/final symlinks,
non-regular files, typed-decode/schema disagreement, renderer refusal,
different-byte substitution, identical-byte inode ABA, and final read failure.
All refuse before canonical truth or either fingerprint is returned.

Authoring and flow prove no legacy Markdown read/write/copy/rename/import path,
no partial mutation, no unsupported-platform mutation, no clock/environment/
repository input to the renderer, and no source/rendered-domain confusion.
Doctor and flow surface only the packet's closed reasons and preserve the
existing failure precedence.

## Impact analysis

GitNexus impact analysis was run before every existing Rust symbol edit during
implementation. The validator repair required an explicit upstream analysis of
`validate_subject_manifest`; it reported `CRITICAL`, with 115 impacted symbols,
53 direct dependents, and 18 processes. That warning was reported before the
edit. The graph visibly contains unrelated Rust edges and full-text search was
unavailable, so the result was treated conservatively rather than discarded.

The validator change is limited to one immutable dispatch filename plus its
exact raw-LF SHA-256. Duplicate paths still fail, and entry hashes, aggregate
fingerprint, subject fingerprint, schema, and every other ordering check remain
strict. Direct negative proof establishes:

```text
exact filename + exact bytes: PASS
one-byte valid-JSON mutation: REFUSED
exact bytes + unknown filename: REFUSED
```

The admitted immutable dispatch raw SHA-256 is
`7e72338480ec262014c06e6b8661d2ffd578368c3c6c4029458366bdc83b700f`.

## Review and remediation record

Six fresh implementation-review dispatches were executed in order:

1. `20260719T011843Z--HCM-2-1--fresh-implementation-review-1`
2. `20260719T020840Z--HCM-2-1--fresh-implementation-review-2`
3. `20260719T025054Z--HCM-2-1--fresh-implementation-review-3`
4. `20260719T032242Z--HCM-2-1--fresh-implementation-review-4`
5. `20260719T042034Z--HCM-2-1--fresh-implementation-review-5`
6. `20260719T044453Z--HCM-2-1--fresh-implementation-review-6`

Each non-clean result was remediated under the packet and sent to a different
fresh reviewer. Review 6 returned `CLEAN` over its exact 55-path subject.
Subsequent parent proof replay found missing ready-repository fixture coverage,
test budget/setup expectation defects, a cross-document live-smoke authority
contradiction, and the immutable Review 1 manifest-order defect. Those later
changes intentionally supersede Review 6 as the final admission boundary.

The two authority-only packet repairs were separately admitted and returned
`CLEAN`:

- `20260719T054948Z--HCM-2-1--cross-document-smoke-repair-review` permits only
  harness-owned empty scaffold namespaces plus explicit setup exit-1 proof;
  production setup and the installed skill remain non-authoring;
- `20260719T061816Z--HCM-2-1--validator-admission-packet-repair-review` permits
  only the exact filename/raw-byte ordering admission described above.

One final fresh isolated read-only review over the complete implementation,
control, repair, dispatch-history, and proof subject remains the post-wall
commit gate. No subject byte may change after that review returns `CLEAN`.

## Verification wall

The canonical-LF Linux wall and the native-Windows wall used Rust 1.89.0. The
durable result set is:

| Command or suite | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test --workspace --all-targets --no-fail-fast` | PASS |
| `cargo test --workspace --doc` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo doc --workspace --no-deps` | PASS |
| compiler rendering surface / resolver core | PASS, 12 / 18 tests |
| manual ready-fixture checkout | PASS, 2 / 2 fixtures |
| bounded native-Windows Project Context, author, doctor, flow, budget, CLI, and refusal suites | PASS, 45 tests plus final refusal mapping 6 / 6 |
| Unix refusal mapping | PASS, 9 / 9 tests |
| installed all-three live smoke | PASS |
| public install-wrapper smoke | PASS |
| `bash -n tools/ci/codex-skill-live-smoke.sh` | PASS on exact LF subject |
| `tools/check_archive_boundary.py --self-test` | PASS |
| `tools/check_archive_boundary.py` | PASS |
| handoff validator, v1 admission self-test, orchestration-contract self-test | PASS in all three modes |
| staged scope, secret, machine-path, whitespace, and relative-link checks | PASS |

The installed smoke proves install, happy path, all-three offline execution,
existing-charter refusal, outside-Git refusal, and missing-binary refusal. The
harness creates only its own empty scaffold namespaces, asserts setup's expected
exit 1, and does not grant materialization authority to production setup.

## Package replay

`cargo package -p handbook-engine --allow-dirty --no-verify` produced 100 files,
1.1 MiB unpacked and 192.2 KiB compressed. The package archive SHA-256 is
`b8ac8a84800d9f3c1b0334845ec891746f15afa88b05e39e083eb84d19daa1c1`.
The extracted crate passes `cargo check --all-features`, and the workspace path-
dependency scan passes.

The exact 29-member definition manifest SHA-256 remains
`a7678b8f3761e681e98f2b5e94459f4568c4d2ff9851e329bc5f776856829a23`.
Source, package manifest, and archive member sets match exactly by path, size,
SHA-256, and bytes.

## Execution notes and classification

Native-Windows proof was temporarily blocked by host volume pressure and WSL
scratch behavior. Cargo registry caches were moved recoverably to the secondary
volume, the isolated proof roots were rebuilt, and both live-smoke and native
Rust proof completed. No scratch path or host-specific path is persisted in the
subject.

This closes `PG-YAML-01` only for exact family `project_context`.
`PG-ARTIFACT-01` and `PG-YAML-02` remain open program-wide, `PG-KIND-01` remains
open for semantic validation/intake/lifecycle/Projection coverage, and the
mixed-flow bridge stays active only until its mandatory HCM-2.4 deletion gate.
HCM-2.2 is not started.

Primary implementation/control/proof commit, completed parent v1.2 handoff,
and deterministic ledger closeout remain post-wall orchestration steps.
