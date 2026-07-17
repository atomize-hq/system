use handbook_engine::{
    ExactDefinitionRef, ProjectConditionDefinition, ProjectConditionRegistry, RegistryLoadErrorKind,
};
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

#[test]
fn complete_condition_record_mutation_matrix_refuses_before_acceptance() {
    for case in [
        "missing",
        "extra",
        "wrong_type",
        "unsupported_version",
        "fingerprint",
        "over_bound",
        "nested_wrong_type",
        "extra_literal_member",
    ] {
        let r = tempfile::tempdir().unwrap();
        let mut value: serde_json::Value = serde_yaml_bw::from_slice(BYTES).unwrap();
        let expected = match case {
            "missing" => {
                value.as_object_mut().unwrap().remove("effects");
                RegistryLoadErrorKind::SyntaxError
            }
            "extra" => {
                value["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "wrong_type" => {
                value["freshness_requirement"] = serde_json::json!(false);
                RegistryLoadErrorKind::SyntaxError
            }
            "unsupported_version" => {
                value["schema_version"] = serde_json::json!("2.0");
                RegistryLoadErrorKind::UnsupportedRecord
            }
            "fingerprint" => {
                value["definition_fingerprint"] = serde_json::json!(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                );
                RegistryLoadErrorKind::FingerprintMismatch
            }
            "over_bound" => {
                value["minimum_independent_current_bases"] = serde_json::json!(256);
                RegistryLoadErrorKind::SyntaxError
            }
            "nested_wrong_type" => {
                value["outcomes"][0] = serde_json::json!(true);
                RegistryLoadErrorKind::SyntaxError
            }
            _ => {
                value["effects"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!("unexpected_effect"));
                RegistryLoadErrorKind::UnsupportedRecord
            }
        };
        write(
            &r.path().join(PATH),
            serde_yaml_bw::to_string(&value).unwrap().as_bytes(),
        );
        let error = ProjectConditionDefinition::load(r.path(), PATH).unwrap_err();
        assert_eq!(error.kind(), expected, "{case}");
    }
}

#[test]
fn typed_condition_registry_loads_and_indexes_the_exact_definition() {
    let repository = tempfile::tempdir().unwrap();
    write(&repository.path().join(PATH), BYTES);

    let registry = ProjectConditionRegistry::load(repository.path(), &[PATH.to_owned()]).unwrap();
    let exact_ref =
        ExactDefinitionRef::parse("handbook.condition.project.managed-operational-surface@1.0.0")
            .unwrap();

    assert_eq!(
        registry.refs(),
        std::collections::BTreeSet::from([exact_ref.clone()])
    );
    assert_eq!(
        registry.definition(&exact_ref).unwrap().exact_ref(),
        &exact_ref
    );
}

#[test]
fn duplicate_condition_identity_refuses_stably_in_both_source_orders() {
    let repository = tempfile::tempdir().unwrap();
    let first = "definitions/project-conditions/first.yaml";
    let second = "definitions/project-conditions/second.yaml";
    write(&repository.path().join(first), BYTES);
    write(&repository.path().join(second), BYTES);

    for paths in [
        vec![first.to_owned(), second.to_owned()],
        vec![second.to_owned(), first.to_owned()],
    ] {
        let error = ProjectConditionRegistry::load(repository.path(), &paths).unwrap_err();
        assert_eq!(error.kind(), RegistryLoadErrorKind::DuplicateIdentity);
    }
}

#[test]
fn condition_registry_refuses_wrong_record_and_stale_fingerprint_sources() {
    for case in ["wrong_record", "stale_fingerprint"] {
        let repository = tempfile::tempdir().unwrap();
        let mut value: serde_json::Value = serde_yaml_bw::from_slice(BYTES).unwrap();
        let expected = match case {
            "wrong_record" => {
                value["schema_id"] = serde_json::json!("handbook.vocabulary-definition");
                RegistryLoadErrorKind::UnsupportedRecord
            }
            _ => {
                value["definition_fingerprint"] = serde_json::json!(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                );
                RegistryLoadErrorKind::FingerprintMismatch
            }
        };
        write(
            &repository.path().join(PATH),
            serde_yaml_bw::to_string(&value).unwrap().as_bytes(),
        );

        let error =
            ProjectConditionRegistry::load(repository.path(), &[PATH.to_owned()]).unwrap_err();
        assert_eq!(error.kind(), expected, "{case}");
    }
}
