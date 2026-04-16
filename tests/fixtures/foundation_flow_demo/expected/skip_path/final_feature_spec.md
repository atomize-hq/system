# M4 Foundation Skip Path Proof - Feature Specification
- Spec ID: `FS-M4-FOUNDATION-SKIP-2026-04`
- Date (UTC): `2026-04-15T16:00:00Z`
- Owner: Platform Foundations
- Team: Build Systems
- Repo / Project Ref: `system`
- Charter Ref: `CH-M4-FOUNDATION-SKIP-2026-04`

## 0) Charter Alignment
- Charter Ref: `CH-M4-FOUNDATION-SKIP-2026-04`
- Baseline posture (from Charter): Level 4 - High rigor with delivery pragmatism
- Feature risk delta vs Charter context snapshot: none; the skip proof narrows the route behavior already allowed by the charter

### Dimension alignment
| Dimension | Charter default level | Feature stance (inherit/raise/lower) | Reason |
|---|---:|---|---|
| Speed vs Quality | 4 | inherit | The skip proof still prioritizes correctness. |
| Type safety / static analysis | 4 | inherit | Product ownership is unchanged. |
| Testing rigor | 4 | raise | The skip reason and stage-10 boundary need explicit regression coverage. |
| Scalability & performance | 4 | inherit | Local proof execution remains the target. |
| Reliability & operability | 4 | raise | Incorrect stage activation must fail fast. |
| Security & privacy | 4 | inherit | Fixtures stay local and secret-free. |
| Observability | 4 | raise | Skip reasons and stage-10 boundaries must be obvious in proof output. |
| DX & automation | 4 | inherit | No new commands or flags are introduced. |
| UX polish / API usability | 4 | inherit | Operator guidance improves through truthful proof and docs. |

### Charter red lines check
- Global red lines touched? no
- Dimension red lines touched? no
- Notes / impacted areas: changes stay within fixtures, proof tests, and operator docs

## 1) Summary
Build the `M4` skip-path proof that shows stage 06 remains skipped when the charter already contains the needed planning context, while preserving the same external-model handoff at stage 10.

## 2) Problem & Context
- Current state: the route needs one explicit proof that skip behavior is intentional and not an artifact of missing text.
- Pain / opportunity: without a truthful skip-path corpus, tests could prove the right route shape for the wrong reason.
- Background / constraints from Charter: no new product surface, no repo rereads to reconstruct missing truth, and no direct `compile | capture` story at stage 10.
- Related links: `PLAN.md`, `crates/cli/tests/cli_surface.rs`, `docs/CLI_OPERATOR_JOURNEY.md`

## 3) Goals
- G1: Prove stage 06 remains skipped because `needs_project_context=false` and `charter_gaps_detected=false`.
- G2: Prove stage 07 still provides the context needed for stage 10 compile input.
- G3: Preserve the same external-model stage-10 capture contract as the happy path.

## 4) Non-Goals
- NG1: Add automated project-context synthesis when the charter already answers the needed questions.
- NG2: Allow stage 10 capture to consume compile payload directly.

## 5) Users & Stakeholders
- Primary users: engineers maintaining route-state proof behavior
- Secondary users: operators validating stage activation and skip reasons
- Stakeholders: compiler maintainers, CLI maintainers, docs owners

## 6) Scope
### In Scope
- S1: dedicated skip-path charter, foundation-pack outputs, and final feature-spec output
- S2: explicit proof that stage 06 remains skipped for the correct reason
- S3: explicit proof that stage 10 still consumes completed external markdown

### Out of Scope
- O1: changing stage activation logic
- O2: adding new product surfaces or automation

## 7) Requirements
### Functional Requirements
- R1: Skip-path charter content must already provide the planning context needed for stage 07 and stage 10.
- R2: Skip-path proof must show stage 06 skipped because both activation predicates are false.
- R3: Stage 10 capture must consume a completed external `FEATURE_SPEC.md`.

### Non-Functional Requirements (NFRs)
- Security (Charter target: 4): committed fixtures remain secret-free and local.
- Performance (Charter target: 4): skip-path tests stay focused and fast enough for local runs.
- Reliability (Charter target: 4): skip-path reruns stay deterministic with fixed timestamps and normalized capture ids.
- Observability (Charter target: 4): proof output clearly names the skipped stage and its reason.
- Compatibility / Portability: fixtures remain plain text and repo-local.
- Maintainability: the skip-path corpus stays separate from other fixture families.
- Compliance (if any): not applicable; no regulated data or external systems are involved.

## 8) Acceptance Criteria (testable)
- AC-001: A CLI skip-path test resolves and captures stages 04 and 05, proves stage 06 is skipped, captures stage 07, compiles stage 10, and captures stage 10 from completed external markdown.
- AC-002: The skip-path final `FEATURE_SPEC.md` exactly matches the committed completed stage-10 fixture body.
- AC-003: The skip-path charter contains sufficient planning context to justify `needs_project_context=false`.
- AC-004: Stage-10 success tests do not capture raw compile payload.

## 9) Technical Design
### Proposed Approach (recommended)
- Overview: Provide dedicated skip-path fixtures whose content keeps stage 06 skipped for explicit factual reasons, then use the same stage-10 external-output pattern as the happy path.
- Key components: skip-path charter output, skip-path foundation-pack output, skip-path completed feature-spec output
- Data flow / control flow: resolve computes route basis; stage 05 capture writes the complete charter; resolve keeps stage 06 skipped; stage 07 capture writes the foundation pack; compile emits model input; capture writes final external markdown.
- Interfaces (APIs, CLI commands, library calls, events): `pipeline resolve`, `pipeline capture`, `pipeline compile`
- Storage / persistence (if any): repo-local artifact files and route-state snapshots in temp workdirs
- Error handling expectations: fail if stage 06 activates incorrectly or if stage 10 capture accepts compile payload
- Integration touchpoints (files/modules/services):
  - `crates/cli/tests/cli_surface.rs`
  - `crates/compiler/tests/pipeline_capture.rs`
  - `tests/fixtures/foundation_flow_demo`

### Data Model (if applicable)
- Entities: route basis, skip-path charter, completed feature spec
- Schemas: stage 04 YAML, stage 07 declared multi-file block format, stage 10 markdown contract
- Migrations: none

### Security Considerations
- Threats: fixture wording accidentally implying missing context; incorrect stage-10 handoff
- Mitigations: keep the charter complete and explicit; assert the stage-10 boundary in tests
- Secrets handling: no secrets or production identifiers in committed fixtures

### Observability
- Signals (logs/metrics/traces): resolve stage status, explicit skip reason, capture outcome, capture id
- Key dashboards/alerts (conceptual): not applicable; local proof tests provide the signal

## 10) Alternatives Considered
- Alt A: Reuse the happy-path charter and force skip state manually
  - Pros: fewer fixture files
  - Cons: hides whether the charter actually supports the skip
  - Why not chosen: M4 needs the skip path to be truthful by content
- Alt B: Omit the skip path from the dedicated demo corpus
  - Pros: less initial setup
  - Cons: leaves a major route branch unproved
  - Why not chosen: the bounded M4 proof requires one happy path and one skip path

## 11) Testing Strategy
- Unit tests: structural checks that the final feature spec keeps required sections and traceability
- Integration tests: skip-path CLI journey plus stage-10 capture regression coverage
- E2E tests (if applicable): not applicable
- Negative tests: stage 06 should not activate; stage 10 capture should not consume compile payload directly
- Performance tests (if applicable): not applicable

## 12) Rollout Plan
- Rollout strategy (flags, staged release, canary): land skip-path fixture/test/doc updates with the rest of the M4 packet
- Rollout strategy appropriate for Charter reliability level: require green journey and drift tests before claiming completion
- Monitoring/alerts appropriate for Charter observability level: rely on deterministic test failures for skip reason and stage-10 boundary regressions
- Backward compatibility: no product surface changes
- Migration plan: replace old shortcut proof with explicit skip-path evidence
- Monitoring during rollout: targeted CLI/compiler suites plus full workspace tests
- Rollback plan: revert the M4 proof packet if stage activation or docs drift unexpectedly

## 13) Risks & Mitigations
- Risk 1 -> skip-path charter becomes ambiguous; mitigation: keep the charter explicit about users, rollout, data handling, and scope
- Risk 2 -> tests reintroduce direct compile-to-capture semantics; mitigation: add stage-10 regression assertions

## 14) Open Questions
- Q1: none
- Q2: none

## 15) Traceability Map
- Goals -> Acceptance Criteria:
  - G1 -> AC-001, AC-003
  - G2 -> AC-001
  - G3 -> AC-002, AC-004
- Requirements -> Acceptance Criteria:
  - R1 -> AC-003
  - R2 -> AC-001
  - R3 -> AC-002, AC-004
- Charter constraints referenced:
  - no new product surface area
  - explicit skip reason
  - stage 10 requires completed external markdown

## X) Exceptions (if any)
No charter exceptions are required.

## Y) Debt Tracking (Charter-aligned)
- Expected debt introduced by this feature: future M5 workflow scope decisions
- Where tracked: `PLAN.md`
- When paid down: during bounded M5 planning

## Appendix (optional)
- Glossary: skip path = route where stage 06 remains inactive because both activation predicates are false
- Diagrams (links): not applicable
