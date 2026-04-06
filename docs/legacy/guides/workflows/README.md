# Workflows

These are practical, copy/paste-friendly “recipes”.

## Foundation + Planning

1. **Foundation run (TEST MODE Charter)**  
   `01_foundation_test_mode_charter.md`

2. **Foundation run (real project / interactive Charter)**  
   `02_foundation_real_project.md`

## Delivery / Planning Layers

3. **Release planning**  
   `03_release_planning.md`

4. **Sprint planning**  
   `04_sprint_planning.md`

5. **Feature spec → slices → execute (if present in your pipeline)**  
   `05_feature_to_slice_execution.md`

## System extension

6. **Add a profile**  
   `06_create_a_profile.md`

7. **Add an overlay**  
   `07_create_an_overlay.md`

8. **Add a runner**  
   `08_create_a_runner.md`

## Development / Test

9. **Foundation run (DEV/TEST Charter Inputs → synthesize Charter)**  
   `09_foundation_inputs_charter.md`

Tip: If you’re unsure which pipeline file is active, run:

```bash
python3 tools/harness.py list
```

If you have multiple pipelines, list them:

```bash
ls -1 pipelines/
```
