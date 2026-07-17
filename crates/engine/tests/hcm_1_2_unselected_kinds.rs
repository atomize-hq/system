use handbook_engine::{
    load_artifact_kind_registry, ArtifactKindRegistryLoadRequest, ExactDefinitionRef,
};
use std::collections::BTreeSet;
use std::path::Path;
fn source(class: &str, id: &str, suffix: &str) -> String {
    format!("definitions/{class}/{id}/1.0.0{suffix}")
}
#[test]
fn unselected_catalog_is_exact_capability_free_and_permutation_stable() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let names = ["work-specification", "decision-record", "risk-record"];
    let schemas = names
        .iter()
        .map(|n| {
            source(
                "schemas",
                &format!("handbook.schemas.artifacts.{n}"),
                ".entry.yaml",
            )
        })
        .collect::<Vec<_>>();
    let kinds = names
        .iter()
        .map(|n| {
            source(
                "artifact-kinds",
                &format!("handbook.artifact-kind.{n}"),
                ".yaml",
            )
        })
        .collect::<Vec<_>>();
    let req = |kinds| {
        ArtifactKindRegistryLoadRequest::new(
            ExactDefinitionRef::parse("handbook.roles.core@1.1.0").unwrap(),
            schemas.clone(),
            vec!["definitions/schemas".into()],
            kinds,
        )
    };
    let forward = load_artifact_kind_registry(root, req(kinds.clone())).unwrap();
    let mut reversed = kinds;
    reversed.reverse();
    let reverse = load_artifact_kind_registry(root, req(reversed)).unwrap();
    assert_eq!(forward.fingerprint(), reverse.fingerprint());
    let refs = forward
        .kind_refs()
        .into_iter()
        .map(|r| r.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        refs,
        BTreeSet::from([
            "handbook.artifact-kind.decision-record@1.0.0".into(),
            "handbook.artifact-kind.risk-record@1.0.0".into(),
            "handbook.artifact-kind.work-specification@1.0.0".into()
        ])
    );
    for r in forward.kind_refs() {
        let k = forward.kind(&r).unwrap();
        assert!(k.semantic_capabilities().is_empty());
        if r.as_str().contains("work-specification") {
            assert_eq!(
                k.supported_role_refs(),
                [
                    "atomic_action",
                    "coordination_horizon",
                    "delivery_unit",
                    "execution_envelope",
                    "implementation_unit"
                ]
            );
        } else {
            assert!(k.supported_role_refs().is_empty());
        }
    }
}
