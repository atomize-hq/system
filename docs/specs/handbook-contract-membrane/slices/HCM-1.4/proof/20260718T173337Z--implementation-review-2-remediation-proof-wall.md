# HCM-1.4 implementation review 2 remediation proof wall

- entry HEAD: `3d0c57f8be0941457d9121ae5d853dd7bd92fbd7`
- branch: `feat/handbook-contract-membrane`
- blocked review: `/root/hcm_1_4_impl_review_2`
- contract-repair review: `/root/hcm_1_4_contract_repair_review_1` — `CLEAN`
- proof ceiling: `BoundaryLanded` for the HCM-1.4 setup/doctor membrane only

## Required-finding disposition

The second fresh implementation review returned four `Required` findings. This
wall preserves that blocked result and records the independently replayable
remediation; it does not revise the prior dispatch or proof wall.

1. **Checkout-stable definition bytes.** The separately reviewed contract
   repair admits only
   `crates/engine/definitions/** text eol=lf` in the repository-root
   `.gitattributes`. No definition blob or fingerprint changed. In a fresh
   native-Windows clone with `core.autocrlf=true`, all 29 paths resolve to
   `text: set`, `eol: lf`; every worktree byte sequence equals its index blob;
   and both mandatory Windows runtime tests pass.
2. **Retired CLI regressions.** The obsolete fixed-universe setup/doctor tests
   remain compile-disabled so they cannot assert removed semantics, but every
   test and helper is inventoried below and its profile-aware replacement or
   deliberate deletion is explicit. The active CLI suite increases to 103
   tests and the full workspace wall executes the new rows.
3. **Compiler-exhaustive exit policy.** `repository_status` now matches all four
   `RepositoryReadinessStatus` variants explicitly: `Ready` succeeds and
   `ActionRequired`, `Indeterminate`, and `Invalid` fail. The closed enum makes
   future variants a compile failure at this boundary.
4. **Named matrix coverage.** New named rows freeze the six kind refs, all
   shipped artifact fields, always/optional/conditional requiredness mappings,
   exact conditional identity/fingerprint/outcome/reason, the four-field-only
   capability projection, every low-level inspection mapping including unsafe
   path and aggregate-later rows, and custom setup/doctor byte equality.

## Retired CLI test inventory

The disabled helpers `tracked_workspace_checkout`,
`assert_doctor_empty_baseline_invalid`, `starter_template_bytes_for_path`,
`nested_git_repo_inside_managed_parent_with_nested_cwd`,
`SetupSuccessExpectation`, and `assert_setup_success` are used only by the
retired tests below. They have no active production or test caller and are
deliberately not replaced as compatibility helpers.

| Retired test | Profile-aware replacement or disposition |
|---|---|
| `bare_setup_routes_to_init_on_uninitialized_repo` | `init_plans_root_creation_but_writes_no_selected_artifact`; active CLI typed-row integration |
| `bare_setup_repairs_file_backed_invalid_system_root` | `root_symlink_and_non_directory_refuse_without_repair`; repair semantics deliberately removed |
| `bare_setup_repairs_symlinked_invalid_system_root` | `root_symlink_and_non_directory_refuse_without_repair`; repair semantics deliberately removed |
| `bare_setup_routes_to_refresh_on_initialized_repo` | `profile_setup_auto_refresh_preserves_existing_root_without_artifact_writes` |
| `setup_init_creates_scaffold_and_starter_files_and_ends_with_system_doctor` | `init_plans_root_creation_but_writes_no_selected_artifact`; canonical artifact writes deliberately removed |
| `setup_init_refuses_when_canonical_system_already_exists` | `profile_setup_auto_refresh_preserves_existing_root_without_artifact_writes` plus compiler mode rows |
| `setup_refresh_default_preserves_canonical_files_by_default` | `profile_setup_auto_refresh_preserves_existing_root_without_artifact_writes` |
| `setup_refresh_rewrite_rewrites_only_setup_owned_starter_files` | `rewrite_refuses_before_root_or_reset_mutation`; rewrite semantics deliberately removed |
| `setup_refresh_reset_state_mutates_only_system_state` | `injected_ready_profile_reset_succeeds_and_preserves_non_state_bytes` |
| `setup_refresh_reset_state_refusal_is_fail_safe` | `injected_ready_profile_reset_refuses_symlink_before_any_state_mutation` and `profile_reset_symlink_refusal_is_fail_safe` |
| `bare_setup_respects_nested_git_root_boundary` | `profile_doctor_discovers_repository_root_from_nested_directory` and `profile_doctor_does_not_cross_nested_git_repository_boundary` |
| `doctor_retry_after_repair_reports_ready_after_repair` | structural row transitions in `missing_and_structurally_valid_rows_follow_applicability`; no legacy semantic-repair authority |
| `doctor_rejects_legacy_placeholder_project_context_truth` | `structural_validation_failed_row_is_exact` and `parser_and_structural_failures_remain_distinct` |
| `doctor_marks_empty_charter_as_invalid_baseline` | `yaml_syntax_invalid_row_is_exact` plus the complete required-row matrix |
| `doctor_marks_empty_project_context_as_invalid_baseline` | `yaml_syntax_invalid_row_is_exact` plus the complete optional-row matrix |
| `doctor_marks_empty_environment_inventory_as_invalid_baseline` | `yaml_syntax_invalid_row_is_exact` plus the complete conditional-row matrix |
| `workspace_root_does_not_ship_canonical_scaffold_and_doctor_points_to_setup` | active no-write compiler/CLI setup rows; no checkout-owned canonical scaffold |
| `setup_scaffold_does_not_satisfy_doctor_or_generate_until_required_truth_is_replaced` | `init_plans_root_creation_but_writes_no_selected_artifact` and shared setup/doctor closure tests |
| `doctor_blocks_when_system_root_missing` | `profile_setup_and_doctor_use_typed_rows_json_and_exit_policy` and exact missing-path inspection rows |
| `doctor_reports_ready_when_feature_spec_is_missing_in_partial_system_tree` | optional/conditional applicability rows and custom ready-profile setup/doctor equality |
| `doctor_blocks_against_repo_root_when_nested_git_repo_has_invalid_system_root` | `profile_doctor_discovers_repository_root_from_nested_directory` |
| `doctor_does_not_cross_nested_git_repo_boundary_into_parent_system_root` | `profile_doctor_does_not_cross_nested_git_repository_boundary` |
| `doctor_reports_ready_when_required_artifacts_present` | `custom_profile_setup_and_doctor_project_byte_equal_closure_rows` and CLI ready rendering unit row |
| `doctor_succeeds_from_nested_directory_inside_ready_repo` | repository-root discovery is frozen by the two active nested-repository CLI tests; ready rendering is frozen independently |
| `doctor_from_committed_fixture_dir_blocks_against_workspace_git_root` | active repository-root discovery/boundary tests; committed fixed-artifact fixture semantics deliberately removed |
| `doctor_blocks_when_optional_project_context_path_is_malformed` | `structural_validation_failed_row_is_exact` and optional applicability mapping rows |

The shipped profile intentionally remains `Indeterminate` because the selected
conditional artifact has no condition-evidence owner in HCM-1.4. The CLI cannot
invent evidence merely to manufacture `Ready`. Ready status, rendering, exit,
and reset behavior are therefore proved with the closed-domain unit row and an
explicit injected custom profile, while the shipped CLI tests prove truthful
indeterminate/no-mutation behavior.

## Named decision and inspection matrix

The decision suite has seven tests. Its added named rows are
`shipped_profile_registry_retains_exact_six_kind_refs`,
`shipped_profile_artifact_rows_match_exact_selected_fields`,
`always_requiredness_maps_to_required_without_condition_truth`,
`conditional_requiredness_binds_exact_unresolved_definition_truth`, and
`shipped_capability_identity_projection_is_exact_and_four_field_only`.

The Unix inspection suite has 22 tests, including exact named rows for required,
optional, and conditional missing/present mappings; syntax, duplicate-key,
non-object, structural, document-limit, aggregate-current-and-later, symlink,
non-regular, unreadable, and optional-requiredness outcomes. Seven lower-level
unit rows directly freeze missing-by-applicability, symlink, non-regular,
platform-unsafe-path, and read-failure mappings. The non-Unix refusal test runs
on the native Windows host.

The compiler setup suite has eight tests. Added rows prove byte-equal custom
setup/doctor closure projection, injected `Ready` reset success with non-state
byte preservation, and symlink refusal before state mutation. CLI ready text,
JSON status, and exactly one trailing LF are frozen in
`ready_report_text_and_json_use_exact_status_and_single_lf`.

## Platform and byte proof

Fresh native-Windows proof checkout:
`C:\hcm14win-20260718T171123Z`. It was cloned from a subject assembled with
`core.autocrlf=true`. All 29 definition worktree files equal their index blobs,
the definition tree equals entry HEAD, both mandatory named Windows tests pass,
and Windows MSVC checks pass for `handbook-engine`, `handbook-compiler`, and
`handbook-cli`. Existing unrelated Windows-only warnings in pipeline/compiler
file-lock code are unchanged.

`cargo package -p handbook-engine --allow-dirty --no-verify` packages 98 files,
1.1 MiB uncompressed and 184.2 KiB compressed. The new archive SHA-256 is
`a92508e18b2ac16cd9a6f21fd8812272839a053ecc0a3a9335528416998e9c6c`.
The immutable manifest blob remains
`a7678b8f3761e681e98f2b5e94459f4568c4d2ff9851e329bc5f776856829a23`;
its 29 rows equal the source tree and archive by exact set, byte size, SHA-256,
and byte sequence. The extracted crate passes `cargo check --all-features`.
`cargo metadata --no-deps --format-version 1` also passes.

## Full replay

The replay uses Rust 1.89.0 in a canonical-LF Ubuntu 22.04 checkout. A root-only
run made the legacy chmod-000 refusal test inapplicable because root can read
mode-000 files; the exact test and full wall were then replayed under the
unprivileged `dev` account.

| Command or suite | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| HCM-1.4 engine decisions | PASS, 7 tests |
| HCM-1.4 engine inspection | PASS, 22 tests plus 7 low-level unit rows |
| HCM-1.2/HCM-1.3 profile and registry regressions | PASS |
| complete `handbook-engine` suite and docs | PASS |
| compiler setup / doctor / author | PASS, 8 / 3 / 47 tests |
| complete `handbook-compiler` suite and docs | PASS |
| CLI surface / author | PASS, 103 / 22 tests |
| `cargo test --workspace --all-targets` as unprivileged Linux user | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| native-Windows named runtime and three-crate MSVC checks | PASS |
| package, extracted crate, metadata, and literal manifest replay | PASS |

The original Ubuntu distro became read-only after an external virtual-disk I/O
failure, so the wall was completed in the separate Ubuntu 22.04 distro. That
host event did not alter the repository or waive any test. The Windows host
later exhausted free space because the proof VHD expanded; disposable Cargo
outputs from the exact proof checkouts were cleaned, existing distro data was
preserved, and the bounded Windows checks completed through the existing Cargo
cache.

Final exact-subject review, manifest replay, staged GitNexus change detection,
validator modes, primary commit, and mechanical handoff/ledger closeout remain
post-wall gates. No HCM-2 work is authorized.
