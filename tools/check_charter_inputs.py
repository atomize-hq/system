#!/usr/bin/env python3
"""
check_charter_inputs.py

Schema-ish validator for artifacts/charter/CHARTER_INPUTS.yaml.

Goal: make the "inputs → synthesize" dev/test path regression-testable without any model in the loop.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Tuple

try:
    import yaml  # type: ignore
except ModuleNotFoundError:  # pragma: no cover
    # Local fallback used elsewhere in this repo.
    import yaml_lite as yaml  # type: ignore


ALLOWED_CLASSIFICATIONS = {"greenfield", "brownfield", "integration", "modernization", "hardening"}
ALLOWED_USERS = {"internal", "external", "mixed"}
ALLOWED_EXPECTED_LIFETIME = {"days", "weeks", "months", "years"}
ALLOWED_SURFACES = {"web_app", "api", "cli", "lib", "infra", "ml"}
ALLOWED_RUNTIMES = {"browser", "server", "cloud", "on_prem", "edge"}
ALLOWED_BACKCOMPAT = {"required", "not_required", "boundary_only"}
ALLOWED_MIGRATION = {"required", "not_required"}
ALLOWED_ROLLOUT = {"none", "lightweight", "required"}
ALLOWED_DEPRECATION = {"required", "not_required_yet"}
ALLOWED_OBS = {"minimal", "standard", "high", "regulated"}
ALLOWED_DECISION_FORMATS = {"md", "markdown", "rst", "txt"}


def _is_mapping(v: Any) -> bool:
    return isinstance(v, dict)


def _is_sequence(v: Any) -> bool:
    return isinstance(v, list)


def _path_list(paths: List[Path]) -> str:
    return ", ".join(str(p) for p in paths)


def _fail(errs: List[str], path: Path) -> None:
    for e in errs:
        print(f"[ERROR] {path}: {e}", file=sys.stderr)


def _warn(warns: List[str], path: Path) -> None:
    for w in warns:
        print(f"[WARN]  {path}: {w}", file=sys.stderr)


def _req_map(d: Dict[str, Any], key: str, errs: List[str]) -> Optional[Dict[str, Any]]:
    v = d.get(key)
    if v is None:
        errs.append(f"missing required key '{key}'")
        return None
    if not _is_mapping(v):
        errs.append(f"'{key}' must be a mapping")
        return None
    return v


def _req_list(d: Dict[str, Any], key: str, errs: List[str]) -> Optional[List[Any]]:
    v = d.get(key)
    if v is None:
        errs.append(f"missing required key '{key}'")
        return None
    if not _is_sequence(v):
        errs.append(f"'{key}' must be a list")
        return None
    return v


def _req_str(d: Dict[str, Any], key: str, errs: List[str], allow_empty: bool = False) -> Optional[str]:
    v = d.get(key)
    if v is None:
        errs.append(f"missing required key '{key}'")
        return None
    if not isinstance(v, str):
        errs.append(f"'{key}' must be a string")
        return None
    if (not allow_empty) and v.strip() == "":
        errs.append(f"'{key}' must not be empty")
        return None
    return v


def _opt_str(d: Dict[str, Any], key: str, errs: List[str]) -> Optional[str]:
    v = d.get(key)
    if v is None:
        return None
    if not isinstance(v, str):
        errs.append(f"'{key}' must be a string if provided")
        return None
    return v


def _req_int(d: Dict[str, Any], key: str, errs: List[str], *, min_v: int, max_v: int) -> Optional[int]:
    v = d.get(key)
    if v is None:
        errs.append(f"missing required key '{key}'")
        return None
    if not isinstance(v, int):
        errs.append(f"'{key}' must be an integer")
        return None
    if v < min_v or v > max_v:
        errs.append(f"'{key}' must be between {min_v} and {max_v}")
        return None
    return v


def _opt_level(v: Any, errs: List[str], *, ctx: str) -> Optional[int]:
    if v is None:
        return None
    if not isinstance(v, int):
        errs.append(f"{ctx}: 'level' must be an integer (1-5) or null")
        return None
    if v < 1 or v > 5:
        errs.append(f"{ctx}: 'level' must be between 1 and 5")
        return None
    return v


def _req_enum_str(d: Dict[str, Any], key: str, errs: List[str], allowed: set[str]) -> Optional[str]:
    v = _req_str(d, key, errs)
    if v is None:
        return None
    if v not in allowed:
        errs.append(f"'{key}' must be one of {sorted(allowed)} (got {v!r})")
        return None
    return v


def _opt_enum_str(d: Dict[str, Any], key: str, errs: List[str], allowed: set[str]) -> Optional[str]:
    v = d.get(key)
    if v is None:
        return None
    if not isinstance(v, str):
        errs.append(f"'{key}' must be a string if provided")
        return None
    if v not in allowed:
        errs.append(f"'{key}' must be one of {sorted(allowed)} (got {v!r})")
        return None
    return v


def _list_of_str(v: Any, errs: List[str], *, ctx: str, allow_empty: bool = True) -> Optional[List[str]]:
    if not _is_sequence(v):
        errs.append(f"{ctx} must be a list of strings")
        return None
    out: List[str] = []
    for idx, item in enumerate(v):
        if not isinstance(item, str):
            errs.append(f"{ctx}[{idx}] must be a string")
            continue
        if (not allow_empty) and item.strip() == "":
            errs.append(f"{ctx}[{idx}] must not be empty")
            continue
        out.append(item)
    return out


def validate_charter_inputs(data: Any, *, strict: bool) -> Tuple[List[str], List[str]]:
    errs: List[str] = []
    warns: List[str] = []

    if not _is_mapping(data):
        return ["top-level YAML must be a mapping"], []

    schema_version = data.get("schema_version")
    if schema_version is None:
        errs.append("missing required key 'schema_version'")
    elif not isinstance(schema_version, str):
        errs.append("'schema_version' must be a string (e.g., '0.1.0')")
    elif schema_version != "0.1.0":
        warns.append(f"unexpected schema_version {schema_version!r} (expected '0.1.0')")

    project = _req_map(data, "project", errs)
    posture = _req_map(data, "posture", errs)
    dimensions = data.get("dimensions")
    exceptions = _req_map(data, "exceptions", errs)
    debt_tracking = _req_map(data, "debt_tracking", errs)
    decision_records = _req_map(data, "decision_records", errs)

    domains_raw = data.get("domains")
    if domains_raw is None:
        errs.append("missing required key 'domains'")
        domains: Optional[List[Any]] = None
    elif not _is_sequence(domains_raw):
        errs.append("'domains' must be a list")
        domains = None
    else:
        domains = domains_raw

    if project:
        _req_str(project, "name", errs)
        _req_enum_str(project, "classification", errs, ALLOWED_CLASSIFICATIONS)
        _req_int(project, "team_size", errs, min_v=1, max_v=25)
        _req_enum_str(project, "users", errs, ALLOWED_USERS)
        _req_enum_str(project, "expected_lifetime", errs, ALLOWED_EXPECTED_LIFETIME)

        surfaces = project.get("surfaces")
        if surfaces is None:
            errs.append("project.surfaces is required")
        else:
            ss = _list_of_str(surfaces, errs, ctx="project.surfaces", allow_empty=False)
            if ss is not None:
                bad = [x for x in ss if x not in ALLOWED_SURFACES]
                if bad:
                    errs.append(f"project.surfaces contains unknown values: {bad} (allowed: {sorted(ALLOWED_SURFACES)})")

        runtimes = project.get("runtime_environments")
        if runtimes is None:
            errs.append("project.runtime_environments is required")
        else:
            rr = _list_of_str(runtimes, errs, ctx="project.runtime_environments", allow_empty=False)
            if rr is not None:
                bad = [x for x in rr if x not in ALLOWED_RUNTIMES]
                if bad:
                    errs.append(
                        f"project.runtime_environments contains unknown values: {bad} (allowed: {sorted(ALLOWED_RUNTIMES)})"
                    )

        constraints = _req_map(project, "constraints", errs)
        if constraints:
            for k in ["deadline", "budget", "experience_notes"]:
                _opt_str(constraints, k, errs)
            must_use = constraints.get("must_use_tech", [])
            if must_use is None:
                must_use = []
            _list_of_str(must_use, errs, ctx="project.constraints.must_use_tech", allow_empty=False)

        reality = _req_map(project, "operational_reality", errs)
        if reality:
            if "in_production_today" not in reality or not isinstance(reality.get("in_production_today"), bool):
                errs.append("project.operational_reality.in_production_today must be a boolean")
            _opt_str(reality, "prod_users_or_data", errs)
            contracts = reality.get("external_contracts_to_preserve", [])
            if contracts is None:
                contracts = []
            _list_of_str(contracts, errs, ctx="project.operational_reality.external_contracts_to_preserve", allow_empty=False)
            _opt_str(reality, "uptime_expectations", errs)

        impl = _req_map(project, "default_implications", errs)
        if impl:
            _req_enum_str(impl, "backward_compatibility", errs, ALLOWED_BACKCOMPAT)
            _req_enum_str(impl, "migration_planning", errs, ALLOWED_MIGRATION)
            _req_enum_str(impl, "rollout_controls", errs, ALLOWED_ROLLOUT)
            _req_enum_str(impl, "deprecation_policy", errs, ALLOWED_DEPRECATION)
            _req_enum_str(impl, "observability_threshold", errs, ALLOWED_OBS)

    if posture:
        _req_str(posture, "rubric_scale", errs)
        _req_int(posture, "baseline_level", errs, min_v=1, max_v=5)
        br = posture.get("baseline_rationale")
        if br is None:
            errs.append("posture.baseline_rationale is required")
        else:
            _list_of_str(br, errs, ctx="posture.baseline_rationale", allow_empty=False)

    if domains is not None:
        for di, d in enumerate(domains):
            ctx = f"domains[{di}]"
            if not _is_mapping(d):
                errs.append(f"{ctx} must be a mapping")
                continue
            _req_str(d, "name", errs)
            blast = d.get("blast_radius")
            if blast is None or not isinstance(blast, str) or blast.strip() == "":
                errs.append(f"{ctx}.blast_radius must be a non-empty string")
            touches = d.get("touches", [])
            if touches is None:
                touches = []
            _list_of_str(touches, errs, ctx=f"{ctx}.touches", allow_empty=False)
            constraints = d.get("constraints", [])
            if constraints is None:
                constraints = []
            _list_of_str(constraints, errs, ctx=f"{ctx}.constraints", allow_empty=False)

    if dimensions is None:
        errs.append("missing required key 'dimensions'")
    elif not _is_sequence(dimensions):
        errs.append("'dimensions' must be a list")
    else:
        if strict and len(dimensions) == 0:
            errs.append("'dimensions' must not be empty in --strict mode")
        for i, dim in enumerate(dimensions):
            ctx = f"dimensions[{i}]"
            if not _is_mapping(dim):
                errs.append(f"{ctx} must be a mapping")
                continue
            name = dim.get("name")
            if not isinstance(name, str) or name.strip() == "":
                errs.append(f"{ctx}.name must be a non-empty string")
            _opt_level(dim.get("level"), errs, ctx=ctx)
            ds = dim.get("default_stance")
            if ds is None or not isinstance(ds, str):
                errs.append(f"{ctx}.default_stance must be a string (can be empty)")
            for lk in ["raise_the_bar_triggers", "allowed_shortcuts", "red_lines", "domain_overrides"]:
                lv = dim.get(lk)
                if lv is None:
                    errs.append(f"{ctx}.{lk} is required (can be empty list)")
                    continue
                if lk == "domain_overrides":
                    if not _is_sequence(lv):
                        errs.append(f"{ctx}.{lk} must be a list")
                        continue
                    for oi, ov in enumerate(lv):
                        octx = f"{ctx}.domain_overrides[{oi}]"
                        if not _is_mapping(ov):
                            errs.append(f"{octx} must be a mapping")
                            continue
                        if "domain" not in ov or not isinstance(ov.get("domain"), str) or not ov.get("domain").strip():
                            errs.append(f"{octx}.domain must be a non-empty string")
                        _opt_level(ov.get("level"), errs, ctx=octx)
                        if "note" not in ov or not isinstance(ov.get("note"), str):
                            errs.append(f"{octx}.note must be a string")
                else:
                    _list_of_str(lv, errs, ctx=f"{ctx}.{lk}", allow_empty=False)

    if exceptions:
        appr = exceptions.get("approvers")
        if appr is None:
            errs.append("exceptions.approvers is required")
        else:
            _list_of_str(appr, errs, ctx="exceptions.approvers", allow_empty=False)
        _req_str(exceptions, "record_location", errs)
        mf = exceptions.get("minimum_fields")
        if mf is None:
            errs.append("exceptions.minimum_fields is required")
        else:
            _list_of_str(mf, errs, ctx="exceptions.minimum_fields", allow_empty=False)

    if debt_tracking:
        _req_str(debt_tracking, "system", errs)
        labels = debt_tracking.get("labels")
        if labels is None:
            errs.append("debt_tracking.labels is required")
        else:
            _list_of_str(labels, errs, ctx="debt_tracking.labels", allow_empty=False)
        _req_str(debt_tracking, "review_cadence", errs)

    if decision_records:
        if "enabled" not in decision_records or not isinstance(decision_records.get("enabled"), bool):
            errs.append("decision_records.enabled must be a boolean")
        path = decision_records.get("path")
        if path is None or not isinstance(path, str):
            errs.append("decision_records.path must be a string")
        fmt = decision_records.get("format")
        if fmt is None or not isinstance(fmt, str):
            errs.append("decision_records.format must be a string")
        elif fmt not in ALLOWED_DECISION_FORMATS:
            warns.append(f"decision_records.format {fmt!r} is unusual (allowed: {sorted(ALLOWED_DECISION_FORMATS)})")

    # Warn on unexpected top-level keys (helps catch typos).
    expected_top = {"schema_version", "project", "posture", "domains", "dimensions", "exceptions", "debt_tracking", "decision_records"}
    extra = sorted(set(data.keys()) - expected_top)
    if extra:
        warns.append(f"unexpected top-level keys: {extra}")

    return errs, warns


def _iter_yaml_paths(inputs: List[str]) -> List[Path]:
    out: List[Path] = []
    for raw in inputs:
        p = Path(raw)
        if p.is_dir():
            out.extend(sorted(p.rglob("*.yaml")))
            out.extend(sorted(p.rglob("*.yml")))
            continue
        # Globs
        if any(ch in raw for ch in ["*", "?", "["]):
            out.extend(sorted(Path().glob(raw)))
            continue
        out.append(p)
    # Dedup while preserving order
    seen = set()
    uniq: List[Path] = []
    for p in out:
        rp = str(p.resolve())
        if rp in seen:
            continue
        seen.add(rp)
        uniq.append(p)
    return uniq


def main(argv: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(description="Validate CHARTER_INPUTS.yaml fixtures.")
    parser.add_argument(
        "paths",
        nargs="+",
        help="YAML file(s), directory(ies), or glob(s) to validate (e.g., artifacts/charter/CHARTER_INPUTS.yaml or tools/fixtures/charter_inputs/).",
    )
    parser.add_argument("--strict", action="store_true", help="Fail on empty dimensions and other stricter checks.")
    parser.add_argument("--quiet", action="store_true", help="Only print errors.")
    args = parser.parse_args(argv)

    paths = _iter_yaml_paths(args.paths)
    if not paths:
        print("No YAML files found.", file=sys.stderr)
        return 2

    any_err = False
    for p in paths:
        if not p.exists():
            print(f"[ERROR] missing path: {p}", file=sys.stderr)
            any_err = True
            continue
        text = p.read_text(encoding="utf-8", errors="replace")
        try:
            data = yaml.safe_load(text)
        except Exception as e:
            print(f"[ERROR] {p}: failed to parse YAML: {e}", file=sys.stderr)
            any_err = True
            continue

        errs, warns = validate_charter_inputs(data, strict=bool(args.strict))
        if errs:
            any_err = True
            _fail(errs, p)
        if warns and not args.quiet:
            _warn(warns, p)
        if (not errs) and (not args.quiet):
            print(f"[OK]   {p}")

    return 1 if any_err else 0


if __name__ == "__main__":
    raise SystemExit(main())

