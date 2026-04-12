use std::path::PathBuf;

use system_compiler::{
    load_pipeline_catalog, render_pipeline_list, render_pipeline_show, PipelineLookupError,
    PipelineSelection,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

use std::path::Path;

#[test]
fn catalog_discovers_canonical_pipeline_and_stage_ids() {
    let catalog = load_pipeline_catalog(repo_root()).expect("catalog");

    assert_eq!(catalog.pipeline_count(), 4);
    assert!(catalog.stage_count() >= 8);

    let pipeline = catalog
        .resolve_selector("foundation")
        .expect("pipeline shorthand");
    match pipeline {
        PipelineSelection::Pipeline(pipeline) => {
            assert_eq!(pipeline.definition.header.id, "pipeline.foundation");
        }
        other => panic!("expected pipeline selection, got {other:?}"),
    }

    let unique_stage = catalog
        .resolve_selector("00_base")
        .expect("stage shorthand");
    match unique_stage {
        PipelineSelection::Stage(stage) => {
            assert_eq!(stage.id, "stage.00_base");
            assert_eq!(
                stage.pipelines,
                vec![
                    "pipeline.foundation".to_string(),
                    "pipeline.foundation_inputs".to_string(),
                    "pipeline.release".to_string(),
                    "pipeline.sprint".to_string(),
                ]
            );
        }
        other => panic!("expected stage selection, got {other:?}"),
    }

    let unsupported = catalog
        .resolve_selector("pipelines/foundation.yaml")
        .expect_err("path selector refusal");
    match unsupported {
        PipelineLookupError::UnsupportedSelector { selector, reason } => {
            assert_eq!(selector, "pipelines/foundation.yaml");
            assert!(reason.contains("evidence only"));
        }
        other => panic!("expected unsupported-selector refusal, got {other:?}"),
    }
}

#[test]
fn catalog_refuses_ambiguous_shorthand_with_explicit_conflicting_ids() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join("core/stages/alpha.md"),
        r#"---
kind: stage
id: stage.alpha
version: 0.1.0
title: Alpha Stage
description: alpha
---
# alpha
"#,
    );
    write_file(
        &root.join("pipelines/alpha.yaml"),
        r#"---
kind: pipeline
id: pipeline.alpha
version: 0.1.0
title: Alpha Pipeline
description: alpha
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.alpha
    file: core/stages/alpha.md
"#,
    );

    let catalog = load_pipeline_catalog(root).expect("catalog");
    let err = catalog.resolve_selector("alpha").expect_err("ambiguous");
    match err {
        PipelineLookupError::AmbiguousSelector { selector, matches } => {
            assert_eq!(selector, "alpha");
            assert_eq!(
                matches,
                vec!["pipeline.alpha".to_string(), "stage.alpha".to_string()]
            );
        }
        other => panic!("expected ambiguous-selector refusal, got {other:?}"),
    }
}

#[test]
fn catalog_renders_pipeline_yaml_and_stage_front_matter_as_distinct_sources() {
    let catalog = load_pipeline_catalog(repo_root()).expect("catalog");

    let pipeline = catalog
        .resolve_selector("pipeline.foundation_inputs")
        .expect("pipeline selection");
    let pipeline_render = render_pipeline_show(&pipeline);
    assert!(pipeline_render.contains("PIPELINE: pipeline.foundation_inputs"));
    assert!(pipeline_render.contains("TITLE: Foundation Pipeline (Dev/Test Charter Inputs"));
    assert!(pipeline_render.contains("SOURCE: pipelines/foundation_inputs.yaml"));
    assert!(pipeline_render.contains("DEFAULTS:"));
    assert!(pipeline_render.contains("runner: codex-cli"));
    assert!(pipeline_render.contains("profile: python-uv"));
    assert!(pipeline_render.contains("enable_complexity: false"));
    assert!(pipeline_render.contains("  1. stage.00_base | core/stages/00_base.md"));
    assert!(pipeline_render.contains("stage.04_charter_inputs"));
    assert!(pipeline_render.contains("core/stages/04_charter_inputs.md"));
    assert!(pipeline_render
        .contains("  5. stage.07_foundation_pack | core/stages/07_foundation_pack.md"));
    assert!(pipeline_render.contains("DEFAULTS:"));
    assert!(pipeline_render.contains("STAGES:"));

    let stage = catalog
        .resolve_selector("stage.07_foundation_pack")
        .expect("stage selection");
    let stage_render = render_pipeline_show(&stage);
    assert!(stage_render.contains("STAGE: stage.07_foundation_pack"));
    assert!(stage_render.contains("KIND: stage"));
    assert!(stage_render.contains("VERSION: 0.1.0"));
    assert!(stage_render.contains("TITLE: Foundation Pack Synthesis"));
    assert!(stage_render.contains("DESCRIPTION: Synthesizes project-specific foundation artifacts"));
    assert!(stage_render.contains("WORK_LEVEL: L1"));
    assert!(stage_render.contains("SOURCE: core/stages/07_foundation_pack.md"));
    assert!(stage_render.contains("pipeline.foundation"));
    assert!(stage_render.contains("pipeline.foundation_inputs"));
    assert!(stage_render.contains("PIPELINES:"));
    assert!(!stage_render.contains("DEFAULTS:"));
    assert!(!stage_render.contains("STAGES:"));

    let list = render_pipeline_list(&catalog);
    assert!(list.contains("PIPELINE INVENTORY"));
    assert!(list.contains("PIPELINE COUNT: 4"));
    assert!(list.contains("PIPELINE: pipeline.foundation"));
    assert!(list.contains("SOURCE: pipelines/foundation.yaml"));
    assert!(list.contains("PIPELINE: pipeline.sprint"));
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}
