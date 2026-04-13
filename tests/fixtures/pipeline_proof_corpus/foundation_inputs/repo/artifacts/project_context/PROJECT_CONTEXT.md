# Project Context

## Users
- Primary: engineers maintaining the Rust compiler/CLI migration
- Secondary: operators validating pipeline behavior through proof-corpus tests

## Current State
`M1` established pipeline declaration ingest, routing, state mutation rails, and proof-corpus
coverage for resolve/state-set behavior. `M2` now needs one end-to-end compile wedge that uses
the persisted route basis and real stage metadata instead of synthetic output.

## Repo Constraints
- The proof corpus is committed under `tests/fixtures/pipeline_proof_corpus/foundation_inputs`.
- Fixture content should resemble realistic planning artifacts, but stay compact enough for tests.
- Compile success coverage should be shared between compiler and CLI suites via the same goldens.

## Known Risks
- Drift between live repo metadata and the proof-corpus copy can invalidate compile tests.
- Missing upstream artifacts can make compile behavior look broken when the real contract is refusal.
- Over-broad compile support would expand surface area beyond the `M2` budget.
