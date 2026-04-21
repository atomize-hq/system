---
contract_id: C-06
seam_id: SEAM-6
owner_seam: SEAM-6
version: reduced-v1
currentness: current
status: published
revalidation_triggers:
  - Any change to the demo invocation surface (flags, packet IDs, or fixture selection rules).
  - Any change to fixture lineage determinism rules or ordering tie-breaks.
  - Any change to the required "fixture-backed" labeling rules on any proof surface.
  - Any change to the explicit refusal semantics for unsupported live slice execution requests.
---

# C-06 Fixture Execution Demo Boundary Contract

## Purpose

This contract defines the reduced-v1 execution-demo boundary owned by `SEAM-6`.

It exists to ensure the product can demonstrate an **execution packet** capability in v1 without misrepresenting the system as supporting **live slice execution**.

Downstream conformance (`SEAM-7`) consumes this contract to write tests and docs that can enforce:

- the demo is **fixture-backed** and cannot be mistaken for live capability
- fixture lineage evidence is deterministic and auditable
- unsupported live slice execution requests refuse explicitly and explain the reduced-v1 boundary

## Canonical location

- Canonical artifact: `docs/contracts/C-06-fixture-execution-demo-boundary.md`
- Direct consumer seam: `SEAM-7`

## Owned surface

`C-06` is authoritative about:

- **Demo invocation surface** (how the demo is requested without adding new verbs beyond `C-02`)
- **Fixture lineage** nouns, directory layout, and deterministic ordering rules
- **Fixture-backed labeling** requirements on `generate` and `inspect` proof surfaces
- **Unsupported live slice execution refusal** semantics (category + required wording intent)

`C-06` is **not** authoritative about renderer formatting details (`C-05`) or resolver result structure (`C-04`).

## Normative rules

### Request and packet identity

- Reduced v1 MUST continue to expose only the verbs defined by `C-02` (`setup`, `author`, `pipeline`, `generate`, `inspect`, `doctor`).
- The execution demo MUST be requested through the existing `generate` surface by selecting an execution-demo packet identity.
- The canonical packet identities for this boundary are:
  - **Planning packet** (live): `planning.packet` (default)
  - **Fixture-backed execution demo packet**: `execution.demo.packet`

Any attempt to request **live** slice execution (for example via a reserved packet identity such as `execution.live.packet`, or any future `--live` flag) is an **unsupported request** in reduced v1 and MUST be refused explicitly (see Refusal rules).

### CLI request surface (no new verbs)

The demo request surface MUST NOT add a new verb beyond `C-02`. The supported request surface for selecting the demo is:

- `system generate --packet <packet_id>`
  - default: `--packet planning.packet`
  - allowed: `--packet execution.demo.packet`

If the CLI does not expose `--packet` yet, this contract still defines the canonical mapping that the CLI MUST implement when the demo is added.

### Fixture root and fixture set selection

The fixture-backed execution demo MUST operate on committed fixtures, not on live slice discovery or live execution.

- Canonical fixture root (repo-relative): `tests/fixtures/execution_demo/`
- A fixture set MUST be selected deterministically by an explicit identifier (example CLI surface):
  - `system generate --packet execution.demo.packet --fixture-set <fixture_set_id>`
- The fixture set path MUST be resolved as:
  - `tests/fixtures/execution_demo/<fixture_set_id>/`
- The fixture set MUST contain an explicit `.system/` tree (fixture-backed canonical artifacts) used as the packet basis for the demo.

The demo MUST NOT:

- read from `archived/` as a runtime input
- infer fixture selection from filesystem traversal without an explicit `fixture_set_id`
- treat arbitrary live repo roots as fixture sources

### Fixture lineage nouns and determinism

Terms:

- **Fixture root**: the directory containing all committed execution-demo fixture sets.
- **Fixture set**: one named scenario under the fixture root (selected via `fixture_set_id`).
- **Fixture lineage**: the ordered list of fixture-backed canonical artifacts and dependencies used to produce the demo packet output.

Determinism rules:

- Fixture set selection MUST be explicit (no default "pick one", no glob-based selection).
- When enumerating fixture lineage evidence, ordering MUST be deterministic and MUST NOT depend on filesystem traversal order.
- The lineage order MUST be:
  1. canonical artifacts in `C-03` kind order: `CHARTER`, then `PROJECT_CONTEXT` (if present), then `FEATURE_SPEC`
  2. inherited dependency artifacts (if used), ordered lexically by dependency id
  3. any additional fixture-only evidence items, ordered lexically by their repo-relative path
- If tie-breaks are required, they MUST use lexical order of repo-relative paths.

### Proof-surface labeling rules (fixture-backed)

When the selected packet identity is `execution.demo.packet`:

- `generate` markdown output MUST visibly label the packet as **fixture-backed execution demo** (the phrase "fixture-backed" MUST appear).
- `inspect` output MUST include:
  - the packet identity `execution.demo.packet`
  - the selected `fixture_set_id`
  - the ordered fixture lineage list (as defined above)

The labeling MUST be present in a way that survives narrow-terminal views and cannot be confused with partially implemented live execution capability.

### Refusal rules for unsupported live slice execution requests

If a user attempts a **live slice execution request** (any request whose intent is to execute slices against live repo state rather than fixtures), the system MUST refuse explicitly:

- Refusal category MUST be `UnsupportedRequest` (from `C-04`).
- Refusal summary MUST include the reduced-v1 boundary statement:
  - reduced v1 supports **live planning packets** plus **fixture-backed execution demos only**
  - live slice execution is deferred / unsupported
- The refusal MUST provide exactly one next-safe-action line that points at a safe alternative, such as:
  - run `system generate` for `planning.packet`, or
  - run the fixture-backed demo (`execution.demo.packet`) with an explicit fixture set

## Verification checklist

- [ ] `docs/contracts/C-06-fixture-execution-demo-boundary.md` exists and uses MUST/MUST NOT language for demo invocation, fixture determinism, and refusal semantics.
- [ ] Conformance adds tests proving `execution.demo.packet` surfaces "fixture-backed" labeling and deterministic fixture lineage ordering.
- [ ] Conformance adds tests proving any live slice execution request refuses with `UnsupportedRequest` and the reduced-v1 boundary statement.
