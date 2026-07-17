use handbook_engine::{ContextResolutionPolicyRegistry, ContextResolutionStackDefinition};
use std::path::Path;
const STACK_PATH: &str =
    "definitions/context-resolution-stacks/handbook.context-resolution.shipped-root/1.0.0.yaml";
const STACK: &[u8] = include_bytes!(
    "../definitions/context-resolution-stacks/handbook.context-resolution.shipped-root/1.0.0.yaml"
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
