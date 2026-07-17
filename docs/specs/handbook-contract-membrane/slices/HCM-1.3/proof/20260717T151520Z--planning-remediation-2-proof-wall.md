# HCM-1.3 Planning Remediation 2 Proof Wall

## Classification and lineage

- phase/slice: `HCM-1` / `HCM-1.3`
- work class: planning-only remediation
- baseline: `c5733785fbd60b7d7a19318cb86058395a02e1c3`
- predecessor subject:
  `sha256:e6e43f4d39f3f0c958bb7b91fe784924071cd4d4d982a0b797aac8f878abc5ec`
- predecessor dispatch:
  `20260717T145715Z--HCM-1-3--fresh-planning-review-2`
- predecessor reviewer: `/root/hcm_1_3_planning_review_2`
- predecessor verdict: `CHANGES_REQUIRED`
- remediation timestamp: `20260717T151520Z`

The prior subjects, proof walls, and dispatches remain immutable historical
evidence. Review 2 confirmed the API, ownership/borrowing, TDD ordering,
package set/hash/size requirements, security boundaries, classification
ceiling, and two-commit closeout without another Critical or Required finding.
This wall remediates its sole Required inventory finding and supersedes the
invalidated 28-file completeness claim. It does not claim final `CLEAN`.

## Review 2 finding accepted

`HCM-1.3-R2-001` identified
`crates/compiler/src/canonical_artifacts.rs`, whose one-line wildcard re-export
propagates the complete fixed `handbook_engine::canonical_artifacts::*` facade
but contains none of the narrow symbol names used by the 28-file scan.

The parent accepts the finding. The SPEC, plan, and todo now freeze a 29-file
direct-surface/facade set and explicitly scan `canonical_artifacts::*`.

## Remediation

The exact scan pattern is now:

```python
pattern = r"CanonicalArtifactKind|CANONICAL_ARTIFACT_ORDER|canonical_artifact_descriptors|canonical_artifacts::\*|CanonicalLayoutContract|baseline_artifact_validations|from_canonical_artifacts|CanonicalArtifactIdentity|CanonicalArtifact\b|ArtifactPresence|ArtifactManifest"
```

The compiler facade is added to the literal ledger with this exact
disposition: it remains an unchanged pre-membrane compiler facade and moves
only with its HCM-1.4 setup/doctor and HCM-2 content consumers; HCM-1.3 adds no
registry bridge or wildcard export.

The parent compared the reproduced scan to a separately literalized expected
set. Result:

```text
PASS: exact 29-file direct-surface/facade set equality
```

The complete literal set is:

```text
crates/cli/src/doctor_rendering.rs
crates/cli/src/rendering.rs
crates/compiler/src/author/charter_shell.rs
crates/compiler/src/author/environment_inventory_shell.rs
crates/compiler/src/author/mod.rs
crates/compiler/src/author/project_context_shell.rs
crates/compiler/src/baseline_validation.rs
crates/compiler/src/blocker.rs
crates/compiler/src/canonical_artifacts.rs
crates/compiler/src/doctor.rs
crates/compiler/src/doctor_shell.rs
crates/compiler/src/layout.rs
crates/compiler/src/lib.rs
crates/compiler/src/refusal.rs
crates/compiler/src/rendering/json.rs
crates/compiler/src/rendering/markdown.rs
crates/compiler/src/rendering/shared.rs
crates/compiler/src/resolver.rs
crates/compiler/src/setup.rs
crates/engine/src/artifact_manifest.rs
crates/engine/src/baseline_validation.rs
crates/engine/src/canonical_artifacts.rs
crates/engine/src/canonical_paths.rs
crates/engine/src/freshness.rs
crates/engine/src/lib.rs
crates/flow/src/budget.rs
crates/flow/src/packet_result.rs
crates/flow/src/resolver.rs
crates/pipeline/src/pipeline_handoff.rs
```

An independent wildcard-facade scan found exactly the one admitted facade:

```text
crates/compiler/src/canonical_artifacts.rs:1:
pub use handbook_engine::canonical_artifacts::*;
```

No additional production wildcard facade was found for artifact manifest,
baseline validation, canonical paths, or freshness. The future implementation
entry gate and todo both require exact 29-file set equality, not count equality.

## Replayed invalidated gates

- the direct fixed-surface/facade command is byte-reproducible: PASS;
- the literal 29-file expected/actual set equality is exact: PASS;
- every newly added facade has an explicit later owner/disposition: PASS;
- future unchanged-file and scope proof names the complete set: PASS;
- todo remains entirely unchecked: PASS;
- `git diff --check`, final-newline, and trailing-whitespace checks: PASS;
- planning changes remain control-pack documentation only: PASS; and
- no implementation, setup/doctor adoption, canonical content migration,
  behavior execution, compatibility dispatch, or later slice began: PASS.

The R1-002 TDD-order and R1-003 public-API remediations were not invalidated by
Review 2 and remain byte-present. The next fresh reviewer must nevertheless
review the entire newly manifested subject, not only this inventory delta.

## Remediation verdict before Review 3

| Gate | Result |
|---|---|
| R2-001 wildcard facade admitted | PASS |
| reproducible literal 29-file set | PASS |
| later-owner disposition | PASS |
| API/TDD/package/security/scope replay | PASS |
| different fresh exact-subject review | PENDING |

A new aggregate fingerprint and immutable dispatch follow. Any valid Critical
or Required finding requires a new proof wall/fingerprint and another different
fresh reviewer.
