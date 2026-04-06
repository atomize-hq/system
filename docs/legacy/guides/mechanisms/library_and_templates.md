# Mechanism: Library + Templates

## What it is

The **library** contains:
- stage directives (instructions to the LLM)
- document templates (`*.md.tmpl`, `*.yaml.tmpl`)
- optional helper patterns

Typical location:
- `core/library/<topic>/...`

## Why it exists

- Stage bodies stay minimal.
- Directives + templates define the output contract.
- Templates make outputs consistent and easier to parse.

## How it works today

A stage usually includes library inputs like:
- `<topic>_directive.md` (tells the model how to behave)
- `<DOC>.md.tmpl` (structure to fill)
- `<DOC>.yaml.tmpl` (machine-readable version when needed)

The compiled prompt inlines those files so the model can fill them.

## Creating a new library module

1) Create directory:

```bash
mkdir -p core/library/<topic>
```

2) Add:
- `<topic>_directive.md`
- one or more templates

3) Declare in the stage:

- add to `inputs.library` with `required: true`

## Do / Don’t

✅ Do:
- keep templates explicit and structured
- include “output only the document” discipline in directives
- when multi-file outputs are required, specify the wrapper format

❌ Don’t:
- rely on the model to invent file names/paths
- mix tool commands into templates (use profile keys in runner/executor context)
