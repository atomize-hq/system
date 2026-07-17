use crate::definition_identity::ExactDefinitionRef;

pub(crate) struct BuiltInSource {
    pub(crate) package_path: &'static str,
    pub(crate) bytes: &'static [u8],
}

macro_rules! source {
    ($path:literal) => {
        BuiltInSource {
            package_path: concat!("definitions/", $path),
            bytes: include_bytes!(concat!("../definitions/", $path)),
        }
    };
}

pub(crate) fn definition(reference: &ExactDefinitionRef) -> Option<BuiltInSource> {
    Some(match reference.as_str() {
        "handbook.profile.shipped-root@1.0.0" => {
            source!("profiles/handbook.profile.shipped-root/1.0.0.yaml")
        }
        "handbook.roles.core@1.0.0" => {
            source!("stable-roles/handbook.roles.core/1.0.0.yaml")
        }
        "handbook.roles.core@1.1.0" => {
            source!("stable-roles/handbook.roles.core/1.1.0.yaml")
        }
        "handbook.schemas.artifacts.project-authority@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.project-context@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.environment-context@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.environment-context/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.work-specification@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.work-specification/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.decision-record@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.decision-record/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.risk-record@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.risk-record/1.0.0.entry.yaml")
        }
        "handbook.artifact-kind.project-authority@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml")
        }
        "handbook.artifact-kind.project-context@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.project-context/1.0.0.yaml")
        }
        "handbook.artifact-kind.environment-context@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.environment-context/1.0.0.yaml")
        }
        "handbook.artifact-kind.work-specification@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.work-specification/1.0.0.yaml")
        }
        "handbook.artifact-kind.decision-record@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.decision-record/1.0.0.yaml")
        }
        "handbook.artifact-kind.risk-record@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.risk-record/1.0.0.yaml")
        }
        "handbook.capabilities.constitutional-root@1.0.0" => {
            source!("semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml")
        }
        "handbook.semantic-validation.constitutional-root@1.0.0" => source!(
            "semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml"
        ),
        "handbook.condition.project.managed-operational-surface@1.0.0" => source!(
            "project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml"
        ),
        "handbook.vocabulary.shipped-root@1.0.0" => {
            source!("vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml")
        }
        "handbook.context-resolution.shipped-root@1.0.0" => {
            source!("context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml")
        }
        "handbook.mutation-matcher.core@1.0.0" => {
            source!("context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml")
        }
        "handbook.resolution-escalation.core@1.0.0" => {
            source!("context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml")
        }
        "handbook.memory-promotion.core@1.0.0" => {
            source!("context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml")
        }
        _ => return None,
    })
}

pub(crate) fn schema_document(package_path: &str) -> Option<&'static [u8]> {
    Some(match package_path {
        "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.schema.json"
        ),
        _ => return None,
    })
}
