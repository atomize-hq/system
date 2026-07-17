use handbook_engine::{
    ContextResolutionPolicyRegistry, ContextResolutionStackDefinition, RegistryLoadErrorKind,
};
use std::path::Path;
const STACK_PATH: &str =
    "definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml";
const STACK: &[u8] = include_bytes!(
    "../definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml"
);
const POLICY_PATHS: [&str; 3] = [
    "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml",
    "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml",
    "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml",
];
fn write(p: &Path, b: &[u8]) {
    if let Some(x) = p.parent() {
        std::fs::create_dir_all(x).unwrap();
    }
    std::fs::write(p, b).unwrap();
}
#[test]
fn exact_four_level_six_domain_stack_loads_without_policy_execution() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let policies =
        ContextResolutionPolicyRegistry::load(root, &POLICY_PATHS.map(str::to_string)).unwrap();
    let stack = ContextResolutionStackDefinition::load(root, STACK_PATH, &policies).unwrap();
    assert_eq!(
        stack.exact_ref().as_str(),
        "handbook.context-resolution.shipped-root@1.0.0"
    );
}
#[test]
fn rank_default_and_dependency_drift_refuse() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));
    for pointer in [
        "/levels/0/defaults/scope_horizon",
        "/dimension_domains/scope_horizon/0/rank",
        "/mutation_matcher/fingerprint",
    ] {
        let r = tempfile::tempdir().unwrap();
        for p in POLICY_PATHS {
            write(&r.path().join(p), &std::fs::read(source.join(p)).unwrap());
        }
        let mut v: serde_json::Value = serde_yaml_bw::from_slice(STACK).unwrap();
        *v.pointer_mut(pointer).unwrap() = serde_json::json!("drift");
        write(
            &r.path().join(STACK_PATH),
            serde_yaml_bw::to_string(&v).unwrap().as_bytes(),
        );
        let policies =
            ContextResolutionPolicyRegistry::load(r.path(), &POLICY_PATHS.map(str::to_string))
                .unwrap();
        assert!(
            ContextResolutionStackDefinition::load(r.path(), STACK_PATH, &policies).is_err(),
            "{pointer}"
        );
    }
}

#[test]
fn complete_stack_record_and_nested_closure_mutation_matrix_refuses() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));
    for case in [
        "missing",
        "extra",
        "wrong_type",
        "unsupported_version",
        "fingerprint",
        "extensions",
        "level_missing",
        "level_extra",
        "level_wrong_type",
        "defaults_missing",
        "defaults_extra",
        "defaults_wrong_type",
        "domains_missing",
        "ranked_missing",
        "ranked_extra",
        "ranked_wrong_type",
        "ranked_over_bound",
        "selection_missing",
        "selection_extra",
        "selection_wrong_type",
        "levels_over_bound",
        "domain_over_bound",
        "policy_ref_drift",
        "policy_fingerprint_drift",
    ] {
        let r = tempfile::tempdir().unwrap();
        for path in POLICY_PATHS {
            write(
                &r.path().join(path),
                &std::fs::read(source.join(path)).unwrap(),
            );
        }
        let mut value: serde_json::Value = serde_yaml_bw::from_slice(STACK).unwrap();
        let expected = match case {
            "missing" => {
                value.as_object_mut().unwrap().remove("stack_id");
                RegistryLoadErrorKind::SyntaxError
            }
            "extra" => {
                value["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "wrong_type" => {
                value["levels"] = serde_json::json!({});
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
            "extensions" => {
                value["extensions"]["future"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnsupportedRecord
            }
            "level_missing" => {
                value["levels"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("display_label");
                RegistryLoadErrorKind::SyntaxError
            }
            "level_extra" => {
                value["levels"][0]["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "level_wrong_type" => {
                value["levels"][0]["defaults"] = serde_json::json!(false);
                RegistryLoadErrorKind::SyntaxError
            }
            "defaults_missing" => {
                value["levels"][0]["defaults"]
                    .as_object_mut()
                    .unwrap()
                    .remove("scope_horizon");
                RegistryLoadErrorKind::SyntaxError
            }
            "defaults_extra" => {
                value["levels"][0]["defaults"]["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "defaults_wrong_type" => {
                value["levels"][0]["defaults"]["scope_horizon"] = serde_json::json!(true);
                RegistryLoadErrorKind::SyntaxError
            }
            "domains_missing" => {
                value["dimension_domains"]
                    .as_object_mut()
                    .unwrap()
                    .remove("validation_horizon");
                RegistryLoadErrorKind::SyntaxError
            }
            "ranked_missing" => {
                value["dimension_domains"]["scope_horizon"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("rank");
                RegistryLoadErrorKind::SyntaxError
            }
            "ranked_extra" => {
                value["dimension_domains"]["scope_horizon"][0]["unexpected"] =
                    serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "ranked_wrong_type" => {
                value["dimension_domains"]["scope_horizon"][0]["rank"] = serde_json::json!("0");
                RegistryLoadErrorKind::SyntaxError
            }
            "ranked_over_bound" => {
                value["dimension_domains"]["scope_horizon"][0]["rank"] = serde_json::json!(256);
                RegistryLoadErrorKind::SyntaxError
            }
            "selection_missing" => {
                value["mutation_matcher"]
                    .as_object_mut()
                    .unwrap()
                    .remove("ref");
                RegistryLoadErrorKind::SyntaxError
            }
            "selection_extra" => {
                value["mutation_matcher"]["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "selection_wrong_type" => {
                value["mutation_matcher"]["fingerprint"] = serde_json::json!(true);
                RegistryLoadErrorKind::SyntaxError
            }
            "levels_over_bound" => {
                let extra = value["levels"][0].clone();
                value["levels"].as_array_mut().unwrap().push(extra);
                RegistryLoadErrorKind::UnsupportedRecord
            }
            "domain_over_bound" => {
                value["dimension_domains"]["scope_horizon"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!({"value_id":"extra","rank":4}));
                RegistryLoadErrorKind::UnsupportedRecord
            }
            "policy_ref_drift" => {
                value["mutation_matcher"]["ref"] =
                    serde_json::json!("handbook.mutation-matcher.other@1.0.0");
                RegistryLoadErrorKind::UnsupportedRecord
            }
            _ => {
                value["mutation_matcher"]["fingerprint"] = serde_json::json!(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                );
                RegistryLoadErrorKind::FingerprintMismatch
            }
        };
        write(
            &r.path().join(STACK_PATH),
            serde_yaml_bw::to_string(&value).unwrap().as_bytes(),
        );
        let policies =
            ContextResolutionPolicyRegistry::load(r.path(), &POLICY_PATHS.map(str::to_string))
                .unwrap();
        let error =
            ContextResolutionStackDefinition::load(r.path(), STACK_PATH, &policies).unwrap_err();
        assert_eq!(error.kind(), expected, "{case}");
    }
}
