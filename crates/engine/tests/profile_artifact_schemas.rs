use handbook_engine::{ExactDefinitionRef, SchemaRegistry};
use serde_json::{json, Value};
use std::path::Path;

const SCHEMA_ROOT: &str = "definitions/schemas";
const PROJECT_AUTHORITY_ID: &str = "handbook.schemas.artifacts.project-authority";
const PROJECT_AUTHORITY_ENTRY: &str =
    "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml";
const PROJECT_AUTHORITY_DOCUMENT: &str =
    "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json";
const PROJECT_AUTHORITY_ENTRY_BYTES: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml"
);
const PROJECT_AUTHORITY_SCHEMA_BYTES: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json"
);

fn write(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, bytes).unwrap();
}

fn load_schema(
    identity: &str,
    entry_path: &str,
    entry_bytes: &[u8],
    document_path: &str,
    document_bytes: &[u8],
) -> (tempfile::TempDir, handbook_engine::ResolvedSchema) {
    let repo = tempfile::tempdir().unwrap();
    write(&repo.path().join(entry_path), entry_bytes);
    write(&repo.path().join(document_path), document_bytes);
    let registry = SchemaRegistry::load(repo.path(), &[entry_path.into()], &[SCHEMA_ROOT.into()])
        .expect("packaged schema closure");
    let schema_ref = ExactDefinitionRef::parse(&format!("{identity}@1.0.0")).unwrap();
    let resolved = registry.resolved(&schema_ref).unwrap().clone();
    (repo, resolved)
}

fn project_authority() -> Value {
    json!({
        "schema_id": "handbook.artifact.project-authority",
        "schema_version": "1.0",
        "record_id": "example.record.project-authority",
        "policy": {
            "revision": "1",
            "authority_statement": "The project authority is explicit."
        },
        "governance": {
            "decision_authority": ["Owner"],
            "required_approvals": ["Owner"],
            "exception_policy": "Exceptions require explicit approval.",
            "review_triggers": ["Authority changes"],
            "reassessment_triggers": ["Scope changes"]
        },
        "engineering_posture": {
            "dimensions": ["Reliability"],
            "red_lines": ["No silent authority mutation"]
        }
    })
}

#[test]
fn project_authority_schema_is_complete_closed_and_bounded() {
    let (_repo, schema) = load_schema(
        PROJECT_AUTHORITY_ID,
        PROJECT_AUTHORITY_ENTRY,
        PROJECT_AUTHORITY_ENTRY_BYTES,
        PROJECT_AUTHORITY_DOCUMENT,
        PROJECT_AUTHORITY_SCHEMA_BYTES,
    );
    let valid = project_authority();
    assert!(schema.validate_json(&valid).is_ok());

    for pointer in [
        "/policy/revision",
        "/policy/authority_statement",
        "/governance/decision_authority",
        "/governance/required_approvals",
        "/governance/exception_policy",
        "/governance/review_triggers",
        "/governance/reassessment_triggers",
        "/engineering_posture/dimensions",
        "/engineering_posture/red_lines",
    ] {
        let mut changed = valid.clone();
        if let Some(slot) = changed.pointer_mut(pointer) {
            *slot = Value::Null;
        }
        assert!(schema.validate_json(&changed).is_err(), "{pointer}");
    }

    let mut extra = valid.clone();
    extra["policy"]["extra"] = json!(true);
    assert!(schema.validate_json(&extra).is_err());

    let mut duplicate = valid.clone();
    duplicate["governance"]["decision_authority"] = json!(["Owner", "Owner"]);
    assert!(schema.validate_json(&duplicate).is_err());

    let mut short_n = valid.clone();
    short_n["policy"]["revision"] = json!("a".repeat(256));
    assert!(schema.validate_json(&short_n).is_ok());
    short_n["policy"]["revision"] = json!("a".repeat(257));
    assert!(schema.validate_json(&short_n).is_err());

    let mut long_n = valid.clone();
    long_n["policy"]["authority_statement"] = json!("a".repeat(8192));
    assert!(schema.validate_json(&long_n).is_ok());
    long_n["policy"]["authority_statement"] = json!("a".repeat(8193));
    assert!(schema.validate_json(&long_n).is_err());

    let mut list_n = valid;
    list_n["governance"]["decision_authority"] = Value::Array(
        (0..64)
            .map(|index| json!(format!("Owner {index}")))
            .collect(),
    );
    assert!(schema.validate_json(&list_n).is_ok());
    list_n["governance"]["decision_authority"]
        .as_array_mut()
        .unwrap()
        .push(json!("Owner 64"));
    assert!(schema.validate_json(&list_n).is_err());
}
