use std::path::{Path, PathBuf};

use system_compiler::{
    load_pipeline_definition, ActivationOperator, PipelineLoadError, PipelineValidationError,
    StageFileValidationError,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[test]
fn foundation_pipeline_loads_with_deterministic_stage_order() {
    let repo_root = repo_root();

    let definition =
        load_pipeline_definition(&repo_root, "pipelines/foundation.yaml").expect("load pipeline");

    assert_eq!(definition.header.kind, "pipeline");
    assert_eq!(definition.header.id, "pipeline.foundation");
    assert_eq!(definition.body.defaults.runner, "codex-cli");
    assert_eq!(definition.body.defaults.profile, "python-uv");
    assert!(!definition.body.defaults.enable_complexity);
    assert_eq!(
        definition
            .body
            .stages
            .iter()
            .map(|stage| stage.id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "stage.00_base",
            "stage.05_charter_interview",
            "stage.06_project_context_interview",
            "stage.07_foundation_pack",
        ]
    );
    assert_eq!(
        definition.body.stages[1].sets.as_ref().expect("sets"),
        &vec!["needs_project_context".to_string()]
    );

    let activation = definition.body.stages[2]
        .activation
        .as_ref()
        .expect("activation");
    assert_eq!(activation.when.operator, ActivationOperator::Any);
    assert_eq!(activation.when.clauses.len(), 1);
    assert_eq!(activation.when.clauses[0].variable, "needs_project_context");
    assert!(activation.when.clauses[0].value);
}

#[test]
fn foundation_inputs_pipeline_parses_pipeline_entry_activation_only() {
    let repo_root = repo_root();

    let definition = load_pipeline_definition(&repo_root, "pipelines/foundation_inputs.yaml")
        .expect("load pipeline");

    let activation = definition.body.stages[3]
        .activation
        .as_ref()
        .expect("activation");
    assert_eq!(activation.when.operator, ActivationOperator::Any);
    assert_eq!(activation.when.clauses.len(), 2);
    assert_eq!(activation.when.clauses[0].variable, "needs_project_context");
    assert!(activation.when.clauses[0].value);
    assert_eq!(activation.when.clauses[1].variable, "charter_gaps_detected");
    assert!(activation.when.clauses[1].value);
}

#[test]
fn declared_stage_order_is_preserved_for_core_pipelines() {
    let repo_root = repo_root();

    let cases = [
        (
            "pipelines/foundation_inputs.yaml",
            vec![
                "stage.00_base",
                "stage.04_charter_inputs",
                "stage.05_charter_synthesize",
                "stage.06_project_context_interview",
                "stage.07_foundation_pack",
            ],
        ),
        (
            "pipelines/release.yaml",
            vec!["stage.00_base", "stage.01_release_plan"],
        ),
        (
            "pipelines/sprint.yaml",
            vec!["stage.00_base", "stage.02_sprint_plan"],
        ),
    ];

    for (pipeline_path, expected_stage_ids) in cases {
        let definition =
            load_pipeline_definition(&repo_root, pipeline_path).expect("schema-compatible load");
        assert_eq!(definition.header.kind, "pipeline", "path={pipeline_path}");
        assert_eq!(
            definition
                .declared_stages()
                .iter()
                .map(|stage| stage.id.as_str())
                .collect::<Vec<_>>(),
            expected_stage_ids,
            "path={pipeline_path}"
        );
    }
}

#[test]
fn richer_root_pipeline_yaml_is_refused_as_out_of_scope_shape() {
    let repo_root = repo_root();

    let err = load_pipeline_definition(&repo_root, "pipeline.yaml").expect_err("root pipeline");

    match err {
        PipelineLoadError::BodyParse { .. } => {}
        other => panic!("expected body parse refusal, got {other:?}"),
    }
}

#[test]
fn wrong_document_count_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/one-doc.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.one_doc
version: 0.1.0
title: "One Doc"
description: "missing body"
"#,
    );

    let err =
        load_pipeline_definition(repo_root, "pipelines/one-doc.yaml").expect_err("wrong docs");

    match err {
        PipelineLoadError::WrongDocumentCount { actual, .. } => assert_eq!(actual, 1),
        other => panic!("expected wrong-document-count refusal, got {other:?}"),
    }
}

#[test]
fn malformed_yaml_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/malformed.yaml");
    write_file(
        &pipeline_path,
        "---\n\
kind: pipeline\n\
id: pipeline.bad\n\
version: 0.1.0\n\
title: \"Bad\"\n\
description: \"header\"\n\
---\n\
defaults:\n\
\trunner: codex-cli\n\
  profile: python-uv\n\
  enable_complexity: false\n\
stages:\n\
  - id: stage.00_base\n\
    file: core/stages/00_base.md\n",
    );

    let err = load_pipeline_definition(repo_root, "pipelines/malformed.yaml")
        .expect_err("malformed yaml");

    match err {
        PipelineLoadError::HeaderParse { .. } | PipelineLoadError::BodyParse { .. } => {}
        other => panic!("expected parse refusal, got {other:?}"),
    }
}

#[test]
fn empty_header_fields_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("pipelines/empty-title.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.empty_title
version: 0.1.0
title: "   "
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );

    let err =
        load_pipeline_definition(repo_root, "pipelines/empty-title.yaml").expect_err("empty title");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::EmptyField { field },
            ..
        } => assert_eq!(field, "title"),
        other => panic!("expected empty-field refusal, got {other:?}"),
    }
}

#[test]
fn unknown_fields_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/unknown-field.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.unknown_field
version: 0.1.0
title: "Unknown Field"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    outputs:
      - artifacts/charter/CHARTER.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/unknown-field.yaml")
        .expect_err("unknown field");

    match err {
        PipelineLoadError::BodyParse { .. } => {}
        other => panic!("expected body parse refusal, got {other:?}"),
    }
}

#[test]
fn unsupported_header_kind_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/wrong-kind.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: schema
id: pipeline.wrong_kind
version: 0.1.0
title: "Wrong Kind"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/wrong-kind.yaml")
        .expect_err("wrong header kind");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::UnsupportedKind { actual },
            ..
        } => assert_eq!(actual, "schema"),
        other => panic!("expected unsupported-kind refusal, got {other:?}"),
    }
}

#[test]
fn duplicate_stage_ids_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    write_file(
        &repo_root.join("core/stages/05_charter_interview.md"),
        "charter",
    );
    let pipeline_path = repo_root.join("pipelines/duplicate-stage.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.duplicate_stage
version: 0.1.0
title: "Duplicate Stage"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
  - id: stage.00_base
    file: core/stages/05_charter_interview.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/duplicate-stage.yaml")
        .expect_err("duplicate stage");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::DuplicateStageId { stage_id },
            ..
        } => assert_eq!(stage_id, "stage.00_base"),
        other => panic!("expected duplicate-stage refusal, got {other:?}"),
    }
}

#[test]
fn unsupported_activation_syntax_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/bad-activation.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.bad_activation
version: 0.1.0
title: "Bad Activation"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.needs_project_context != true
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/bad-activation.yaml")
        .expect_err("bad activation");

    match err {
        PipelineLoadError::BodyParse { .. } => {}
        other => panic!("expected activation parse refusal, got {other:?}"),
    }
}

#[test]
fn unsupported_activation_value_type_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/bad-activation-value.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.bad_activation_value
version: 0.1.0
title: "Bad Activation Value"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.target_env == "production"
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/bad-activation-value.yaml")
        .expect_err("bad activation value");

    match err {
        PipelineLoadError::BodyParse { source, .. } => {
            assert!(
                source
                    .to_string()
                    .contains("reduced v1 supports only boolean activation values"),
                "unexpected parse error: {source}"
            );
        }
        other => panic!("expected activation-value refusal, got {other:?}"),
    }
}

#[test]
fn numeric_activation_value_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/numeric-activation-value.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.numeric_activation_value
version: 0.1.0
title: "Numeric Activation Value"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.max_attempts == 3
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/numeric-activation-value.yaml")
        .expect_err("numeric activation value");

    match err {
        PipelineLoadError::BodyParse { source, .. } => {
            assert!(
                source
                    .to_string()
                    .contains("reduced v1 supports only boolean activation values"),
                "unexpected parse error: {source}"
            );
        }
        other => panic!("expected activation-value refusal, got {other:?}"),
    }
}

#[test]
fn out_of_root_stage_paths_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/out-of-root.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.out_of_root
version: 0.1.0
title: "Out Of Root"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: ../outside.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/out-of-root.yaml")
        .expect_err("out-of-root stage path");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::StageFileOutsideRepoRoot { stage_id, file },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "../outside.md");
        }
        other => panic!("expected stage-path refusal, got {other:?}"),
    }
}

#[test]
fn missing_stage_files_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/missing-stage.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.missing_stage
version: 0.1.0
title: "Missing Stage"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/does_not_exist.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/missing-stage.yaml")
        .expect_err("missing stage file");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::Missing,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/does_not_exist.md");
        }
        other => panic!("expected missing-stage refusal, got {other:?}"),
    }
}

#[test]
fn repo_local_stage_paths_outside_stage_directory_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("README.md"), "not a stage");
    let pipeline_path = repo_root.join("pipelines/outside-stage-directory.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.outside_surface
version: 0.1.0
title: "Outside Surface"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: README.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/outside-stage-directory.yaml")
        .expect_err("outside stage directory");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::OutsideStageDirectory,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "README.md");
        }
        other => panic!("expected stage-directory refusal, got {other:?}"),
    }
}

#[test]
fn stage_paths_with_wrong_extension_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(
        &repo_root.join("core/stages/not_markdown.txt"),
        "not markdown",
    );
    let pipeline_path = repo_root.join("pipelines/wrong-extension-stage.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.wrong_extension_stage
version: 0.1.0
title: "Wrong Extension Stage"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/not_markdown.txt
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/wrong-extension-stage.yaml")
        .expect_err("wrong extension stage path");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::WrongExtension,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/not_markdown.txt");
        }
        other => panic!("expected wrong-extension refusal, got {other:?}"),
    }
}

#[test]
fn non_regular_stage_paths_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let stage_dir = repo_root.join("core/stages/not_a_file.md");
    std::fs::create_dir_all(&stage_dir).expect("mkdirs");
    let pipeline_path = repo_root.join("pipelines/non-regular-stage.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.non_regular_stage
version: 0.1.0
title: "Non Regular Stage"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/not_a_file.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/non-regular-stage.yaml")
        .expect_err("non-regular stage path");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::NotRegularFile,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/not_a_file.md");
        }
        other => panic!("expected non-regular-stage refusal, got {other:?}"),
    }
}

#[test]
fn activation_all_operator_parses_multiple_boolean_clauses() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("pipelines/all-activation.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.all_activation
version: 0.1.0
title: "All Activation"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        all:
          - variables.needs_project_context == true
          - variables.charter_gaps_detected == false
"#,
    );

    let definition =
        load_pipeline_definition(repo_root, "pipelines/all-activation.yaml").expect("load");

    let activation = definition.body.stages[0]
        .activation
        .as_ref()
        .expect("activation");
    assert_eq!(activation.when.operator, ActivationOperator::All);
    assert_eq!(activation.when.clauses.len(), 2);
    assert_eq!(activation.when.clauses[0].variable, "needs_project_context");
    assert!(activation.when.clauses[0].value);
    assert_eq!(activation.when.clauses[1].variable, "charter_gaps_detected");
    assert!(!activation.when.clauses[1].value);
}

#[test]
fn pipeline_path_must_stay_repo_relative() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let err = load_pipeline_definition(repo_root, "../pipelines/outside.yaml")
        .expect_err("pipeline path should be rejected");

    match err {
        PipelineLoadError::UnsupportedPipelinePath { reason, .. } => {
            assert_eq!(reason, "path must not escape the repo root");
        }
        other => panic!("expected unsupported-pipeline-path refusal, got {other:?}"),
    }
}

#[test]
fn missing_pipeline_file_is_reported_as_read_failure() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let err = load_pipeline_definition(repo_root, "pipelines/missing.yaml")
        .expect_err("missing pipeline file");

    match err {
        PipelineLoadError::ReadFailure { path, .. } => {
            assert_eq!(path, repo_root.join("pipelines/missing.yaml"));
        }
        other => panic!("expected read-failure refusal, got {other:?}"),
    }
}

#[test]
fn extra_yaml_documents_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("pipelines/extra-doc.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.extra_doc
version: 0.1.0
title: "Extra Doc"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
---
extra: true
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/extra-doc.yaml")
        .expect_err("third document should be refused");

    match err {
        PipelineLoadError::WrongDocumentCount { actual, .. } => assert_eq!(actual, 3),
        other => panic!("expected wrong-document-count refusal, got {other:?}"),
    }
}

#[test]
fn empty_stage_list_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("pipelines/empty-stages.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.empty_stages
version: 0.1.0
title: "Empty Stages"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages: []
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/empty-stages.yaml")
        .expect_err("empty stages");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::EmptyStages,
            ..
        } => {}
        other => panic!("expected empty-stages refusal, got {other:?}"),
    }
}

#[test]
fn empty_sets_list_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("pipelines/empty-sets.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.empty_sets
version: 0.1.0
title: "Empty Sets"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    sets: []
"#,
    );

    let err =
        load_pipeline_definition(repo_root, "pipelines/empty-sets.yaml").expect_err("empty sets");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::EmptySetsList { stage_id },
            ..
        } => assert_eq!(stage_id, "stage.00_base"),
        other => panic!("expected empty-sets refusal, got {other:?}"),
    }
}

#[test]
fn blank_set_variable_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("pipelines/blank-set-variable.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.blank_set_variable
version: 0.1.0
title: "Blank Set Variable"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    sets:
      - "   "
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/blank-set-variable.yaml")
        .expect_err("blank set variable");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::EmptySetVariable { stage_id, index },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(index, 0);
        }
        other => panic!("expected empty-set-variable refusal, got {other:?}"),
    }
}

#[test]
fn empty_activation_clause_list_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("pipelines/empty-activation-list.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.empty_activation_list
version: 0.1.0
title: "Empty Activation List"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any: []
"#,
    );

    let err = load_pipeline_definition(repo_root, "pipelines/empty-activation-list.yaml")
        .expect_err("empty activation list");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidActivation {
                    stage_id,
                    reason:
                        system_compiler::ActivationValidationError::EmptyConditionList {
                            operator: ActivationOperator::Any,
                        },
                },
            ..
        } => assert_eq!(stage_id, "stage.00_base"),
        other => panic!("expected empty-activation-list refusal, got {other:?}"),
    }
}
