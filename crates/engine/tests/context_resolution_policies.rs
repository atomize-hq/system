use handbook_engine::{ContextResolutionPolicyRegistry, RegistryLoadErrorKind};
use std::collections::BTreeSet;
use std::path::Path;
const ITEMS:[(&str,&[u8]);3]=[("definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml",include_bytes!("../definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml")),("definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml",include_bytes!("../definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml")),("definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml",include_bytes!("../definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml"))];
fn write(p: &Path, b: &[u8]) {
    if let Some(x) = p.parent() {
        std::fs::create_dir_all(x).unwrap();
    }
    std::fs::write(p, b).unwrap();
}
#[test]
fn exact_three_typed_policy_producers_load_without_execution() {
    let r = tempfile::tempdir().unwrap();
    for (p, b) in ITEMS {
        write(&r.path().join(p), b);
    }
    let registry =
        ContextResolutionPolicyRegistry::load(r.path(), &ITEMS.map(|x| x.0.to_string())).unwrap();
    assert_eq!(
        registry
            .refs()
            .iter()
            .map(|r| r.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "handbook.memory-promotion.core@1.0.0",
            "handbook.mutation-matcher.core@1.0.0",
            "handbook.resolution-escalation.core@1.0.0"
        ])
    );
}
#[test]
fn matcher_or_policy_drift_refuses() {
    for (field, value) in [
        ("deny_precedence", serde_json::json!(false)),
        ("max_segments", serde_json::json!(65)),
    ] {
        let r = tempfile::tempdir().unwrap();
        for (p, b) in ITEMS {
            write(&r.path().join(p), b);
        }
        let mut v: serde_json::Value = serde_yaml_bw::from_slice(ITEMS[0].1).unwrap();
        if field == "max_segments" {
            v["selector_grammar"][field] = value
        } else {
            v[field] = value
        }
        write(
            &r.path().join(ITEMS[0].0),
            serde_yaml_bw::to_string(&v).unwrap().as_bytes(),
        );
        assert!(
            ContextResolutionPolicyRegistry::load(r.path(), &ITEMS.map(|x| x.0.to_string()))
                .is_err()
        );
    }
}

#[test]
fn complete_matcher_escalation_and_promotion_mutation_matrices_refuse() {
    for (producer, item) in ITEMS.iter().enumerate() {
        let cases: &[&str] = if producer == 0 {
            &[
                "missing",
                "extra",
                "wrong_type",
                "unsupported_version",
                "fingerprint",
                "extensions",
                "array_wrong_type",
                "over_bound",
                "nested_missing",
                "nested_extra",
                "nested_wrong_type",
            ]
        } else {
            &[
                "missing",
                "extra",
                "wrong_type",
                "unsupported_version",
                "fingerprint",
                "extensions",
                "array_wrong_type",
                "over_bound",
            ]
        };
        for case in cases {
            let r = tempfile::tempdir().unwrap();
            for (path, bytes) in ITEMS {
                write(&r.path().join(path), bytes);
            }
            let mut value: serde_json::Value = serde_yaml_bw::from_slice(item.1).unwrap();
            let expected = match (*case, producer) {
                ("missing", 0) => {
                    value.as_object_mut().unwrap().remove("case_mode");
                    RegistryLoadErrorKind::SyntaxError
                }
                ("missing", 1) => {
                    value.as_object_mut().unwrap().remove("proposal_relation");
                    RegistryLoadErrorKind::SyntaxError
                }
                ("missing", _) => {
                    value.as_object_mut().unwrap().remove("source_requirement");
                    RegistryLoadErrorKind::SyntaxError
                }
                ("extra", _) => {
                    value["unexpected"] = serde_json::json!(true);
                    RegistryLoadErrorKind::UnknownField
                }
                ("wrong_type", 0) => {
                    value["deny_precedence"] = serde_json::json!("true");
                    RegistryLoadErrorKind::SyntaxError
                }
                ("wrong_type", _) => {
                    value["terminal_cardinality"] = serde_json::json!(true);
                    RegistryLoadErrorKind::SyntaxError
                }
                ("unsupported_version", _) => {
                    value["schema_version"] = serde_json::json!("2.0");
                    RegistryLoadErrorKind::UnsupportedRecord
                }
                ("fingerprint", 0) => {
                    value["definition_fingerprint"] = serde_json::json!(
                        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    );
                    RegistryLoadErrorKind::FingerprintMismatch
                }
                ("fingerprint", _) => {
                    value["policy_fingerprint"] = serde_json::json!(
                        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    );
                    RegistryLoadErrorKind::FingerprintMismatch
                }
                ("extensions", _) => {
                    value["extensions"]["future"] = serde_json::json!(true);
                    RegistryLoadErrorKind::UnsupportedRecord
                }
                ("array_wrong_type", 0) => {
                    value["target_kinds"][0] = serde_json::json!(true);
                    RegistryLoadErrorKind::SyntaxError
                }
                ("array_wrong_type", 1) => {
                    value["trigger_classes"][0] = serde_json::json!(true);
                    RegistryLoadErrorKind::SyntaxError
                }
                ("array_wrong_type", _) => {
                    value["forbidden_authorities"][0] = serde_json::json!(true);
                    RegistryLoadErrorKind::SyntaxError
                }
                ("over_bound", 0) => {
                    value["selector_grammar"]["max_segments"] = serde_json::json!(65);
                    RegistryLoadErrorKind::UnsupportedRecord
                }
                ("over_bound", 1) => {
                    value["trigger_classes"]
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::json!("unexpected_trigger"));
                    RegistryLoadErrorKind::UnsupportedRecord
                }
                ("over_bound", _) => {
                    value["forbidden_authorities"]
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::json!("unexpected_authority"));
                    RegistryLoadErrorKind::UnsupportedRecord
                }
                ("nested_missing", _) => {
                    value["selector_grammar"]
                        .as_object_mut()
                        .unwrap()
                        .remove("encoding");
                    RegistryLoadErrorKind::SyntaxError
                }
                ("nested_extra", _) => {
                    value["selector_grammar"]["unexpected"] = serde_json::json!(true);
                    RegistryLoadErrorKind::UnknownField
                }
                ("nested_wrong_type", _) => {
                    value["selector_grammar"]["max_bytes"] = serde_json::json!("1024");
                    RegistryLoadErrorKind::SyntaxError
                }
                _ => unreachable!(),
            };
            write(
                &r.path().join(item.0),
                serde_yaml_bw::to_string(&value).unwrap().as_bytes(),
            );
            let error = ContextResolutionPolicyRegistry::load(
                r.path(),
                &ITEMS.map(|item| item.0.to_string()),
            )
            .unwrap_err();
            assert_eq!(error.kind(), expected, "producer={producer} case={case}");
        }
    }
}
