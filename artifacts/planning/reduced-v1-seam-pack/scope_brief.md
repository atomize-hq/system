---
pack_id: reduced-v1-seam-pack
pack_version: v1
pack_status: extracted
source_ref: PLAN.md
execution_horizon:
  active_seam: SEAM-1
  next_seam: SEAM-2
---

# Scope Brief - Reduced V1 Rust-First CLI Cutover

- **Goal**: Ship a reduced v1 where Rust is the only supported product path, live planning packets resolve deterministically from canonical project artifacts, fixture-backed execution demos exist honestly, and unsupported live slice execution requests refuse clearly.
- **Why now**: The repo is in transition, and the current Python harness still shapes contributor expectations. The plan needs a governance-ready seam map so implementation can lock the repo contract, establish the Rust workspace, and land the resolver without drifting back into ambiguous support boundaries.
- **Primary user(s) + JTBD**: Repo maintainers and operators who need to establish trusted project posture once, generate the minimum correct planning packet from `.system/` artifacts, inspect why the resolver made its choices, and recover safely when trust is broken.
- **In-scope**:
  - repo-surface freeze of the legacy Python path and Rust-first support messaging
  - root Rust workspace scaffold and CLI command hierarchy
  - typed canonical artifact ingest, manifest, freshness, and override-with-rationale rules
  - deterministic planning packet resolution, decision log, budget policy, and `doctor`
  - markdown, JSON, and inspect renderers over one typed resolver result
  - fixture-backed execution demo plus explicit live-slice refusal
  - tests, CI, install smoke, and doc/help cutover to the supported Rust story
- **Out-of-scope**:
  - preserving Python as a supported runtime path
  - live `project -> feature -> slice` lineage
  - review/fix packets
  - MCP UI
  - unbounded metadata/schema expansion
  - public package-manager or release publishing
- **Success criteria**:
  - the repo root communicates one supported Rust-first story
  - the manifest and resolver produce deterministic planning packets from approved inputs
  - stale, missing, or contradictory inputs refuse with exact next safe actions
  - `inspect` proves the same decision log used by packet generation
  - execution demo scope is honest and cannot be mistaken for live slice support
  - CI validates build, lint, test, and install smoke on `macOS arm64` and `Linux x86_64`
- **Constraints**:
  - exactly one supported packet-resolution authority: Rust
  - canonical project truth lives in repo-local `.system/`, not user-home state
  - root-facing docs may exist, but they are derived views rather than runtime inputs
  - `doctor` is the canonical recovery verb
  - packet budgets are first-class typed policy with deterministic keep, summarize, exclude, and refuse behavior
  - performance work stays simple until measurement proves otherwise
- **External systems / dependencies**:
  - managed project repositories that store canonical `.system/` artifacts
  - legacy Python harness and prompt assets retained only as frozen reference material
  - Rust toolchain and local install targets for `macOS arm64` and `Linux x86_64`
  - CI infrastructure capable of format, lint, test, and install-smoke validation
- **Known unknowns / risks**:
  - contributors may still interpret legacy harness docs as supported unless the repo contract lands first
  - Python implementation patterns may leak back into runtime design during migration
  - the execution demo may be read as live slice capability unless refusal semantics and docs are sharp
  - metadata/schema scope can grow beyond the reduced v1 wedge without strict contract discipline
- **Assumptions**:
  - seam extraction is workflow-first because the operator journey and repo migration order are the dominant organizing axes in `PLAN.md`
  - the execution horizon is inferred from the critical path: lock the repo/support boundary first, then scaffold the Rust workspace and CLI surface
  - seam-exit concerns are inferred from the plan’s explicit contracts, milestones, and cutover rules rather than additional user input

