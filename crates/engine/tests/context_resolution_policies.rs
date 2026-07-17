use handbook_engine::ContextResolutionPolicyRegistry;
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
