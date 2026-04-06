# Workflow: Foundation run (real project / interactive Charter)

Use this when you want a Charter based on real answers (not synthetic).

## 0) Optional: reset state

```bash
rm -f artifacts/_harness_state.yaml
```

## 1) Run Base

```bash
python3 tools/harness.py run stage.00_base \
  --project-name "YourProject" \
  --repo-or-project-ref "org/repo" \
  --owner "owner" \
  --team "team" \
  --enable-complexity false
```

## 2) Run Charter Interview (interactive)

```bash
python3 tools/harness.py run stage.05_charter_interview \
  --test-mode false \
  --enable-complexity false
```

If your system does not have an explicit `--test-mode` flag, simply omit it and ensure the stage uses the non-test directive by default.

## 3) Decide Project Context

If the harness prompts `needs_project_context`, answer accordingly.
Or force it:

```bash
python3 tools/harness.py run stage.06_project_context_interview \
  --needs-project-context true \
  --enable-complexity false
```

## 4) Run Foundation Pack

```bash
python3 tools/harness.py run stage.07_foundation_pack \
  --enable-complexity false
```

## Verify key files

```bash
ls -1 artifacts/charter/CHARTER.md
ls -1 artifacts/foundation/QUALITY_GATES_SPEC.md
ls -1 artifacts/foundation/quality_gates.yaml
ls -1 artifacts/foundation/ENVIRONMENT_INVENTORY.md
```
