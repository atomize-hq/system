use handbook_engine::{
    InstanceProfileDefinition, ProjectConditionRegistry, SemanticCapabilityContract,
};

fn assert_public_type<T>() {}

#[test]
fn exact_hcm_1_2_public_owner_type_names_are_exported() {
    assert_public_type::<SemanticCapabilityContract>();
    assert_public_type::<ProjectConditionRegistry>();
    assert_public_type::<InstanceProfileDefinition>();
}
