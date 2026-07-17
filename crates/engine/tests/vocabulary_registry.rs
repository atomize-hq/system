use handbook_engine::VocabularyDefinition;
use std::path::Path;
const PATH: &str = "definitions/vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml";
const BYTES: &[u8] =
    include_bytes!("../definitions/vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml");
fn write(p: &Path, b: &[u8]) {
    if let Some(x) = p.parent() {
        std::fs::create_dir_all(x).unwrap();
    }
    std::fs::write(p, b).unwrap();
}
#[test]
fn exact_empty_mapping_vocabulary_loads() {
    let r = tempfile::tempdir().unwrap();
    write(&r.path().join(PATH), BYTES);
    assert_eq!(
        VocabularyDefinition::load(r.path(), PATH)
            .unwrap()
            .exact_ref()
            .as_str(),
        "handbook.vocabulary.shipped-root@1.0.0"
    );
}
#[test]
fn changed_mapping_or_role_pair_refuses() {
    for field in ["labels", "stable_role_registry"] {
        let r = tempfile::tempdir().unwrap();
        let mut v: serde_json::Value = serde_yaml_bw::from_slice(BYTES).unwrap();
        if field == "labels" {
            v[field] = serde_json::json!({"project_context":"Renamed"});
        } else {
            v[field]["ref"] = "handbook.roles.core@1.0.0".into();
        }
        write(
            &r.path().join(PATH),
            serde_yaml_bw::to_string(&v).unwrap().as_bytes(),
        );
        assert!(VocabularyDefinition::load(r.path(), PATH).is_err());
    }
}
