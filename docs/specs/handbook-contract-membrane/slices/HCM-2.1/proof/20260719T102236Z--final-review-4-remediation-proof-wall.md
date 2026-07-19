# HCM-2.1 final review 4 remediation proof wall

- entry HEAD: `fd6242fbab911eebb09a4d541a6e5e1e32dc5468`
- blocked dispatch:
  `20260719T083834Z--HCM-2-1--fresh-final-implementation-review-4`
- reviewer: `/root/hcm_2_1_final_review_4`
- verdict: `CHANGES_REQUIRED`
- admitted subject fingerprint:
  `sha256:f4065298a91723e40a1a529d3904ceb9dcb3300dc48817fb1fa6befb635fdb01`

## Required finding

The fourth final reviewer admitted all 79 canonical-LF subject entries and
reproduced one mixed-family root-detection defect. A fresh Git repository with
a real `.handbook` directory and successfully authored selected Project Context
reported that Project Context as `PresentNonEmpty` and `valid_canonical_truth`,
but packet inspection still refused with `SystemRootMissing`.

The fixed-sibling loader intentionally excluded the retired
`.handbook/project_context/PROJECT_CONTEXT.md` member. Its scaffold detector
also skipped the entire legacy Project Context kind, however, and therefore did
not inspect the selected `.handbook/project` namespace. With no other sibling
namespace present, the loader misclassified the real root and masked the actual
missing required Charter blocker.

## Impact and repair boundary

GitNexus reported `CRITICAL` upstream risk for
`canonical_root_scaffold_exists`: 148 impacted symbols, 47 direct dependents,
18 affected execution processes, and 20 affected modules. The repair is kept
inside that owner helper. When legacy Project Context loading is disabled, the
helper now checks the internal selected Project Context namespace only when it
belongs beneath the active canonical system root. It still never checks, opens,
or derives root truth from the retired Markdown target.

No profile/descriptor, kind, schema, Cargo, pipeline/stage, SDK, or installed-
skill byte changed. The selected source continues to be loaded and validated by
the descriptor-owned Project Context bridge; the new root check establishes
only scaffold presence.

## Test-first and real-path proof

The public flow regression first failed as required:

```text
left: SystemRootMissing
right: RequiredArtifactMissing
test result: FAILED; 0 passed; 1 failed
```

After the owner repair, a repository containing only valid selected Project
Context produces `c03.handbook_root status=Ok`, retains Project Context as valid
canonical truth, and refuses on the missing required Charter at
`.handbook/charter/CHARTER.md`. A paired negative regression proves a retired
Markdown-only tree still returns `SystemRootMissing` and never exposes that path
in the decision log. A cross-platform engine regression independently proves
the fixed-sibling loader recognizes `.handbook/project` as scaffold while its
Project Context member remains missing and the legacy path remains ignored.

The rebuilt CLI real-path probe produced:

```text
c03.handbook_root status=Ok
c03.artifact kind=ProjectContext required=true presence=PresentNonEmpty
c04.baseline.validation kind=ProjectContext required=true verdict=valid_canonical_truth
refusal category=RequiredArtifactMissing ... kind: Charter ...
```

## Remediation verification

| Command or suite | Result |
|---|---|
| selected-only public flow regression before repair | RED, expected `SystemRootMissing` mismatch |
| selected-only plus legacy-only public flow regressions after repair | PASS, 2 / 2 |
| `handbook-flow` resolver core | PASS, 17 / 17 |
| `handbook-engine` HCM-2.1 Project Context suite | PASS, 17 / 17 |
| native Windows MSVC selected-namespace owner regression | PASS, 1 / 1 |
| rebuilt CLI selected-only author + packet inspect | PASS, correct missing-Charter refusal |
| canonical-LF `cargo test --workspace --all-targets` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` | PASS |
| `cargo fmt --all -- --check` | PASS |
| isolated installed live smoke, including installed packet inspect | PASS, terminal `OK` |
| `git diff --check` | PASS |

The workspace wall was replayed from a fresh canonical-LF staged-subject
checkout. A Windows-working-tree attempt correctly exposed CRLF-only snapshot
and fixture comparisons; it did not fail an affected Rust path, and the exact
canonical-LF rerun passed every workspace target. The installed smoke used a
fresh isolated HOME/state root, removed Codex/OpenAI credential variables, and
installed the current binary plus generated skill assets before execution.

## Resumption gate

The fourth blocked review dispatch and result remain immutable. The parent must
rebuild the complete subject manifest, create a fifth immutable dispatch, and
use another different fresh isolated read-only reviewer. No primary commit is
allowed until that exact subject returns `CLEAN`; HCM-2.2 remains unstarted.
