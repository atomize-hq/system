# Test Strategy Brief

## Required Coverage
- happy-path compile for `stage.10_feature_spec` using the shared corpus
- `--explain` proof output over the same success scenario
- stale route-basis refusal
- inactive-stage refusal
- malformed route-basis refusal
- missing required input refusal
- optional artifact absence success

## Test Layers
- compiler tests own compile contract behavior
- CLI tests own command-surface rendering and stdout shape
- both suites consume the same committed proof corpus and success goldens

## Evidence Rules
- keep refusal reasons compact and deterministic
- avoid fixture data that depends on wall-clock mutation
- prefer shared helper support for copying committed corpus cases into temp workdirs
