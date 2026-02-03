# Workflow: Feature spec → slices → execution (if present)

Some systems keep “feature delivery” in a single pipeline with stages like:
- Feature spec
- Phase decomposition
- Slice generation
- Slice check / refine loop
- Slice execution
- Quality gate

If your current system includes these stages, use this workflow.

## 1) Confirm the stages exist

```bash
python3 tools/harness.py list
```

Look for stage IDs like:
- `stage.10_feature_spec`
- `stage.20_phase_decomp`
- `stage.30_slice_gen`
- `stage.40_slice_check`
- `stage.50_slice_refine`
- `stage.60_slice_execute`
- `stage.70_quality_gate`

## 2) Run Feature Spec

```bash
python3 tools/harness.py run stage.10_feature_spec \
  --enable-complexity false
```

## 3) Phase + slices

```bash
python3 tools/harness.py run stage.20_phase_decomp --enable-complexity false
python3 tools/harness.py run stage.30_slice_gen --enable-complexity false
```

## 4) Slice check/refine loop

```bash
python3 tools/harness.py run stage.40_slice_check --enable-complexity false
# If it fails and your pipeline routes to refine:
python3 tools/harness.py run stage.50_slice_refine --enable-complexity false
python3 tools/harness.py run stage.40_slice_check --enable-complexity false
```

## 5) Execute + quality gate

```bash
python3 tools/harness.py run stage.60_slice_execute --enable-complexity false
python3 tools/harness.py run stage.70_quality_gate --enable-complexity false
```

## Notes

- If you are using parallel workstreams at higher levels, keep **slice execution** single-threaded within a worktree/branch.
- Claim gates “passed” only with evidence (runner + evidence policy).
