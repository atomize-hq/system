# Review Surfaces - Reduced V1 Rust-First CLI Cutover

These diagrams orient the pack. They show the actual product/work shape expected to land in reduced v1.
They do not, by themselves, satisfy seam-local pre-exec review.
Active and next seams still require seam-local `review.md` later.

## R1 - Operator workflow

```mermaid
flowchart LR
  O["Operator"] --> S["Guided setup or setup refresh"]
  S --> T["Canonical .system artifacts exist"]
  T --> G["generate planning packet"]
  G --> P["Planning packet + trust header"]
  P --> I["inspect proof surface"]
  G --> D["doctor on missing, stale, or contradictory inputs"]
  D --> R["Next safe action and retry path"]
```

## R2 - Runtime boundary and product surface

```mermaid
flowchart TB
  Root["system repo root approved surface"] --> CLI["Rust CLI"]
  CLI --> Compiler["Rust compiler / resolver core"]
  Compiler --> Project["Managed repo .system canonical artifacts"]
  Root --> Docs["README, help text, docs"]
  Legacy["archived/python-harness reference only"] -. "must not be imported or wrapped" .-> CLI
```

## R3 - Touch surface map

```mermaid
flowchart LR
  Plan["PLAN.md + root docs"] --> Boundary["Repo boundary + support messaging"]
  Boundary --> Workspace["Cargo workspace + crates/cli + crates/compiler"]
  Workspace --> Manifest["Artifact ingest + manifest + freshness"]
  Manifest --> Resolver["Packet resolver + doctor + decision log"]
  Resolver --> Renderers["Markdown / JSON / inspect renderers"]
  Resolver --> Demo["Fixture execution demo + live refusal"]
  Renderers --> Quality["Golden tests + docs/help parity"]
  Demo --> Quality
```
