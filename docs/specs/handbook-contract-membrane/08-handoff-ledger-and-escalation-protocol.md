# Handoff Ledger and Escalation Protocol

## Purpose

One top-level phase/slice orchestrator owns the canonical continuation record. Internal implementation, documentation, proof, remediation, and review subagents return structured results to that parent; they do not create global handoffs or ledger entries.

The protocol supports:

- review-clean slice completion;
- safe top-level continuation across a real context/runtime boundary;
- human-visible or interactive action;
- external blockers;
- Resolution/authority escalation;
- parent-managed documentation repair and finding-driven decomposition;
- built-in delegation evidence and review/remediation/re-review lineage;
- optional top-level start/end Snapshot Memory refs and deterministic deltas until snapshot capture becomes mandatory.

A dispatch is an execution/audit envelope. Creating an internal dispatch is not completion and normally does not return control to the user.

## Storage model

~~~text
handoffs/
├── handoff-record.schema.json          # immutable v1.0 historical schema
├── handoff-record.v1.1.schema.json     # immutable v1.1 historical schema
├── handoff-record.v1.2.schema.json     # current parent-closeout schema
├── ledger-entry.schema.json
├── internal-dispatch.schema.json       # hash-admitted v1.0 first-review schema
├── internal-dispatch.v1.1.schema.json  # current replayable-subject envelope
├── handoff-template.json               # current v1.2 parent template
├── internal-dispatch-template.json     # current internal JSON dispatch template
├── dispatch-template.md                # human-readable field/instruction guide
├── validate_handoffs.py
├── ledger.jsonl
├── records/
│   └── <immutable-parent-or-historical-records>.json
└── dispatches/
    ├── <hash-admitted-legacy-dispatches>.md
    └── <current-internal-dispatches>.json
~~~

## Canonical truth and immutability

- Each file under records/ is an immutable canonical handoff record.
- ledger.jsonl is a byte-deterministic rebuildable query index, not a second authority.
- Corrections create a new record whose supersedes array names only the prior recommendations or facts it replaces.
- source_handoff_ids records consumed resume context without implying that every source is superseded.
- Handoffs reference snapshots/deltas and semantic records; they do not duplicate or change those records' authority.
- Internal JSON dispatches are immutable bounded execution envelopes.
- The eight pre-correction Markdown dispatches remain immutable evidence of the user-routed workflow defect; they are not migrated into the current internal format.
- Existing v1.0 and v1.1 records remain immutable historical evidence and are never rewritten into v1.2.
- The first HCM-0.8 internal-dispatch v1.0 review remains hash-admitted evidence of the findings that required v1.1; it is not rewritten.

validate_handoffs.py hash-admits the exact historical v1.0/v1.1 record filenames, IDs, versions, and bytes plus the exact eight legacy Markdown dispatch filenames and bytes. Unknown new historical-version records, missing history, or byte changes fail closed.

New v1.2 records and current internal-dispatch v1.1 JSON are validated against their schemas, replayable subjects, identities, cross-record lineage, and Git-reviewed diffs. Once committed, they are immutable and corrections are additive.

## Handoff schema routing

handbook.session-handoff records route only by top-level schema_version:

| Version | Schema | Creation policy |
|---|---|---|
| 1.0 | handoff-record.schema.json | Historical-only exact admission; never create. |
| 1.1 | handoff-record.v1.1.schema.json | Historical-only exact admission; never create. |
| 1.2 | handoff-record.v1.2.schema.json | Required for every new top-level closeout. |

V1.2 requires:

- session.kind=orchestration;
- orchestration_id and source_handoff_ids;
- stop_reason and delegation-capability evidence;
- reviewed-state fingerprint/proof refs;
- proof-relevant delegated_runs, typed parent/delegated remediations, and their lineage;
- snapshot_refs and semantic_refs;
- resume rather than the historical queue-shaped next_session object.

A completed v1.2 record requires stop_reason=completed, available built-in delegation, a completed clean review, and resume.execution_target=none. capability_unavailable requires status=blocked.

## Dispatch routing

The first HCM-0.8 review used hash-admitted handbook.internal-dispatch v1.0. Current internal dispatches use handbook.internal-dispatch v1.1 JSON and internal-dispatch-template.json.

The schema requires:

- parent orchestration, phase, slice, packet, and subject fingerprint;
- a sorted repository-relative path/SHA-256 manifest whose aggregate is always recomputed, whose entries are checked against live files when executed, and whose final clean subject is replayed from the primary `reviewed_state.baseline_head` at completed closeout;
- execution_target=internal_subagent;
- agent_type=default and fresh_context_required=true;
- closeout_owner=parent_orchestrator;
- ordered required_skills beginning with using-agent-skills;
- exact authority, repo truth, allowed scope, non-goals, tasks, gates, and stop conditions;
- built_in_subagent result transport;
- explicit prohibition of global handoff, ledger write, and user task hop.

dispatch-template.md is explanatory guidance for these fields and for rare top_level_resume/human_interactive presentation. New internal proof runs use JSON so the execution envelope can be schema-validated and fingerprint-bound.

## Snapshot, handoff, dispatch, and result roles

| Record | Question answered |
|---|---|
| Snapshot Memory | What selected state was observed at a top-level or strategic internal boundary? |
| SnapshotDelta | What changed between compatible observations? |
| Handoff | Why did top-level orchestration stop, what happened, and how may it resume? |
| Internal dispatch | What exact built-in subagent job is authorized by the active parent? |
| Delegated-run result | What did that built-in agent return, against which subject, and how did the parent dispose it? |

Delegated-run results are recorded in the parent handoff when proof-relevant. They do not become standalone global handoffs merely because a subagent turn ended.

Snapshot refs remain nullable until HCM-3.4 lands. V1.2 snapshot_refs still requires an honest capture_status:

- captured: all applicable refs exist;
- partial: a bounded subset exists and omissions are named;
- failed: required capture failed and a blocker/finding is recorded;
- not_available: capability has not landed and refs remain null.

Never invent a snapshot ref. semantic_refs remains mandatory and uses empty arrays/null when no applicable semantic record exists.

## True top-level stop reasons

| stop_reason | Required meaning |
|---|---|
| completed | Selected slice/top-level objective is proof-complete, review-clean, and committed in the reviewed-slice commit. |
| human_input | Exact user judgment, approval, or interactive observation is required. |
| external_blocker | Named state outside the repository prevents progress and has an exact recheck condition. |
| authority_boundary | Broader scope, Resolution, or decision authority is required. |
| context_boundary | Current top-level context/runtime capacity cannot safely finish the active loop. |
| capability_unavailable | Mandatory built-in subagent execution is unavailable; no external/self-review fallback is allowed. |

Local remediation, a child packet, review findings, cross-document repair, or a local proof gap is not a top-level stop reason by itself.

## Status model

| Status | Meaning |
|---|---|
| completed | The selected top-level objective and all local gates finished review-clean. |
| partial | Safe work remains in the same authority but a genuine top-level resume boundary was reached. |
| blocked | A concrete external/capability dependency prevents progress. |
| escalation_required | Broader scope, authority, Resolution, contract, or planning decision is required. |
| review_required | Historical status; do not use merely because a delegable built-in review has not yet been dispatched. |
| superseded | A later durable record replaces this record's recommendation or facts. |

Do not use completed to claim a broader phase/seam than the selected packet proved.

## Finding and escalation behavior

| Classification | Parent behavior |
|---|---|
| local_remediation | Fix within current authority, verify, and obtain fresh review. |
| child_packet_required | Create an independently reviewable child and execute it internally; keep the parent slice open. |
| cross_document_repair | Pause behavior-changing implementation, repair coupled authority docs, review them, then resume. |
| resolution_escalation | Stop only when the broader decision cannot be resolved inside current authorization. |
| external_blocker | Stop only when named external/human state prevents further work. |
| proof_gap | Dispatch bounded proof/review work internally and reconcile it. |
| future_program | Record the disposition and continue current authorized work. |

Creating a child packet does not complete its parent. An internal agent cannot promote its own finding into program authority.

## Built-in delegated-run protocol

For every proof-relevant internal run, the parent:

1. creates a schema-valid immutable JSON dispatch;
2. fingerprints the dispatch and subject state;
3. spawns a fresh built-in default subagent with isolated context;
4. supplies the exact bounded packet directly through the built-in spawn message;
5. waits using built-in wait/status operations;
6. closes completed or abandoned agents through built-in capabilities when exposed;
7. records agent ID/canonical task name and final built-in status;
8. validates results/findings against live truth;
9. reconciles edits or evidence into the active slice;
10. continues without asking the user to launch the internal dispatch.

Forbidden reviewer transports include shell-managed agents, codex exec, another Codex CLI, background/PTY agents, temporary-file prompt/output transport, filesystem identities, and filesystem polling.

### Review lineage

Review agents are read-only, fresh, and isolated from implementation reasoning and earlier conclusions. Their dispatch requires code-review-and-quality and binds the exact subject fingerprint.

Findings are ordered Critical, Required, Optional, Nit and include file/line, violated contract/gate, reason, smallest valid remediation, and missing proof.

When findings are valid:

1. the parent repairs them or spawns a fresh bounded remediation agent;
2. the remediation run names the findings review in remediation_for_run_ids;
3. verification reruns;
4. a different fresh reviewer receives the new subject fingerprint;
5. the loop repeats until clean or genuinely blocked.

A completed v1.2 record fails semantic validation when a findings review lacks typed successful parent/delegated remediation, delegated remediation is failed/wrong-role, remediation lacks a completed different-fresh re-review of its result fingerprint, a reviewer is reused after remediation, dispatch/result lineage mismatches, the final completed review is not clean, or the final clean review does not bind the replayable reviewed-state manifest/fingerprint. A re-review may discover another findings round; that round must have its own remediation and later fresh re-review before the final clean verdict.

For multi-packet slices, perform bounded packet review as needed and use a different fresh agent for final slice closeout. A single-packet review may serve as final closeout review only when its dispatch covers the full final subject and proof wall.

## Resolution escalation record

Every authority escalation states:

- current and required Resolution/authority horizon;
- trigger and exact missing decision;
- options and tradeoffs;
- recommended option;
- affected pack/spec/task sections;
- whether current work is safe to preserve;
- exact resumption condition.

Needs more context is insufficient.

When authority is reserved to a user/product decision, such as the shipped default artifact set, request that named research/decision boundary. Do not infer approval from current code, historical artifacts, or examples.

## Two-commit true-stop closeout

A handoff cannot contain the hash of the commit that contains itself. Use a deliberate two-commit protocol for a completed top-level slice:

1. Finish implementation/documentation, verification, fresh review, remediation/re-review, proof, and control-pack updates.
2. Run final scoped change detection and diff checks for the reviewed slice state.
3. Commit the reviewed slice state. This is the primary slice commit.
4. Capture/verify the top-level end state and use the same primary slice commit as both `repo_state.head` and `reviewed_state.baseline_head`; completed validation rejects mismatched commit identities.
5. Create one v1.2 parent handoff. Record the final reviewed subject fingerprint, delegated runs, stop reason, proof refs, and source/supersession truth.
6. Rebuild ledger.jsonl from canonical records.
7. Run all handoff/internal-dispatch schemas, historical admission checks, cross-record semantics, ledger parity, and self-tests.
8. Run git diff --check and scoped change detection for the mechanical closeout artifacts.
9. Commit only the new handoff, ledger entry, and any directly required closeout index artifact in a second closeout commit.
10. Report both commit hashes in chat. Do not start the next slice.

The final dispatch may include the pre-closeout `ledger.jsonl` because that file
is part of the reviewed primary state. Completed-record validation replays the
manifest from `reviewed_state.baseline_head`, not from the post-closeout working
tree; exact record/index parity independently validates the rebuilt ledger after
the parent record is added.

For a blocked/partial stop, commit only safe reviewed work when appropriate, then create and commit the handoff separately. Never label an unreviewed or failed state completed.

## Create a v1.2 parent handoff

1. Copy handoff-template.json to a correctly named records/YYYYMMDDTHHMMSSZ--<phase-or-slice>--orchestration--<slug>.json file.
2. Fill every field; remove placeholders.
3. Use source_handoff_ids for consumed resume records.
4. Use supersedes only when replacing prior recommendations/facts.
5. Record the true stop_reason.
6. Record built-in delegation-capability evidence.
7. Record only proof-relevant delegated runs and exact JSON dispatch refs/fingerprints.
8. Bind the final clean review to reviewed_state.subject_fingerprint.
9. Reference snapshots, semantic records, and long evidence rather than embedding content.
10. Prefer repository-relative refs.
11. Rebuild and validate the ledger.

A jq syntax check is optional and never substitutes for schema/semantic validation.

## Ledger entry creation

For one new record:

~~~bash
record="docs/specs/handbook-contract-membrane/handoffs/records/<record>.json"
jq -c --arg record_path "$record" '{
  schema_id: "handbook.handoff-ledger-entry",
  schema_version: "1.0",
  handoff_id,
  created_at_utc,
  status,
  session_kind: .session.kind,
  phase_id,
  slice_id,
  packet_id,
  record_path: $record_path
}' "$record" >> docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
~~~

Before appending, confirm the ID is not indexed:

~~~bash
jq -e -s --arg id '<handoff-id>' 'any(.[]; .handoff_id == $id)'   docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl >/dev/null
~~~

Exit 1 means absent and appendable. Exit 0 means already present.

## Deterministic ledger rebuild

~~~bash
root="docs/specs/handbook-contract-membrane/handoffs"
tmp="$root/ledger.jsonl.tmp"
: > "$tmp"
find "$root/records" -type f -name '*.json' -print   | LC_ALL=C sort   | while IFS= read -r record; do
      jq -c --arg record_path "$record" '{
        schema_id: "handbook.handoff-ledger-entry",
        schema_version: "1.0",
        handoff_id,
        created_at_utc,
        status,
        session_kind: .session.kind,
        phase_id,
        slice_id,
        packet_id,
        record_path: $record_path
      }' "$record"
    done > "$tmp"
mv "$tmp" "$root/ledger.jsonl"
python3 "$root/validate_handoffs.py"
~~~

## Required validation

~~~bash
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
~~~

The normal command validates:

- all Draft 2020-12 handoff/internal-dispatch schemas and current templates;
- exact immutable v1.0/v1.1 and legacy-dispatch admission;
- all canonical records, the hash-admitted internal-dispatch v1.0 review, and current v1.1 JSON dispatches;
- v1.2 source/supersession and delegated-run/dispatch lineage;
- clean-review/finding-remediation/fresh-re-review semantics;
- record/index identity and parity;
- a byte-identical deterministic ledger rebuild.

## Common jq queries

### Latest handoff for the selected slice

~~~bash
slice="HCM-X.Y"
jq -s --arg slice "$slice"   '[.[] | select(.slice_id == $slice)] | sort_by(.created_at_utc) | last'   docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
~~~

### Read the latest full record for the selected slice

~~~bash
slice="HCM-X.Y"
ledger="docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl"
record="$(jq -rs --arg slice "$slice"   '[.[] | select(.slice_id == $slice)] | sort_by(.created_at_utc) | last | .record_path // empty'   "$ledger")"
test -n "$record" && jq . "$record"
~~~

### Select one exact handoff

~~~bash
id="<handoff-id>"
ledger="docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl"
record="$(jq -rs --arg id "$id"   'map(select(.handoff_id == $id)) | last | .record_path // empty' "$ledger")"
test -n "$record" && jq . "$record"
~~~

### Latest escalation for the selected slice

~~~bash
slice="HCM-X.Y"
jq -s --arg slice "$slice"   '[.[] | select(.slice_id == $slice and .status == "escalation_required")] |
   sort_by(.created_at_utc) | last'   docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
~~~

## Top-level orchestration protocol

The parent:

1. receives explicit phase, slice, optional packet, and optional handoff selector;
2. validates dependencies, authorization, worktree state, and selected resume truth;
3. assembles bounded authority/repo/proof context;
4. captures or verifies current grounding state;
5. repairs specification/plan authority when needed;
6. performs or internally delegates the selected work;
7. verifies every packet;
8. executes fresh built-in review and waits for results;
9. validates/remediates findings and executes different-fresh re-review;
10. runs the full proof wall and updates canonical pack truth;
11. commits the reviewed slice state;
12. writes one v1.2 handoff only at a true stop;
13. commits the mechanical closeout record/index;
14. returns a short durable closeout.

If selected handoff facts are stale, do not edit the record. Name it in source_handoff_ids and supersedes only when the new record actually replaces its recommendation/facts.

For artifact/intake/posture work, preserve kind/instance separation, intake candidate versus canonical authority, advisory posture recommendations, and fixed generic CLI operations.

## Short chat closeout

After the parent-owned record and closeout commit exist, return:

~~~text
STATUS: <status>
HANDOFF: <repo-relative parent-owned record path>
SUMMARY: <one or two sentences>
NEXT: <human action, exact top-level resume condition, or none>
READ: jq . <repo-relative record path>
COMMITS: <primary-slice-commit> <closeout-commit>
~~~

Include DISPATCH only for top_level_resume or human_interactive. Never return an internal JSON dispatch as a manual user prompt.

## Failure rules

- If mandatory built-in delegation is unavailable, stop blocked with stop_reason=capability_unavailable; do not self-review or launch an external agent.
- If a reviewer is slow, continue bounded built-in waits; slowness is not capability failure.
- If an internal subagent attempts a global handoff/ledger write, reject that result and remediate the dispatch boundary.
- If an immutable historical record/legacy dispatch is missing or byte-modified, fail closed; never repair by rewriting history or its admission hash without explicit historical-integrity authority.
- If a current JSON dispatch or v1.2 record fails schema/semantic lineage, the top-level run is not closed.
- If ledger and records disagree, rebuild the ledger from canonical records.
- If a snapshot is unstable, retry or record the bounded blocker; do not ground promotion on it.
- If projection exposes sensitive/out-of-Resolution state, omit it and record the omission.
- If scope must broaden beyond current authority, stop at an authority boundary rather than completing extra work.
- If a contract contradiction can be repaired inside current authority, repair/review it internally; do not create a user task hop.
- If the record cannot be written, report the complete closeout in chat as a fallback and state that durable closeout failed.
