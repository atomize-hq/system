# HCM-2.1 final review 3 remediation proof wall

- entry HEAD: `fd6242fbab911eebb09a4d541a6e5e1e32dc5468`
- blocked dispatch:
  `20260719T075547Z--HCM-2-1--fresh-final-implementation-review-3`
- reviewer: `/root/hcm_2_1_final_review_3`
- verdict: `CHANGES_REQUIRED`
- evidence ceiling: unchanged from the HCM-2.1 implementation proof wall

## Required finding

The third final reviewer admitted the exact 77-entry canonical-LF subject and
confirmed both lock-contention remediations. It found one remaining installed-
runtime proof gap: the all-three offline live smoke authored Charter, Project
Context, and Environment Inventory and inspected Doctor JSON, but it did not
invoke an installed packet flow surface after authoring. That left the SPEC's
flow-visible Project Context acceptance row dependent on non-installed tests.

The remediation is confined to `tools/ci/codex-skill-live-smoke.sh`. After the
all-three authoring and Doctor assertions, the installed binary now runs
`inspect --packet planning.packet` in the isolated repository with credential
variables removed and the offline PATH. The assertion binds the flow output to
the Doctor row rather than duplicating fingerprints as constants.

## Installed-flow contract

The strengthened smoke requires all of the following from the installed
runtime:

- `OUTCOME: READY` and `OBJECT: planning.packet`;
- selected `ProjectContext [.handbook/project/context.yaml]` plus the matching
  `PROJECT_CONTEXT` section;
- `MODE: rendered from selected canonical YAML`;
- source and rendered SHA-256 values exactly equal to the fingerprints retained
  by Doctor for the same repository;
- rendered `# Project Context`, `## Summary`, and the escaped fixture summary;
- no `.handbook/project_context/PROJECT_CONTEXT.md` legacy path.

No Rust source, artifact definition, schema, pipeline/stage, or installed-skill
byte changed for this remediation.

## Remediation verification

| Command or suite | Result |
|---|---|
| exact LF subject reconstruction | PASS, 78 / 78 staged paths admitted |
| `bash -n tools/ci/codex-skill-live-smoke.sh` | PASS on exact LF subject |
| isolated installed live smoke | PASS |
| installed `cargo install --locked --force --path crates/cli` | PASS |
| generated Codex skill asset installation | PASS |
| installed all-three offline authoring + Doctor + packet inspect | PASS |
| existing-truth, outside-repository, and missing-binary refusal smokes | PASS |
| `git diff --check` | PASS |

The isolated proof used no Codex or OpenAI credential, a fresh HOME and state
root on the durable proof volume, and a clean canonical-LF checkout reconstructed
from the exact staged subject. The live-smoke terminal ended in `OK`.

## Resumption gate

All three blocked final-review dispatches and results remain immutable. The
parent must rebuild the complete-subject manifest, create a fourth immutable
dispatch, and use another different fresh isolated read-only reviewer. No
primary commit is allowed until that exact subject returns `CLEAN`; HCM-2.2
remains unstarted.
