use handbook_engine::{RegistryLoadErrorKind, VocabularyDefinition};
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
    let vocabulary = VocabularyDefinition::load(r.path(), PATH).unwrap();
    assert_eq!(
        vocabulary.exact_ref().as_str(),
        "handbook.vocabulary.shipped-root@1.0.0"
    );
    assert_eq!(
        vocabulary.stable_role_registry_ref().as_str(),
        "handbook.roles.core@1.1.0"
    );
    assert_eq!(
        vocabulary.stable_role_registry_fingerprint().as_str(),
        "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029"
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

#[test]
fn complete_vocabulary_record_and_nested_selection_matrix_refuses() {
    for case in [
        "missing",
        "extra",
        "wrong_type",
        "unsupported_version",
        "fingerprint",
        "selection_missing",
        "selection_extra",
        "selection_wrong_type",
        "labels_nonempty",
        "aliases_nonempty",
        "absorptions_nonempty",
        "extensions_nonempty",
    ] {
        let r = tempfile::tempdir().unwrap();
        let mut value: serde_json::Value = serde_yaml_bw::from_slice(BYTES).unwrap();
        let expected = match case {
            "missing" => {
                value.as_object_mut().unwrap().remove("vocabulary_id");
                RegistryLoadErrorKind::SyntaxError
            }
            "extra" => {
                value["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "wrong_type" => {
                value["labels"] = serde_json::json!([]);
                RegistryLoadErrorKind::SyntaxError
            }
            "unsupported_version" => {
                value["schema_version"] = serde_json::json!("2.0");
                RegistryLoadErrorKind::UnsupportedRecord
            }
            "fingerprint" => {
                value["vocabulary_fingerprint"] = serde_json::json!(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                );
                RegistryLoadErrorKind::FingerprintMismatch
            }
            "selection_missing" => {
                value["stable_role_registry"]
                    .as_object_mut()
                    .unwrap()
                    .remove("ref");
                RegistryLoadErrorKind::SyntaxError
            }
            "selection_extra" => {
                value["stable_role_registry"]["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "selection_wrong_type" => {
                value["stable_role_registry"]["fingerprint"] = serde_json::json!(true);
                RegistryLoadErrorKind::SyntaxError
            }
            "labels_nonempty" => {
                value["labels"]["project_context"] = serde_json::json!("Renamed");
                RegistryLoadErrorKind::UnsupportedRecord
            }
            "aliases_nonempty" => {
                value["aliases"]["project"] = serde_json::json!("project_context");
                RegistryLoadErrorKind::UnsupportedRecord
            }
            "absorptions_nonempty" => {
                value["absorptions"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!("project_context"));
                RegistryLoadErrorKind::UnsupportedRecord
            }
            _ => {
                value["extensions"]["future"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnsupportedRecord
            }
        };
        write(
            &r.path().join(PATH),
            serde_yaml_bw::to_string(&value).unwrap().as_bytes(),
        );
        let error = VocabularyDefinition::load(r.path(), PATH).unwrap_err();
        assert_eq!(error.kind(), expected, "{case}");
    }
}
