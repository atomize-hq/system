---
contract_id: C-13
seam_id: SEAM-4
owner_seam: SEAM-4
version: m5-v1
currentness: current
status: drafted
revalidation_triggers:
  - Any change to the rule that repo-local `.system/*` remains canonical project truth and `artifacts/*` remains derived.
  - Any change to the supported producer command or the named downstream consumer for this bundle.
  - Any change to the emitted bundle path, required files, or required trust metadata.
  - Any change to the trust-class enum or to the classification of `artifacts/feature_spec/FEATURE_SPEC.md`.
  - Any change to the rule that the consumer reads only the emitted bundle unless an explicit fallback condition is declared and logged.
  - Any change that widens `M5` from one named adoption flow into canonical promotion or a multi-consumer framework.
---

# C-13 Pipeline Handoff And Downstream Trust

## Purpose

`C-13` defines the derived downstream handoff bundle and trust model for:

- `system pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer`

It exists so `M5` can prove one real downstream adoption flow from compiler outputs without treating derived artifacts as canonical project truth and without leaving the consumer's trust boundary informal.

## Canonical Location

- Canonical artifact: `docs/contracts/C-13-pipeline-handoff-and-downstream-trust.md`
- Producer command: `system pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer`
- Named downstream consumer: `feature-slice-decomposer`

## Owned Surface

`C-13` is authoritative about:

- the authority boundary between canonical `.system/*` truth and derived `artifacts/*` handoff outputs
- the required bundle layout under `artifacts/handoff/feature_slice/<feature-id>/`
- the trust classes attached to each file exposed to the named consumer
- the allowlisted read set for the named consumer
- the fallback rule for any read outside the emitted bundle

`C-13` is not a canonical-input contract. It does not promote emitted artifacts into canonical truth, and it does not define a generic multi-consumer framework.

## Normative Rules

### Authority boundary

- Repo-local `.system/*` remains the canonical project-truth surface.
- `artifacts/*` remains derived, including `artifacts/handoff/**`.
- The handoff bundle is a derived trust surface for one downstream adoption proof. It MUST NOT be treated as canonical truth.
- `M5` proves one real downstream adoption flow. It MUST NOT use this bundle to justify canonical promotion or a generic consumer platform.

### Emission scope

- `C-13` applies only to:
  - pipeline id `pipeline.foundation_inputs`
  - consumer id `feature-slice-decomposer`
- Any additional consumer, pipeline id, or bundle family requires a contract revision or successor contract.

### Required bundle layout

The emitted bundle MUST live at:

- `artifacts/handoff/feature_slice/<feature-id>/`

The emitted bundle MUST contain:

- `handoff_manifest.json`
- `trust_matrix.md`
- `read_allowlist.json`
- `scorecard/*`

These files are derived outputs. Their presence does not change the canonical-truth rules in `C-03`.

### Trust classes

The trust-class enum for this bundle MUST be exactly:

- `canonical`
- `compiler_derived`
- `external_manual_derived`

Trust classification rules:

- repo-local `.system/*` sources referenced by the bundle MUST be labeled `canonical`
- compiler-emitted derived artifacts included in the bundle MUST be labeled `compiler_derived`
- stage-10 `artifacts/feature_spec/FEATURE_SPEC.md` MUST be labeled `external_manual_derived`

The consumer MUST validate trust class per file from emitted trust metadata before treating any file as admissible input.

### Required handoff metadata

`handoff_manifest.json` MUST identify, at minimum:

- the emitting command and pipeline id
- the named consumer id, `feature-slice-decomposer`
- the feature id / bundle root
- producer version
- the canonical `.system/*` sources the bundle was derived from, with fingerprints
- the derived artifact sources exposed to the consumer, with fingerprints
- route-basis provenance sufficient to bind the bundle to the upstream pipeline result
- the declared fallback conditions for any read outside the emitted bundle

`trust_matrix.md` MUST present the emitted file set together with each file's trust class in a reviewable, deterministic form.

### Allowlisted reads and fallback

- `read_allowlist.json` MUST enumerate the exact emitted-bundle files the named consumer may read on the happy path.
- The named consumer MUST read only the emitted bundle by default.
- Any read outside the emitted bundle is forbidden unless an explicit fallback condition is:
  - declared in emitted handoff metadata, and
  - logged when the fallback is exercised
- An undeclared repo reread outside the emitted bundle is a contract violation and fails the `M5` adoption proof.

### Scope boundary

- `C-13` proves one named consumer adoption flow for `M5`.
- `C-13` does not make `artifacts/*` canonical.
- `C-13` does not authorize raw `artifacts/*` reads outside the emitted bundle.
- `C-13` does not define a multi-consumer adapter framework.

## Verification Checklist

- [ ] `.system/*` remains canonical project truth.
- [ ] `artifacts/*`, including `artifacts/handoff/**`, remain derived.
- [ ] The emitted bundle path is `artifacts/handoff/feature_slice/<feature-id>/`.
- [ ] The emitted bundle contains `handoff_manifest.json`, `trust_matrix.md`, `read_allowlist.json`, and `scorecard/*`.
- [ ] Trust classes are exactly `canonical`, `compiler_derived`, and `external_manual_derived`.
- [ ] `artifacts/feature_spec/FEATURE_SPEC.md` remains `external_manual_derived`.
- [ ] The named consumer is exactly `feature-slice-decomposer`.
- [ ] The consumer may read only the emitted bundle unless an explicit fallback condition is declared and logged.
- [ ] `M5` remains one real downstream adoption proof, not canonical promotion and not a multi-consumer framework.
