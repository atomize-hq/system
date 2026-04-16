# M4 Foundation Journey Proof - Feature Specification
- Spec ID: `FS-M4-FOUNDATION-JOURNEY-2026-04`
- Date (UTC): `2026-04-15T16:00:00Z`
- Owner: Platform Foundations
- Team: Build Systems
- Repo / Project Ref: `system`
- Charter Ref: `CH-M4-FOUNDATION-2026-04`

## 0) Charter Alignment
- Charter Ref: `CH-M4-FOUNDATION-2026-04`
- Baseline posture (from Charter): Level 4 - High rigor with delivery pragmatism
- Feature risk delta vs Charter context snapshot: none; this feature tightens proof and wording around an already bounded route

### Dimension alignment
| Dimension | Charter default level | Feature stance (inherit/raise/lower) | Reason |
|---|---:|---|---|
| Speed vs Quality | 4 | raise | The handoff proof must prefer correctness over test convenience. |
| Type safety / static analysis | 4 | inherit | Existing compiler and CLI ownership remains unchanged. |
| Testing rigor | 4 | raise | M4 adds explicit happy-path, skip-path, and rerun proof coverage. |
| Scalability & performance | 4 | inherit | Local CLI repeatability is still the target. |
| Reliability & operability | 4 | raise | Stage-10 writes must be impossible without external completed output. |
| Security & privacy | 4 | inherit | Fixtures remain secret-free and local. |
| Observability | 4 | raise | Route transitions, skip reasons, and capture boundaries must stay visible. |
| DX & automation | 4 | inherit | No new commands or flag surface are introduced. |
| UX polish / API usability | 4 | inherit | Operator affordances improve through truthful docs and proof output. |

### Charter red lines check
- Global red lines touched? no
- Dimension red lines touched? no
- Notes / impacted areas: updates stay within test fixtures, proof tests, and operator docs

## 1) Summary
Build the `M4` proof wedge that demonstrates one realistic `pipeline.foundation_inputs` journey from charter inputs to a captured `FEATURE_SPEC.md`, with stage 10 explicitly split into compile payload generation and external model output capture.

## 2) Problem & Context
- Current state: existing proof surfaces still preserve an invalid direct `compile | capture` shortcut for stage 10.
- Pain / opportunity: the repo cannot claim a truthful stage-10 contract until tests and docs prove the external-model boundary.
- Background / constraints from Charter: route basis remains canonical, stage 10 compile stays payload-only, and no new product surface area may be introduced.
- Related links: `PLAN.md`, `docs/CLI_OPERATOR_JOURNEY.md`, `crates/cli/tests/cli_surface.rs`

## 3) Goals
- G1: Prove a believable happy path that reaches stage 10 only after stage 06 and stage 07 complete.
- G2: Prove a believable skip path that leaves stage 06 skipped because both activation predicates are false.
- G3: Lock docs/help/tests to the same stage-10 external-model handoff contract.

## 4) Non-Goals
- NG1: Add a new `pipeline run` surface or any automated stage runner.
- NG2: Materialize `FEATURE_SPEC.md` directly from `pipeline compile`.

## 5) Users & Stakeholders
- Primary users: engineers maintaining pipeline proof tests and route contracts
- Secondary users: operators reading proof output and journey docs
- Stakeholders: compiler maintainers, CLI maintainers, docs owners

## 6) Scope
### In Scope
- S1: dedicated M4 demo corpus for happy and skip journey tests
- S2: stage-10 proof updates so capture consumes completed external feature-spec output
- S3: documentation parity for the corrected boundary

### Out of Scope
- O1: new product commands, flags, or writer modes
- O2: downstream adoption beyond the bounded M4 proof and journey artifact

## 7) Requirements
### Functional Requirements
- R1: Journey fixtures must provide realistic repo inputs plus committed model outputs for happy and skip paths.
- R2: Happy-path proof must show compile payload and captured external `FEATURE_SPEC.md` as separate artifacts.
- R3: Skip-path proof must show stage 06 skipped for the explicit reason that both predicates are false.
- R4: Docs and help drift checks must ban direct stage-10 `compile | capture` wording.

### Non-Functional Requirements (NFRs)
- Security (Charter target: 4): Fixtures and expected outputs must contain no secrets or production data.
- Performance (Charter target: 4): Journey tests should remain small enough for focused local CLI runs.
- Reliability (Charter target: 4): Fixed `now_utc` and normalized capture ids must keep reruns deterministic.
- Observability (Charter target: 4): Proof output must make stage status, skip reasons, and handoff boundaries easy to inspect.
- Compatibility / Portability: Fixtures must stay plain text and work in repo-local temp copies.
- Maintainability: The M4 demo corpus must stay separate from existing proof corpora.
- Compliance (if any): not applicable; no regulated data or external service calls are involved.

## 8) Acceptance Criteria (testable)
- AC-001: A CLI happy-path test resolves, captures stages 04/05/06/07, compiles stage 10, captures stage 10 from external completed output, and writes `artifacts/feature_spec/FEATURE_SPEC.md`.
- AC-002: The happy-path final `FEATURE_SPEC.md` exactly matches the committed completed stage-10 fixture body.
- AC-003: A CLI skip-path test proves stage 06 is skipped because `needs_project_context=false` and `charter_gaps_detected=false`.
- AC-004: No stage-10 success-path test captures raw compile payload.
- AC-005: Docs/help drift checks fail if stage 10 is described as direct `compile | capture`.

## 9) Technical Design
### Proposed Approach (recommended)
- Overview: Add a dedicated `tests/fixtures/foundation_flow_demo` corpus and update proof tests to use committed model outputs for captures.
- Key components: repo-shaped fixture root, happy-path model outputs, skip-path model outputs, expected final feature-spec outputs, drift guards.
- Data flow / control flow: resolve computes route basis; captures write upstream artifacts; compile emits stage-10 model input; external committed markdown is supplied to capture; capture writes the final feature spec.
- Interfaces (APIs, CLI commands, library calls, events): `pipeline resolve`, `pipeline capture`, `pipeline compile`, `pipeline state set`
- Storage / persistence (if any): repo-local artifact files and route-state snapshots in temp workdirs
- Error handling expectations: refuse stale route basis, inactive stages, malformed capture payloads, and missing external model output
- Integration touchpoints (files/modules/services):
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
  - `crates/compiler/tests/pipeline_capture.rs`
  - `docs/CLI_OPERATOR_JOURNEY.md`

### Data Model (if applicable)
- Entities: route basis, capture payload, completed feature spec fixture
- Schemas: stage 04 YAML, stage 07 declared multi-file block format, stage 10 markdown contract
- Migrations: none

### Security Considerations
- Threats: fixture drift that teaches an incorrect handoff contract; accidental secret capture in committed outputs
- Mitigations: keep fixtures local and text-only; assert the stage-10 boundary in tests and docs
- Secrets handling: fixtures must contain no credentials, tokens, or production identifiers

### Observability
- Signals (logs/metrics/traces): resolve stage status, capture outcome, capture id, next safe action, refusal reasons
- Key dashboards/alerts (conceptual): not applicable; local proof tests are the observability mechanism

## 10) Alternatives Considered
- Alt A: Keep using the shared proof corpus and patch its stage-10 helper
  - Pros: less fixture setup
  - Cons: keeps the journey proof coupled to narrow compile/capture goldens and muddies corpus purpose
  - Why not chosen: M4 needs a dedicated believable journey corpus
- Alt B: Reuse `execution_demo`
  - Pros: already oriented around a fuller story
  - Cons: tied to packet-generation assumptions rather than pipeline route proof
  - Why not chosen: it would blur the M4 boundary and introduce unrelated expectations

## 11) Testing Strategy
- Unit tests: structural checker coverage for required feature-spec sections and traceability
- Integration tests: happy-path and skip-path CLI journeys plus stage-10 capture contract tests
- E2E tests (if applicable): not applicable; the bounded proof stays at CLI integration level
- Negative tests: refusal when capture tries to use compile payload directly or when route state is stale
- Performance tests (if applicable): not applicable; focused local test runtime is sufficient

## 12) Rollout Plan
- Rollout strategy (flags, staged release, canary): land fixture/test/doc updates together without feature flags
- Rollout strategy appropriate for Charter reliability level: require green proof and drift tests before claiming M4 done
- Monitoring/alerts appropriate for Charter observability level: rely on deterministic test failures and doc-drift guards
- Backward compatibility: preserve existing product surface; only the proof story changes
- Migration plan: replace old stage-10 shortcut tests and then rewrite the operator journey doc from the proved behavior
- Monitoring during rollout: watch targeted CLI/compiler suites plus full workspace tests
- Rollback plan: revert the M4 fixture/test/doc packet if proof surfaces diverge unexpectedly

## 13) Risks & Mitigations
- Risk 1 -> fixture content drifts from canonical stage metadata; mitigation: reuse the canonical repo-shaped inputs in the dedicated corpus
- Risk 2 -> skip path becomes accidental because gap markers imply missing context; mitigation: make skip-path charter content explicit and complete
- Risk 3 -> docs drift back to direct `compile | capture`; mitigation: add negative drift assertions

## 14) Open Questions
- Q1: none for M4 implementation; remaining manual decisions are documented for `M5`
- Q2: none

## 15) Traceability Map
- Goals -> Acceptance Criteria:
  - G1 -> AC-001, AC-002
  - G2 -> AC-003
  - G3 -> AC-004, AC-005
- Requirements -> Acceptance Criteria:
  - R1 -> AC-001
  - R2 -> AC-001, AC-002, AC-004
  - R3 -> AC-003
  - R4 -> AC-005
- Charter constraints referenced:
  - no new product surface area
  - compile remains payload-only
  - stage 10 capture requires external completed output

## X) Exceptions (if any)
No charter exceptions are required.

## Y) Debt Tracking (Charter-aligned)
- Expected debt introduced by this feature: follow-on work for `M5` manual-to-assisted workflow decisions
- Where tracked: `PLAN.md`
- When paid down: during bounded `M5` planning and implementation

## Appendix (optional)
- Glossary: M4 = foundation journey proof and handoff contract
- Diagrams (links): not applicable
