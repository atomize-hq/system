# Environment Inventory

## Toolchains
- Rust stable toolchain for workspace builds and tests
- Cargo for compile/CLI test execution

## Runtime Assumptions
- local filesystem access for reading canonical repo metadata and proof-corpus fixtures
- no network dependency for compile-path tests
- route basis state stored in a repo-local state file written by `pipeline resolve`

## Important Paths
- `pipelines/foundation_inputs.yaml`
- `core/stages/10_feature_spec.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs`
