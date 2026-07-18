# HCM-1.4 implementation review 3 remediation proof wall

- entry HEAD: `3d0c57f8be0941457d9121ae5d853dd7bd92fbd7`
- branch: `feat/handbook-contract-membrane`
- blocked review: `/root/hcm_1_4_impl_review_3` — `CHANGES_REQUIRED`
- proof ceiling: `BoundaryLanded` for the HCM-1.4 setup/doctor membrane only

## Required-finding disposition

The third fresh implementation review admitted the exact 33-path subject and
replayed the three validator modes, the canonical-LF wall, and both mandatory
native-Windows tests. It returned two `Required` proof-coverage findings. This
wall preserves that result and records the remediation without changing the
production behavior, Cargo surfaces, or definition assets.

1. **Condition evaluation cardinality and missing-definition refusal.** The
   engine library now has active tests for zero selected condition instances,
   one selected instance, and two selected instances that share one condition
   definition. The tests prove exact evaluation counts of zero and one and
   prove that both shared instances bind the same single evaluated result. A
   separate active test removes the selected definition from the test-only
   registry and proves the exact
   `ProfileDecisionError::MissingConditionDefinition` refusal before a decision
   closure can be constructed.
2. **Total setup root/mode/request/status/action proof.** The public compiler
   setup suite now includes an 18-case root/mode/request table covering missing,
   directory, non-directory, rewrite precedence, reset success, and
   root-before-rewrite precedence across auto/init/refresh. A Unix-only test
   creates a real root symlink and proves refusal and preservation for all three
   modes. A further public setup/doctor test constructs exact `Ready`,
   `ActionRequired`, `Indeterminate`, and `Invalid` closures and proves status
   agreement plus exact action precedence for each row.

## Exact active rows

The engine cardinality rows are
`zero_one_and_shared_multiple_condition_instances_have_exact_evaluation_cardinality`
and
`missing_selected_condition_definition_refuses_before_decision_closure_construction`.
They execute inside `profile_selection` so the test can vary only the private
selected condition registry; production visibility and API shape remain
unchanged.

The compiler rows are
`setup_root_mode_and_request_table_is_total_and_ordered`,
`real_root_symlink_is_refused_for_auto_init_and_refresh_without_repair`, and
`setup_and_doctor_cover_all_statuses_with_exact_action_precedence`. The last
row asserts setup/doctor byte-domain agreement in addition to the closed status
and action tables.

These rows supersede two overly broad mappings in
`20260718T173337Z--implementation-review-2-remediation-proof-wall.md`:

- retired `bare_setup_repairs_symlinked_invalid_system_root` is now mapped to
  the real-symlink test above; repair authority remains deliberately removed;
- retired `setup_init_refuses_when_canonical_system_already_exists` is now
  mapped to the 18-case total root/mode/request table rather than an indirect
  auto-refresh row.

The earlier seven-row decision suite remains exact for shipped decision
projection. This wall adds the previously missing zero/one/shared evaluation
cardinality and exact missing-definition refusal; it does not claim those facts
were covered by the earlier decision matrix.

## Replay results

The canonical-LF replay used Rust 1.89.0 on Ubuntu 22.04. The complete wall ran
as the unprivileged `dev` user so unreadable-file behavior remained applicable.

| Command or suite | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| HCM-1.4 engine decisions | PASS, 7 tests |
| HCM-1.4 engine inspection | PASS, 22 tests |
| complete engine library unit suite | PASS, 56 tests including both new rows |
| HCM-1.2/HCM-1.3 profile and registry regressions | PASS |
| compiler setup / doctor / author | PASS, 11 / 3 / 47 tests |
| CLI surface / author | PASS, 103 / 22 tests |
| `cargo test --workspace --all-targets` | PASS as unprivileged Linux user |
| `cargo test --workspace --doc` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| native-Windows mandatory named tests | PASS, 2 tests |

On the native Windows host, the main worktree initially retained CRLF bytes
from before the `.gitattributes` contract repair was staged. That expected
stale-checkout condition reproduced the fingerprint refusal. The 29 definition
files were refreshed from the already verified fresh checkout, all raw
worktree blob hashes then equaled the index blobs, the Cargo dependency cache
was invalidated, and both mandatory Windows runtime tests passed. The separate
fresh `core.autocrlf=true` checkout remains the checkout-stability proof.

## Package and immutable-definition proof

`cargo package -p handbook-engine --allow-dirty --no-verify` packages 98 files,
1.1 MiB uncompressed and 183.9 KiB compressed. Because the new engine proof
module is packaged source, the archive SHA-256 is now
`8105c5b53d3bad76f583fd45e5d225f47514b6eda0ff3814300abc172146ab45`.
The extracted crate passes `cargo check --all-features`, and
`cargo metadata --no-deps --format-version 1` passes.

The immutable 29-member definition manifest remains SHA-256
`a7678b8f3761e681e98f2b5e94459f4568c4d2ff9851e329bc5f776856829a23`.
Its source and package member sets are exact, and every member matches the
literal path, byte size, SHA-256, and byte sequence.

Final exact-subject staging, validator replay, fresh Review 4, staged GitNexus
change detection, primary commit, and mechanical handoff/ledger closeout remain
post-wall gates. No HCM-2 work is authorized.
