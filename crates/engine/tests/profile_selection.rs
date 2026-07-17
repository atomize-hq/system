use handbook_engine::*;
use std::collections::BTreeSet;
use std::path::Path;
fn r(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).unwrap()
}
fn builtin(value: &str) -> DefinitionSourceBinding {
    let x = r(value);
    DefinitionSourceBinding {
        definition_ref: x.clone(),
        source: DefinitionSource::BuiltIn(x),
    }
}
fn request(reverse: bool) -> ProfileSelectionRequest {
    let names = [
        "project-authority",
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    let mut schemas = names
        .map(|n| builtin(&format!("handbook.schemas.artifacts.{n}@1.0.0")))
        .to_vec();
    let mut kinds = names
        .map(|n| builtin(&format!("handbook.artifact-kind.{n}@1.0.0")))
        .to_vec();
    if reverse {
        schemas.reverse();
        kinds.reverse();
    }
    ProfileSelectionRequest {
        selected_profile_ref: r("example.profile.root@1.0.0"),
        profile_sources: vec![DefinitionSourceBinding {
            definition_ref: r("example.profile.root@1.0.0"),
            source: DefinitionSource::RepositoryPath(
                "tests/fixtures/hcm_1_2_profile/root.yaml".into(),
            ),
        }],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources: schemas,
        artifact_kind_sources: kinds,
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![builtin(
            "handbook.semantic-validation.constitutional-root@1.0.0",
        )],
        project_condition_sources: vec![builtin(
            "handbook.condition.project.managed-operational-surface@1.0.0",
        )],
        vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
        context_resolution_sources: vec![builtin("handbook.context-resolution.shipped-root@1.0.0")],
        context_resolution_policy_sources: vec![
            builtin("handbook.mutation-matcher.core@1.0.0"),
            builtin("handbook.resolution-escalation.core@1.0.0"),
            builtin("handbook.memory-promotion.core@1.0.0"),
        ],
        allowed_schema_roots: vec!["definitions/schemas".into()],
    }
}
#[test]
fn exact_selection_recomputes_the_complete_typed_closure() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let forward = resolve_profile_selection(root, request(false)).unwrap();
    let reverse = resolve_profile_selection(root, request(true)).unwrap();
    assert_eq!(forward.exact_ref().as_str(), "example.profile.root@1.0.0");
    assert_eq!(
        forward.resolved_profile_fingerprint(),
        reverse.resolved_profile_fingerprint()
    );
    assert_eq!(forward.layer_decisions().len(), 11);
    assert_eq!(
        forward
            .artifact_kind_registry()
            .kind_refs()
            .into_iter()
            .map(|x| x.as_str().to_owned())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "handbook.artifact-kind.decision-record@1.0.0".into(),
            "handbook.artifact-kind.environment-context@1.0.0".into(),
            "handbook.artifact-kind.project-authority@1.0.0".into(),
            "handbook.artifact-kind.project-context@1.0.0".into(),
            "handbook.artifact-kind.risk-record@1.0.0".into(),
            "handbook.artifact-kind.work-specification@1.0.0".into()
        ])
    );
    assert_eq!(
        forward.artifact_instances().ids(),
        [
            "environment_context",
            "project_authority",
            "project_context"
        ]
    );
}
#[test]
fn missing_typed_source_and_nonempty_later_owned_values_refuse() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut missing = request(false);
    missing.project_condition_sources.clear();
    assert!(resolve_profile_selection(root, missing).is_err());
    let mut unsupported = request(false);
    unsupported.allowed_schema_roots.clear();
    assert!(resolve_profile_selection(root, unsupported).is_err());
}
