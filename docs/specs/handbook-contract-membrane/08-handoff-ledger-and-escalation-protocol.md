# Handoff Ledger and Escalation Protocol

## Purpose

Every session leaves a concise, machine-readable durable record so the user does not have to copy a large closeout report into a program-level orchestration session.

The protocol supports:

- normal completion;
- safe continuation;
- blockers;
- Resolution/authority escalation;
- documentation repair;
- finding-driven decomposition;
- review/proof follow-up;
- orchestration dispatch of the next bounded session.

## Storage model

```text
handoffs/
├── handoff-record.schema.json
├── ledger-entry.schema.json
├── handoff-template.json
├── dispatch-template.md
├── ledger.jsonl
├── records/
│   └── <immutable-handoff-records>.json
└── dispatches/
    └── <orchestration-generated-prompts>.md
```

### Canonical truth

- Each file under `records/` is an immutable canonical handoff record.
- `ledger.jsonl` is a rebuildable append-only query index, not a second authority.
- Each dispatch under `dispatches/` is an immutable ready-to-run next-session prompt.
- Corrections create a new handoff record whose `supersedes` field references the prior record.

Per-record files avoid one large mutable JSON document and reduce worktree/merge conflicts.

## File naming

Use UTC and sortable names:

```text
records/YYYYMMDDTHHMMSSZ--<phase-or-slice>--<session-kind>--<short-slug>.json
dispatches/YYYYMMDDTHHMMSSZ--<phase-or-slice>--<short-slug>.md
```

`handoff_id` should match the record filename without `.json`.

## Required session statuses

| Status | Meaning |
|---|---|
| `completed` | All authorized deliverables and local gates finished. Parent/next work may still remain. |
| `partial` | Work is incomplete but can safely resume inside the same authority and packet. |
| `blocked` | Work cannot proceed because a concrete dependency, environment, or required input is unavailable; broader design authority is not necessarily required. |
| `escalation_required` | Continuing would require broader scope, authority, Resolution, contract, or planning changes. |
| `review_required` | Authorized implementation/docs are complete, but the required independent review or proof wall has not completed. |
| `superseded` | A later durable record replaces this session's recommendation or facts. |

Do not use `completed` to mean an entire phase/seam is complete unless the active packet authorized and proved that closeout.

## Finding and escalation classifications

| Classification | Required behavior |
|---|---|
| `local_remediation` | Record and fix only if inside current packet authority. |
| `child_packet_required` | Stop widening; request orchestration decomposition within the current slice. |
| `cross_document_repair` | Stop implementation if authoritative docs/contracts disagree; dispatch a docs repair first. |
| `resolution_escalation` | Name current and required Resolution plus the missing decision/authority. |
| `external_blocker` | Name the external state/human action and the exact recheck condition. |
| `proof_gap` | Keep implementation and proof status distinct; dispatch review/proof work. |
| `future_program` | Record value and reason, but do not add to the current phase map. |

## Resolution escalation record

Every escalation must state:

- current active Resolution envelope;
- required target Resolution or authority horizon;
- trigger encountered;
- exact decision or context missing;
- options considered;
- recommended option and tradeoff;
- affected pack/spec/task sections;
- whether current work is safe to preserve;
- exact condition that permits resumption.

“Needs more context” is not sufficient.

## Session closeout procedure

1. Re-read the active packet and applicable proof gates.
2. Inspect final git/worktree state.
3. Separate completed work, unresolved findings, proof, and assumptions.
4. Copy `handoff-template.json` to a correctly named file under `records/`.
5. Fill every required field and delete placeholder content.
6. Prefer repository-relative evidence/file references.
7. Keep logs and long reports in referenced artifacts; do not embed them in the record.
8. Validate the record:

   ```bash
   jq empty docs/specs/handbook-contract-membrane/handoffs/records/<record>.json
   ```

9. Append/rebuild the ledger index.
10. Verify the new index entry resolves back to the record.
11. Return only the short chat closeout defined below.

## Ledger entry creation

For one new record:

```bash
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
```

Before appending, confirm the handoff ID is not already indexed:

```bash
jq -e -s --arg id '<handoff-id>' 'any(.[]; .handoff_id == $id)' \
  docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl >/dev/null
```

An exit status of `1` means the ID is not present and may be appended. Exit status `0` means the ID already exists and must not be appended again.

## Rebuild the ledger index

If the index is stale, rebuild it from canonical records:

```bash
root="docs/specs/handbook-contract-membrane/handoffs"
tmp="$root/ledger.jsonl.tmp"
: > "$tmp"
find "$root/records" -type f -name '*.json' -print \
  | LC_ALL=C sort \
  | while IFS= read -r record; do
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
```

## Common `jq` queries

### Latest handoff index entry

```bash
jq -s 'sort_by(.created_at_utc) | last' \
  docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
```

### Read the latest full record

```bash
ledger="docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl"
record="$(jq -rs 'sort_by(.created_at_utc) | last | .record_path // empty' "$ledger")"
test -n "$record" && jq . "$record"
```

### Select one exact handoff

```bash
id="<handoff-id>"
ledger="docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl"
record="$(jq -rs --arg id "$id" 'map(select(.handoff_id == $id)) | last | .record_path // empty' "$ledger")"
test -n "$record" && jq . "$record"
```

### Latest escalation

```bash
jq -s '[.[] | select(.status == "escalation_required")] | sort_by(.created_at_utc) | last' \
  docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
```

### Records requiring pack updates

```bash
find docs/specs/handbook-contract-membrane/handoffs/records -type f -name '*.json' -print \
  | LC_ALL=C sort \
  | while IFS= read -r record; do
      jq -c 'select(any(.pack_updates[]?; .status != "completed")) | {
        handoff_id,
        status,
        pack_updates
      }' "$record"
    done
```

## Orchestration protocol

The orchestration session:

1. selects latest or exact handoff;
2. validates record/index consistency;
3. reads only named pack sections and live files;
4. rechecks drift-prone claims;
5. chooses one next-action classification;
6. repairs/decomposes/escalates documentation if required;
7. writes one dispatch artifact;
8. writes its own orchestration handoff;
9. returns a short chat summary.

If the handoff is stale, the orchestrator does not edit it. It writes a superseding record.

## Dispatch requirements

Every dispatch must be runnable without copying the prior chat transcript.

It includes:

- source handoff and orchestration decision;
- exact objective and session kind;
- active Resolution envelope;
- authority order and must-read sections;
- current repo-truth statement;
- allowed scope and explicit non-goals;
- tasks/deliverables;
- contracts/proof gates;
- stop/escalation conditions;
- durable closeout instructions.

The dispatch should remain bounded. It is not a new catch-all summary of the program.

## Short chat closeout

After writing the durable record, return only:

```text
STATUS: <status>
HANDOFF: <repo-relative record path>
SUMMARY: <one or two sentences>
NEXT: <recommended session kind or immediate action>
READ: jq . <repo-relative record path>
```

For orchestration sessions also include:

```text
DISPATCH: <repo-relative dispatch path>
```

Do not paste the full findings, command output, or next-session prompt into chat unless durable file writing was impossible.

## Failure rules

- If the record cannot be written, report that explicitly and include the complete closeout in chat as a fallback.
- If `jq` validation fails, the session is not closed.
- If ledger index and record disagree, rebuild the index from records.
- If live truth contradicts the handoff, write a superseding record before dispatch.
- If scope must broaden, stop and escalate; do not “finish one extra thing.”
- If documentation repair is required, dispatch that repair and return to implementation only after a new handoff confirms consistency.
