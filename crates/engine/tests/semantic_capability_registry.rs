use handbook_engine::{
    BindingCardinality, BindingEmptyPolicy, BindingJsonType, DefinitionFingerprint,
    ExactDefinitionRef, SemanticCapabilityRegistry,
};
use serde_json::Value;
use std::path::Path;
const VALIDATOR_PATH: &str =
    "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml";
const CAPABILITY_PATH: &str =
    "definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml";
const VALIDATOR:&[u8]=include_bytes!("../definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml");
const CAPABILITY: &[u8] = include_bytes!(
    "../definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml"
);
fn write(p: &Path, b: &[u8]) {
    if let Some(x) = p.parent() {
        std::fs::create_dir_all(x).unwrap();
    }
    std::fs::write(p, b).unwrap();
}
fn repo() -> tempfile::TempDir {
    let r = tempfile::tempdir().unwrap();
    write(&r.path().join(VALIDATOR_PATH), VALIDATOR);
    write(&r.path().join(CAPABILITY_PATH), CAPABILITY);
    r
}
#[test]
fn constitutional_contract_and_nine_rules_are_literal_and_acyclic() {
    let r = repo();
    let registry = SemanticCapabilityRegistry::load(
        r.path(),
        &[CAPABILITY_PATH.into()],
        &[VALIDATOR_PATH.into()],
    )
    .unwrap();
    let cref =
        ExactDefinitionRef::parse("handbook.capabilities.constitutional-root@1.0.0").unwrap();
    let vref = ExactDefinitionRef::parse("handbook.semantic-validation.constitutional-root@1.0.0")
        .unwrap();
    let c = registry.capability(&cref).unwrap();
    let v = registry.validator(&vref).unwrap();
    let keys = [
        "policy_root",
        "policy_revision",
        "decision_authority",
        "required_approvals",
        "exception_policy",
        "engineering_posture_dimensions",
        "red_lines",
        "review_triggers",
        "reassessment_triggers",
    ];
    assert_eq!(
        c.required_bindings()
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<_>>(),
        keys
    );
    assert_eq!(c.semantic_validation_profile_refs(), [vref]);
    assert_eq!(v.binding_rules().len(), 9);
    for (index, rule) in v.binding_rules().iter().enumerate() {
        assert_eq!(rule.rule_id().as_str(), keys[index]);
        assert_eq!(rule.binding_key().as_str(), keys[index]);
        assert_eq!(rule.empty_policy(), BindingEmptyPolicy::Forbidden);
        if index == 0 {
            assert_eq!(rule.json_type(), BindingJsonType::Object);
            assert_eq!(rule.cardinality(), BindingCardinality::Singular);
        } else if matches!(index, 2 | 3 | 5 | 6 | 7 | 8) {
            assert_eq!(rule.json_type(), BindingJsonType::Array);
            assert_eq!(rule.cardinality(), BindingCardinality::Plural);
        } else {
            assert_eq!(rule.json_type(), BindingJsonType::String);
            assert_eq!(rule.cardinality(), BindingCardinality::Singular);
        }
    }
}
#[test]
fn changed_validator_invalidates_validator_then_capability() {
    let r = repo();
    let mut value: Value = serde_yaml_bw::from_slice(VALIDATOR).unwrap();
    value["binding_rules"][1]["empty_policy"] = Value::String("allowed".into());
    let preimage = {
        let mut x = value.clone();
        x.as_object_mut().unwrap().remove("profile_fingerprint");
        x
    };
    value["profile_fingerprint"] = Value::String(
        DefinitionFingerprint::from_json_value(&preimage)
            .unwrap()
            .to_string(),
    );
    write(
        &r.path().join(VALIDATOR_PATH),
        serde_yaml_bw::to_string(&value).unwrap().as_bytes(),
    );
    let error = SemanticCapabilityRegistry::load(
        r.path(),
        &[CAPABILITY_PATH.into()],
        &[VALIDATOR_PATH.into()],
    )
    .unwrap_err();
    assert_eq!(
        error.kind(),
        handbook_engine::RegistryLoadErrorKind::FingerprintMismatch
    );
}
#[test]
fn back_edges_unknown_fields_duplicates_and_forged_fingerprints_refuse() {
    for mutation in ["back_edge", "duplicate", "fingerprint"] {
        let r = repo();
        let mut value: Value = serde_yaml_bw::from_slice(VALIDATOR).unwrap();
        match mutation {
            "back_edge" => {
                value["capability_ref"] =
                    Value::String("handbook.capabilities.constitutional-root@1.0.0".into())
            }
            "duplicate" => {
                let first = value["binding_rules"][0].clone();
                value["binding_rules"].as_array_mut().unwrap().push(first)
            }
            _ => {
                value["profile_fingerprint"] = Value::String(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .into(),
                )
            }
        };
        write(
            &r.path().join(VALIDATOR_PATH),
            serde_yaml_bw::to_string(&value).unwrap().as_bytes(),
        );
        assert!(
            SemanticCapabilityRegistry::load(
                r.path(),
                &[CAPABILITY_PATH.into()],
                &[VALIDATOR_PATH.into()]
            )
            .is_err(),
            "{mutation}"
        );
    }
}
