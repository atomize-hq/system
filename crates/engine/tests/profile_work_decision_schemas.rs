use handbook_engine::{ExactDefinitionRef, SchemaRegistry};
use serde_json::{json, Value};
use std::path::Path;

fn write(path: &Path, bytes: &[u8]) {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).unwrap();
    }
    std::fs::write(path, bytes).unwrap();
}
fn load(slug: &str, entry: &[u8], schema: &[u8]) -> handbook_engine::ResolvedSchema {
    let repo = tempfile::tempdir().unwrap();
    let base = format!("definitions/schemas/{slug}/1.0.0");
    write(&repo.path().join(format!("{base}.entry.yaml")), entry);
    write(&repo.path().join(format!("{base}.schema.json")), schema);
    let registry = SchemaRegistry::load(
        repo.path(),
        &[format!("{base}.entry.yaml")],
        &["definitions/schemas".into()],
    )
    .unwrap();
    registry
        .resolved(&ExactDefinitionRef::parse(&format!("{slug}@1.0.0")).unwrap())
        .unwrap()
        .clone()
}
const WORK: &str = "handbook.schemas.artifacts.work-specification";
const DECISION: &str = "handbook.schemas.artifacts.decision-record";
const WORK_ENTRY: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.entry.yaml"
);
const WORK_SCHEMA: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.schema.json"
);
const DECISION_ENTRY: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.entry.yaml"
);
const DECISION_SCHEMA: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json"
);

#[test]
fn work_specification_shape_is_exact_but_not_selection_authority() {
    let schema = load(WORK, WORK_ENTRY, WORK_SCHEMA);
    let valid = json!({"schema_id":"handbook.artifact.work-specification","schema_version":"1.0","record_id":"example.record.work","objective":"Ship safely","scope":["Engine"],"non_goals":[],"acceptance_criteria":["Tests pass"],"status":"approved"});
    assert!(schema.validate_json(&valid).is_ok());
    for field in [
        "objective",
        "scope",
        "non_goals",
        "acceptance_criteria",
        "status",
    ] {
        let mut v = valid.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(schema.validate_json(&v).is_err(), "{field}");
    }
    for status in [
        "draft",
        "review_ready",
        "approved",
        "active",
        "completed",
        "cancelled",
    ] {
        let mut v = valid.clone();
        v["status"] = json!(status);
        assert!(schema.validate_json(&v).is_ok());
    }
    let mut bad = valid.clone();
    bad["status"] = json!("pending");
    assert!(schema.validate_json(&bad).is_err());
    let mut duplicate = valid.clone();
    duplicate["scope"] = json!(["Engine", "Engine"]);
    assert!(schema.validate_json(&duplicate).is_err());
    let mut authority = valid;
    authority["selected_by_default"] = json!(true);
    assert!(schema.validate_json(&authority).is_err());
}

#[test]
fn decision_record_shape_is_exact_bounded_and_closed() {
    let schema = load(DECISION, DECISION_ENTRY, DECISION_SCHEMA);
    let valid = json!({"schema_id":"handbook.artifact.decision-record","schema_version":"1.0","record_id":"example.record.decision","context":"A choice exists","decision":"Choose safety","status":"accepted","consequences":["More checks"],"supersedes":[]});
    assert!(schema.validate_json(&valid).is_ok());
    for field in [
        "context",
        "decision",
        "status",
        "consequences",
        "supersedes",
    ] {
        let mut v = valid.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(schema.validate_json(&v).is_err(), "{field}");
    }
    for status in ["proposed", "accepted", "superseded", "withdrawn"] {
        let mut v = valid.clone();
        v["status"] = json!(status);
        assert!(schema.validate_json(&v).is_ok());
    }
    let mut bad = valid.clone();
    bad["status"] = json!("approved");
    assert!(schema.validate_json(&bad).is_err());
    let mut long = valid.clone();
    long["decision"] = json!("x".repeat(8192));
    assert!(schema.validate_json(&long).is_ok());
    long["decision"] = json!("x".repeat(8193));
    assert!(schema.validate_json(&long).is_err());
    let mut refs = valid;
    refs["supersedes"] = Value::Array(
        (0..128)
            .map(|i| json!(format!("example.record.d{i}")))
            .collect(),
    );
    assert!(schema.validate_json(&refs).is_ok());
    refs["supersedes"]
        .as_array_mut()
        .unwrap()
        .push(json!("example.record.overflow"));
    assert!(schema.validate_json(&refs).is_err());
}
