use std::path::{Path, PathBuf};
use system_compiler::{
    load_pipeline_definition, resolve_pipeline_route, supported_route_state_variables,
    ActivationClause, ActivationConditionSet, ActivationOperator, PipelineBody, PipelineDefaults,
    PipelineDefinition, PipelineHeader, PipelineStage, RouteEvaluationError, RouteStageReason,
    RouteStageStatus, RouteVariables, StageActivation,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn fixture(path: &str) -> PipelineDefinition {
    let root = repo_root();
    load_pipeline_definition(&root, path).expect("pipeline fixture")
}

#[test]
fn route_evaluation_preserves_declared_order_and_marks_false_activation_skipped() {
    let definition = fixture("pipelines/foundation_inputs.yaml");
    let variables = RouteVariables::from_pairs([
        ("needs_project_context", false),
        ("charter_gaps_detected", false),
    ])
    .expect("route variables");

    let result = resolve_pipeline_route(&definition, &variables).expect("route");

    assert_eq!(result.pipeline_id, definition.header.id);
    assert_eq!(result.stages.len(), definition.declared_stages().len());

    assert_eq!(
        result
            .stages
            .iter()
            .map(|stage| stage.stage_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "stage.00_base",
            "stage.04_charter_inputs",
            "stage.05_charter_synthesize",
            "stage.06_project_context_interview",
            "stage.07_foundation_pack",
        ]
    );
    assert_eq!(
        result
            .stages
            .iter()
            .map(|stage| stage.status)
            .collect::<Vec<_>>(),
        vec![
            RouteStageStatus::Active,
            RouteStageStatus::Active,
            RouteStageStatus::Active,
            RouteStageStatus::Skipped,
            RouteStageStatus::Active,
        ]
    );

    match &result.stages[3].reason {
        Some(RouteStageReason::SkippedActivationFalse {
            operator,
            unsatisfied_variables,
        }) => {
            assert_eq!(*operator, ActivationOperator::Any);
            assert_eq!(
                unsatisfied_variables,
                &vec![
                    "charter_gaps_detected".to_string(),
                    "needs_project_context".to_string(),
                ]
            );
        }
        other => panic!("expected skipped reason, got {other:?}"),
    }
}

#[test]
fn route_evaluation_emits_exactly_one_next_when_route_state_is_missing() {
    let definition = fixture("pipelines/foundation_inputs.yaml");
    let variables = RouteVariables::empty();

    let result = resolve_pipeline_route(&definition, &variables).expect("route");

    assert_eq!(result.pipeline_id, definition.header.id);

    assert_eq!(
        result
            .stages
            .iter()
            .map(|stage| stage.status)
            .collect::<Vec<_>>(),
        vec![
            RouteStageStatus::Active,
            RouteStageStatus::Active,
            RouteStageStatus::Active,
            RouteStageStatus::Next,
            RouteStageStatus::Blocked,
        ]
    );
    assert_eq!(
        result
            .stages
            .iter()
            .filter(|stage| stage.status == RouteStageStatus::Next)
            .count(),
        1
    );

    match &result.stages[3].reason {
        Some(RouteStageReason::NextMissingRouteVariables {
            operator,
            missing_variables,
        }) => {
            assert_eq!(*operator, ActivationOperator::Any);
            assert_eq!(
                missing_variables,
                &vec![
                    "charter_gaps_detected".to_string(),
                    "needs_project_context".to_string(),
                ]
            );
        }
        other => panic!("expected next reason, got {other:?}"),
    }

    match &result.stages[4].reason {
        Some(RouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        }) => {
            assert_eq!(upstream_stage_id, "stage.06_project_context_interview");
            assert_eq!(*upstream_status, RouteStageStatus::Next);
        }
        other => panic!("expected blocked reason, got {other:?}"),
    }
}

#[test]
fn route_evaluation_refuses_out_of_contract_activation_inputs() {
    let definition = PipelineDefinition {
        source_path: PathBuf::from("pipelines/manual.yaml"),
        header: PipelineHeader {
            kind: "pipeline".to_string(),
            id: "pipeline.manual".to_string(),
            version: "0.1.0".to_string(),
            title: "Manual".to_string(),
            description: "manual".to_string(),
        },
        body: PipelineBody {
            defaults: PipelineDefaults {
                runner: "codex-cli".to_string(),
                profile: "python-uv".to_string(),
                enable_complexity: false,
            },
            stages: vec![PipelineStage {
                id: "stage.00_base".to_string(),
                file: "core/stages/00_base.md".to_string(),
                sets: None,
                activation: Some(StageActivation {
                    when: ActivationConditionSet {
                        operator: ActivationOperator::Any,
                        clauses: vec![ActivationClause {
                            variable: "9bad".to_string(),
                            value: true,
                        }],
                    },
                }),
            }],
        },
    };

    let variables = RouteVariables::empty();
    let err = resolve_pipeline_route(&definition, &variables).expect_err("route refusal");

    match err {
        RouteEvaluationError::InvalidActivationClause {
            stage_id, variable, ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(variable, "9bad");
        }
        other => panic!("expected invalid-activation-clause refusal, got {other:?}"),
    }
}

#[test]
fn supported_route_state_variables_are_derived_from_declared_stage_sets() {
    let definition = fixture("pipelines/foundation_inputs.yaml");
    let variables = supported_route_state_variables(&definition);

    assert_eq!(
        variables.into_iter().collect::<Vec<_>>(),
        vec!["needs_project_context".to_string()]
    );
}

#[test]
fn all_operator_with_known_false_and_missing_inputs_is_skipped_not_next() {
    let definition = PipelineDefinition {
        source_path: PathBuf::from("pipelines/manual-all.yaml"),
        header: PipelineHeader {
            kind: "pipeline".to_string(),
            id: "pipeline.manual_all".to_string(),
            version: "0.1.0".to_string(),
            title: "Manual All".to_string(),
            description: "manual all".to_string(),
        },
        body: PipelineBody {
            defaults: PipelineDefaults {
                runner: "codex-cli".to_string(),
                profile: "python-uv".to_string(),
                enable_complexity: false,
            },
            stages: vec![PipelineStage {
                id: "stage.00_base".to_string(),
                file: "core/stages/00_base.md".to_string(),
                sets: None,
                activation: Some(StageActivation {
                    when: ActivationConditionSet {
                        operator: ActivationOperator::All,
                        clauses: vec![
                            ActivationClause {
                                variable: "needs_project_context".to_string(),
                                value: true,
                            },
                            ActivationClause {
                                variable: "charter_gaps_detected".to_string(),
                                value: true,
                            },
                        ],
                    },
                }),
            }],
        },
    };

    let variables =
        RouteVariables::from_pairs([("needs_project_context", false)]).expect("route variables");
    let result = resolve_pipeline_route(&definition, &variables).expect("route");

    assert_eq!(result.stages[0].status, RouteStageStatus::Skipped);
    match &result.stages[0].reason {
        Some(RouteStageReason::SkippedActivationFalse {
            operator,
            unsatisfied_variables,
        }) => {
            assert_eq!(*operator, ActivationOperator::All);
            assert_eq!(
                unsatisfied_variables,
                &vec!["needs_project_context".to_string()]
            );
        }
        other => panic!("expected skipped reason, got {other:?}"),
    }
}
