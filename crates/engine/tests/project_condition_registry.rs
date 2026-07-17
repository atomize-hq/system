use handbook_engine::{ProjectConditionDefinition, RegistryLoadErrorKind};
use std::path::Path;
const PATH:&str="definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml";
const BYTES:&[u8]=include_bytes!("../definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml");
fn write(p: &Path, b: &[u8]) {
    if let Some(x) = p.parent() {
        std::fs::create_dir_all(x).unwrap();
    }
    std::fs::write(p, b).unwrap();
}
#[test]
fn exact_condition_metadata_loads_without_an_evaluator() {
    let r = tempfile::tempdir().unwrap();
    write(&r.path().join(PATH), BYTES);
    let d = ProjectConditionDefinition::load(r.path(), PATH).unwrap();
    assert_eq!(
        d.exact_ref().as_str(),
        "handbook.condition.project.managed-operational-surface@1.0.0"
    );
}
#[test]
fn condition_drift_and_self_reference_refuse() {
    for field in [
        "minimum_independent_current_bases",
        "definition_fingerprint",
    ] {
        let r = tempfile::tempdir().unwrap();
        let mut v: serde_json::Value = serde_yaml_bw::from_slice(BYTES).unwrap();
        if field == "minimum_independent_current_bases" {
            v[field] = 1u64.into();
            v["self_reference_exclusions"] = serde_json::json!([]);
        } else {
            v[field] =
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into();
        }
        write(
            &r.path().join(PATH),
            serde_yaml_bw::to_string(&v).unwrap().as_bytes(),
        );
        let e = ProjectConditionDefinition::load(r.path(), PATH).unwrap_err();
        assert!(matches!(
            e.kind(),
            RegistryLoadErrorKind::UnsupportedRecord | RegistryLoadErrorKind::FingerprintMismatch
        ));
    }
}
