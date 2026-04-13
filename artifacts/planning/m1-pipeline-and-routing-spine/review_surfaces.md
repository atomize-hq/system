# Review Surfaces - M1 Pipeline And Routing Spine

These diagrams orient the pack. They show the actual product/work shape expected to land for `M1`.
They do not, by themselves, satisfy seam-local pre-exec review.

Active and next seams still require seam-local `review.md` artifacts later.

## R1 - Operator workflow

```mermaid
flowchart LR
  Operator["Planning Operator"] --> List["pipeline list"]
  List --> Show["pipeline show --id <pipeline>"]
  Show --> Resolve["pipeline resolve --id <pipeline>"]
  Resolve --> Decision{"Route status"}
  Decision -->|"needs mutation"| StateSet["pipeline state set --id <pipeline> --var key=value"]
  StateSet --> Resolve
  Decision -->|"route is usable"| Downstream["Downstream planning flow consumes resolved route truth"]
```

## R2 - Compiler route-truth flow

```mermaid
flowchart LR
  CLI["CLI adapter"] --> Resolver["Compiler pipeline core"]
  Resolver --> PipelineYaml["pipelines/*.yaml"]
  Resolver --> StageMeta["core/stages/*.md activation metadata"]
  Resolver --> RouteState[".system/state/pipeline/<pipeline-id>.yaml"]
  Resolver --> RouteReport["Resolved route result"]
  RouteReport --> Surface["Compact route report with stage statuses and reasons"]
```

## R3 - Touch surface map

```mermaid
flowchart TB
  Main["crates/cli/src/main.rs"] --> Cmds["pipeline list/show/resolve/state set handlers"]
  Cmds --> Compiler["crates/compiler pipeline modules"]
  Compiler --> YAML["Pipeline definitions"]
  Compiler --> State["Route-state schema + audit trail"]
  Compiler --> Contracts["docs/contracts/ + docs/help surfaces"]
  Contracts --> Proof["tests, goldens, and proof corpus"]
```
