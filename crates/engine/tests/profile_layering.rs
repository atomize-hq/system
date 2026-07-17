use handbook_engine::*;
use serde_json::{json, Value};
fn source(
    id: &str,
    scope: &str,
    parent: Option<&str>,
    fields: serde_json::Map<String, Value>,
) -> Vec<u8> {
    let mut v = json!({"schema_id":"handbook.instance-profile","schema_version":"1.0","profile_id":id,"profile_version":"1.0.0","profile_scope":scope,"extends_profile_ref":parent});
    v.as_object_mut().unwrap().extend(fields);
    let fp = DefinitionFingerprint::from_json_value(&v)
        .unwrap()
        .to_string();
    v["profile_fingerprint"] = fp.into();
    serde_yaml_bw::to_string(&v).unwrap().into_bytes()
}
fn all_fields() -> serde_json::Map<String, Value> {
    serde_json::Map::from_iter([("stable_role_registry",json!({"ref":"handbook.roles.core@1.1.0","fingerprint":"sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029"})),("schema_registry_sources",json!([])),("artifact_kind_sources",json!([])),("artifact_instances",json!([])),("vocabulary_ref",json!("handbook.vocabulary.shipped-root@1.0.0")),("context_resolution_ref",json!("handbook.context-resolution.shipped-root@1.0.0")),("projection_catalog_refs",json!([])),("posture_evaluation_policy",Value::Null),("dock_requirement_refs",json!([])),("adapter_overlay_refs",json!([])),("extensions",json!({}))].map(|(k,v)|(k.to_string(),v)))
}
#[test]
fn all_eleven_fields_inherit_or_replace_whole_with_deterministic_decisions() {
    let root = parse_profile_source(&source(
        "handbook.profile.root",
        "shipped",
        None,
        all_fields(),
    ))
    .unwrap();
    let child = parse_profile_source(&source(
        "handbook.profile.child",
        "repository",
        Some("handbook.profile.root@1.0.0"),
        serde_json::Map::from_iter([
            (
                "schema_registry_sources".into(),
                json!(["example.schema@1.0.0"]),
            ),
            ("posture_evaluation_policy".into(), Value::Null),
        ]),
    ))
    .unwrap();
    let layered = layer_profile_sources(
        &ExactDefinitionRef::parse("handbook.profile.child@1.0.0").unwrap(),
        vec![child, root],
    )
    .unwrap();
    assert_eq!(layered.decisions().len(), 11);
    assert_eq!(
        layered.decisions()[1].disposition(),
        LayerDisposition::Replaced
    );
    assert_eq!(
        layered.decisions()[2].disposition(),
        LayerDisposition::Inherited
    );
    assert_eq!(
        layered.field(ProfileField::SchemaRegistrySources),
        &json!(["example.schema@1.0.0"])
    );
}
#[test]
fn ancestry_depth_32_and_source_count_64_are_exact_boundaries() {
    let mut sources = Vec::new();
    sources.push(
        parse_profile_source(&source("example.profile.p0", "shipped", None, all_fields())).unwrap(),
    );
    for i in 1..32 {
        sources.push(
            parse_profile_source(&source(
                &format!("example.profile.p{i}"),
                "named",
                Some(&format!("example.profile.p{}@1.0.0", i - 1)),
                serde_json::Map::new(),
            ))
            .unwrap(),
        );
    }
    let selected = ExactDefinitionRef::parse("example.profile.p31@1.0.0").unwrap();
    assert_eq!(
        layer_profile_sources(&selected, sources.clone())
            .unwrap()
            .ancestry()
            .len(),
        32
    );
    sources.push(
        parse_profile_source(&source(
            "example.profile.p32",
            "repository",
            Some("example.profile.p31@1.0.0"),
            serde_json::Map::new(),
        ))
        .unwrap(),
    );
    assert!(layer_profile_sources(
        &ExactDefinitionRef::parse("example.profile.p32@1.0.0").unwrap(),
        sources
    )
    .is_err());
    let root = parse_profile_source(&source(
        "example.profile.root",
        "shipped",
        None,
        all_fields(),
    ))
    .unwrap();
    let mut sixty_four = vec![root.clone()];
    for i in 0..63 {
        sixty_four.push(
            parse_profile_source(&source(
                &format!("example.profile.unused{i}"),
                "named",
                Some("example.profile.root@1.0.0"),
                serde_json::Map::new(),
            ))
            .unwrap(),
        );
    }
    assert!(layer_profile_sources(root.exact_ref(), sixty_four.clone()).is_ok());
    sixty_four.push(
        parse_profile_source(&source(
            "example.profile.overflow",
            "named",
            Some("example.profile.root@1.0.0"),
            serde_json::Map::new(),
        ))
        .unwrap(),
    );
    assert!(layer_profile_sources(root.exact_ref(), sixty_four).is_err());
}
