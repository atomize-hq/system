use handbook_pipeline::route_state::{
    handbook_product_pipeline_storage_layout_contract, PipelineStorageLayoutContract,
};

#[test]
fn public_storage_layout_contract_boundary_supports_default_and_custom_contracts() {
    let default_contract = *handbook_product_pipeline_storage_layout_contract();
    let expected_default = PipelineStorageLayoutContract::try_from_paths(
        ".handbook/state",
        ".handbook/state/pipeline",
        ".handbook/state/pipeline/stage_capture",
        ".handbook/state/pipeline/capture",
        "artifacts/handoff/feature_slice",
    )
    .expect("default contract should validate");

    assert_eq!(default_contract, expected_default);

    let custom_contract = PipelineStorageLayoutContract::try_from_paths(
        ".substrate/handbook/state",
        ".substrate/handbook/state/pipeline",
        ".substrate/handbook/state/pipeline/stage_capture",
        ".substrate/handbook/state/pipeline/capture",
        ".substrate/handbook/artifacts/handoff/feature_slice",
    )
    .expect("custom contract should validate");

    assert_ne!(custom_contract, default_contract);
}

#[test]
fn public_storage_layout_contract_boundary_rejects_invalid_contracts() {
    let err = PipelineStorageLayoutContract::try_from_paths(
        ".substrate/handbook/state",
        ".substrate/handbook/pipeline",
        ".substrate/handbook/state/pipeline/stage_capture",
        ".substrate/handbook/state/pipeline/capture",
        ".substrate/handbook/artifacts/handoff/feature_slice",
    )
    .expect_err("invalid public contract should be rejected");

    assert!(
        err.contains("pipeline_dir_relative"),
        "expected pipeline_dir_relative in error, got: {err}"
    );
}
