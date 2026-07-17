use crate::artifact_instance::ArtifactInstanceRegistry;
use crate::artifact_kind_registry::{
    load_artifact_kind_registry, ArtifactKindRegistry, ArtifactKindRegistryLoadRequest,
};
use crate::context_resolution_registry::{
    ContextResolutionPolicyRegistry, ContextResolutionStackDefinition,
};
use crate::definition_identity::{
    fingerprint_serializable, DefinitionFingerprint, ExactDefinitionRef, SourceByteBudget,
};
use crate::instance_profile::{
    admit_selection_request, layer_profile_sources, parse_profile_source, AuthoredProfileSource,
    DefinitionSource, DefinitionSourceBinding, LayeredProfile, ProfileField, ProfileLoadError,
    ProfileLoadErrorKind, ProfileSelectionRequest,
};
use crate::project_condition_registry::ProjectConditionDefinition;
use crate::stable_role_registry::{read_trusted_repo_source, StableRoleRegistry};
use crate::vocabulary_registry::VocabularyDefinition;
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct ResolvedInstanceProfile {
    layered: LayeredProfile,
    stable_role_registry: StableRoleRegistry,
    artifact_kind_registry: ArtifactKindRegistry,
    artifact_instances: ArtifactInstanceRegistry,
    vocabulary: VocabularyDefinition,
    context_resolution: ContextResolutionStackDefinition,
    resolved_profile_fingerprint: DefinitionFingerprint,
}

impl ResolvedInstanceProfile {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        self.layered.selected_profile_ref()
    }
    pub fn layer_decisions(&self) -> &[crate::instance_profile::ProfileLayerDecision] {
        self.layered.decisions()
    }
    pub fn artifact_kind_registry(&self) -> &ArtifactKindRegistry {
        &self.artifact_kind_registry
    }
    pub fn artifact_instances(&self) -> &ArtifactInstanceRegistry {
        &self.artifact_instances
    }
    pub fn stable_role_registry(&self) -> &StableRoleRegistry {
        &self.stable_role_registry
    }
    pub fn vocabulary(&self) -> &VocabularyDefinition {
        &self.vocabulary
    }
    pub fn context_resolution(&self) -> &ContextResolutionStackDefinition {
        &self.context_resolution
    }
    pub fn resolved_profile_fingerprint(&self) -> &DefinitionFingerprint {
        &self.resolved_profile_fingerprint
    }
}

pub fn resolve_profile_selection(
    repo: impl AsRef<Path>,
    request: ProfileSelectionRequest,
) -> Result<ResolvedInstanceProfile, ProfileLoadError> {
    let repo = repo.as_ref();
    admit_selection_request(repo, request.clone())?;
    let profile_sources = load_profile_sources(repo, &request.profile_sources)?;
    let layered = layer_profile_sources(&request.selected_profile_ref, profile_sources.clone())?;
    let ancestry = layered.ancestry().iter().cloned().collect::<BTreeSet<_>>();
    let selected_sources = profile_sources
        .iter()
        .filter(|s| ancestry.contains(s.exact_ref()))
        .collect::<Vec<_>>();

    let stable = stable_selection(layered.field(ProfileField::StableRoleRegistry))?;
    let stable_source = find_binding(&request.stable_role_registry_sources, &stable.0)?;
    if !matches!(stable_source.source, DefinitionSource::BuiltIn(_)) {
        return Err(unsupported(
            "HCM-1.2 stable role registry must be exact built-in",
        ));
    }
    let stable_role_registry =
        StableRoleRegistry::load_builtin(&stable.0).map_err(registry_error)?;
    if stable_role_registry.fingerprint() != &stable.1 {
        return Err(fingerprint("stable role registry"));
    }

    let schema_refs = union_refs(&selected_sources, ProfileField::SchemaRegistrySources)?;
    let kind_refs = union_refs(&selected_sources, ProfileField::ArtifactKindSources)?;
    let kind_registry = load_artifact_kind_registry(
        repo,
        ArtifactKindRegistryLoadRequest::new(
            stable.0.clone(),
            paths_for(&request.schema_entry_sources, &schema_refs)?,
            request.allowed_schema_roots.clone(),
            paths_for(&request.artifact_kind_sources, &kind_refs)?,
        )
        .with_semantic_sources(
            all_paths(&request.semantic_capability_sources)?,
            all_paths(&request.semantic_validator_sources)?,
        ),
    )
    .map_err(registry_error)?;

    let conditions = load_conditions(repo, &request.project_condition_sources)?;
    let condition_refs = conditions.iter().collect::<Vec<_>>();
    let instance_values = layered
        .field(ProfileField::ArtifactInstances)
        .as_array()
        .ok_or_else(|| unsupported("artifact_instances must be an array"))?;
    let artifact_instances =
        ArtifactInstanceRegistry::resolve(instance_values, &kind_registry, &condition_refs)
            .map_err(registry_error)?;

    let vocabulary_refs = union_scalar_refs(&selected_sources, ProfileField::VocabularyRef)?;
    let mut vocabularies = BTreeMap::new();
    for reference in vocabulary_refs {
        let path = path_for(find_binding(&request.vocabulary_sources, &reference)?)?;
        let value = VocabularyDefinition::load(repo, &path).map_err(registry_error)?;
        vocabularies.insert(reference, value);
    }
    let vocabulary_ref = scalar_ref(layered.field(ProfileField::VocabularyRef), "vocabulary_ref")?;
    let vocabulary = vocabularies
        .remove(&vocabulary_ref)
        .ok_or_else(|| missing("vocabulary"))?;

    let policies = ContextResolutionPolicyRegistry::load(
        repo,
        &all_paths(&request.context_resolution_policy_sources)?,
    )
    .map_err(registry_error)?;
    let stack_refs = union_scalar_refs(&selected_sources, ProfileField::ContextResolutionRef)?;
    let mut stacks = BTreeMap::new();
    for reference in stack_refs {
        let path = path_for(find_binding(
            &request.context_resolution_sources,
            &reference,
        )?)?;
        let value = ContextResolutionStackDefinition::load(repo, &path, &policies)
            .map_err(registry_error)?;
        stacks.insert(reference, value);
    }
    let stack_ref = scalar_ref(
        layered.field(ProfileField::ContextResolutionRef),
        "context_resolution_ref",
    )?;
    let context_resolution = stacks
        .remove(&stack_ref)
        .ok_or_else(|| missing("Context Resolution stack"))?;

    require_empty_array(
        layered.field(ProfileField::ProjectionCatalogRefs),
        "projection_catalog_refs",
    )?;
    if !layered
        .field(ProfileField::PostureEvaluationPolicy)
        .is_null()
    {
        return Err(unsupported("posture_evaluation_policy must be null"));
    }
    require_empty_array(
        layered.field(ProfileField::DockRequirementRefs),
        "dock_requirement_refs",
    )?;
    require_empty_array(
        layered.field(ProfileField::AdapterOverlayRefs),
        "adapter_overlay_refs",
    )?;
    if layered
        .field(ProfileField::Extensions)
        .as_object()
        .is_none_or(|m| !m.is_empty())
    {
        return Err(unsupported("profile extensions must be empty"));
    }

    let ancestry_closure = selected_sources
        .iter()
        .map(|s| AncestryMember {
            profile_ref: s.exact_ref().as_str(),
            profile_fingerprint: s.profile_fingerprint().as_str(),
        })
        .collect();
    let decisions = layered
        .decisions()
        .iter()
        .map(|d| DecisionMember {
            field: d.field().key(),
            source_profile_ref: d.source_profile_ref().as_str(),
            disposition: match d.disposition() {
                crate::instance_profile::LayerDisposition::Inherited => "inherited",
                crate::instance_profile::LayerDisposition::Replaced => "replaced",
            },
        })
        .collect();
    let resolved_profile_fingerprint = fingerprint_serializable(&ResolvedClosure {
        selected_profile_ref: layered.selected_profile_ref().as_str(),
        ancestry: ancestry_closure,
        layer_decisions: decisions,
        stable_role_registry_fingerprint: stable_role_registry.fingerprint().as_str(),
        schema_registry_fingerprint: kind_registry.schema_registry().fingerprint().as_str(),
        artifact_kind_registry_fingerprint: kind_registry.fingerprint().as_str(),
        artifact_instance_registry_fingerprint: artifact_instances.fingerprint().as_str(),
        vocabulary_fingerprint: vocabulary.vocabulary_fingerprint().as_str(),
        context_resolution_fingerprint: context_resolution.definition_fingerprint().as_str(),
        condition_fingerprints: conditions
            .iter()
            .map(|c| c.definition_fingerprint().as_str())
            .collect(),
    })
    .map_err(registry_error)?;
    Ok(ResolvedInstanceProfile {
        layered,
        stable_role_registry,
        artifact_kind_registry: kind_registry,
        artifact_instances,
        vocabulary,
        context_resolution,
        resolved_profile_fingerprint,
    })
}

fn load_profile_sources(
    repo: &Path,
    bindings: &[DefinitionSourceBinding],
) -> Result<Vec<AuthoredProfileSource>, ProfileLoadError> {
    let mut out = Vec::new();
    let mut budget = SourceByteBudget::default();
    for binding in bindings {
        let path = path_for(binding)?;
        let (_, bytes) =
            read_trusted_repo_source(repo, &path, &mut budget).map_err(registry_error)?;
        let source = parse_profile_source(&bytes)?;
        if source.exact_ref() != &binding.definition_ref {
            return Err(ProfileLoadError::new(
                ProfileLoadErrorKind::SourceIdentityMismatch,
                "profile source identity does not match binding",
            ));
        }
        out.push(source);
    }
    Ok(out)
}

fn stable_selection(
    v: &Value,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), ProfileLoadError> {
    let o = v
        .as_object()
        .ok_or_else(|| unsupported("stable role registry selection must be an object"))?;
    if o.len() != 2 {
        return Err(unsupported("stable role registry selection must be closed"));
    }
    let r = scalar_ref(
        o.get("ref").ok_or_else(|| missing("stable role ref"))?,
        "stable role ref",
    )?;
    let f = o
        .get("fingerprint")
        .and_then(Value::as_str)
        .ok_or_else(|| missing("stable role fingerprint"))?;
    Ok((r, DefinitionFingerprint::parse(f).map_err(registry_error)?))
}
fn union_refs(
    sources: &[&AuthoredProfileSource],
    field: ProfileField,
) -> Result<Vec<ExactDefinitionRef>, ProfileLoadError> {
    let mut out = BTreeSet::new();
    for source in sources {
        if let Some(v) = source.field(field) {
            for item in v
                .as_array()
                .ok_or_else(|| unsupported("profile definition source field must be an array"))?
            {
                out.insert(scalar_ref(item, field.key())?);
            }
        }
    }
    Ok(out.into_iter().collect())
}
fn union_scalar_refs(
    sources: &[&AuthoredProfileSource],
    field: ProfileField,
) -> Result<Vec<ExactDefinitionRef>, ProfileLoadError> {
    let mut out = BTreeSet::new();
    for source in sources {
        if let Some(v) = source.field(field) {
            out.insert(scalar_ref(v, field.key())?);
        }
    }
    Ok(out.into_iter().collect())
}
fn scalar_ref(v: &Value, label: &str) -> Result<ExactDefinitionRef, ProfileLoadError> {
    ExactDefinitionRef::parse(
        v.as_str()
            .ok_or_else(|| unsupported(format!("{label} must be an exact ref")))?,
    )
    .map_err(registry_error)
}
fn paths_for(
    bindings: &[DefinitionSourceBinding],
    refs: &[ExactDefinitionRef],
) -> Result<Vec<String>, ProfileLoadError> {
    refs.iter()
        .map(|r| path_for(find_binding(bindings, r)?))
        .collect()
}
fn all_paths(bindings: &[DefinitionSourceBinding]) -> Result<Vec<String>, ProfileLoadError> {
    bindings.iter().map(path_for).collect()
}
fn find_binding<'a>(
    bindings: &'a [DefinitionSourceBinding],
    reference: &ExactDefinitionRef,
) -> Result<&'a DefinitionSourceBinding, ProfileLoadError> {
    bindings
        .iter()
        .find(|b| &b.definition_ref == reference)
        .ok_or_else(|| missing("exact typed definition source"))
}
fn path_for(binding: &DefinitionSourceBinding) -> Result<String, ProfileLoadError> {
    match &binding.source {
        DefinitionSource::RepositoryPath(path) => Ok(path.clone()),
        DefinitionSource::BuiltIn(reference) => builtin_path(reference)
            .map(str::to_owned)
            .ok_or_else(|| unsupported("built-in definition is not allowlisted")),
    }
}
fn builtin_path(reference: &ExactDefinitionRef) -> Option<&'static str> {
    Some(match reference.as_str() {
    "handbook.profile.shipped-root@1.0.0" => "definitions/profiles/handbook.profile.shipped-root/1.0.0.yaml",
    "handbook.schemas.artifacts.project-authority@1.0.0" => "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml", "handbook.schemas.artifacts.project-context@1.0.0" => "definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml", "handbook.schemas.artifacts.environment-context@1.0.0" => "definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.entry.yaml", "handbook.schemas.artifacts.work-specification@1.0.0" => "definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.entry.yaml", "handbook.schemas.artifacts.decision-record@1.0.0" => "definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.entry.yaml", "handbook.schemas.artifacts.risk-record@1.0.0" => "definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.entry.yaml",
    "handbook.artifact-kind.project-authority@1.0.0" => "definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml", "handbook.artifact-kind.project-context@1.0.0" => "definitions/artifact-kinds/handbook.artifact-kind.project-context/1.0.0.yaml", "handbook.artifact-kind.environment-context@1.0.0" => "definitions/artifact-kinds/handbook.artifact-kind.environment-context/1.0.0.yaml", "handbook.artifact-kind.work-specification@1.0.0" => "definitions/artifact-kinds/handbook.artifact-kind.work-specification/1.0.0.yaml", "handbook.artifact-kind.decision-record@1.0.0" => "definitions/artifact-kinds/handbook.artifact-kind.decision-record/1.0.0.yaml", "handbook.artifact-kind.risk-record@1.0.0" => "definitions/artifact-kinds/handbook.artifact-kind.risk-record/1.0.0.yaml",
    "handbook.capabilities.constitutional-root@1.0.0" => "definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml", "handbook.semantic-validation.constitutional-root@1.0.0" => "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml", "handbook.condition.project.managed-operational-surface@1.0.0" => "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml", "handbook.vocabulary.shipped-root@1.0.0" => "definitions/vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml", "handbook.context-resolution.shipped-root@1.0.0" => "definitions/context-resolution-stacks/handbook.context-resolution.shipped-root/1.0.0.yaml", "handbook.mutation-matcher.core@1.0.0" => "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml", "handbook.resolution-escalation.core@1.0.0" => "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml", "handbook.memory-promotion.core@1.0.0" => "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml", _ => return None,
})
}
fn load_conditions(
    repo: &Path,
    bindings: &[DefinitionSourceBinding],
) -> Result<Vec<ProjectConditionDefinition>, ProfileLoadError> {
    let mut out = Vec::new();
    for binding in bindings {
        let path = path_for(binding)?;
        let condition = ProjectConditionDefinition::load(repo, &path).map_err(registry_error)?;
        if condition.exact_ref() != &binding.definition_ref {
            return Err(ProfileLoadError::new(
                ProfileLoadErrorKind::SourceIdentityMismatch,
                "condition source identity mismatch",
            ));
        }
        out.push(condition);
    }
    Ok(out)
}
fn require_empty_array(v: &Value, label: &str) -> Result<(), ProfileLoadError> {
    if v.as_array().is_some_and(Vec::is_empty) {
        Ok(())
    } else {
        Err(unsupported(format!("{label} must be empty")))
    }
}
fn registry_error(error: crate::definition_identity::RegistryLoadError) -> ProfileLoadError {
    ProfileLoadError::new(ProfileLoadErrorKind::SourceIdentityMismatch, error.detail())
}
fn unsupported(value: impl Into<String>) -> ProfileLoadError {
    ProfileLoadError::new(ProfileLoadErrorKind::SourceIdentityMismatch, value)
}
fn missing(value: &str) -> ProfileLoadError {
    ProfileLoadError::new(
        ProfileLoadErrorKind::MissingSource,
        format!("missing {value}"),
    )
}
fn fingerprint(value: &str) -> ProfileLoadError {
    ProfileLoadError::new(
        ProfileLoadErrorKind::SourceIdentityMismatch,
        format!("{value} fingerprint mismatch"),
    )
}
#[derive(Serialize)]
struct AncestryMember<'a> {
    profile_ref: &'a str,
    profile_fingerprint: &'a str,
}
#[derive(Serialize)]
struct DecisionMember<'a> {
    field: &'a str,
    source_profile_ref: &'a str,
    disposition: &'a str,
}
#[derive(Serialize)]
struct ResolvedClosure<'a> {
    selected_profile_ref: &'a str,
    ancestry: Vec<AncestryMember<'a>>,
    layer_decisions: Vec<DecisionMember<'a>>,
    stable_role_registry_fingerprint: &'a str,
    schema_registry_fingerprint: &'a str,
    artifact_kind_registry_fingerprint: &'a str,
    artifact_instance_registry_fingerprint: &'a str,
    vocabulary_fingerprint: &'a str,
    context_resolution_fingerprint: &'a str,
    condition_fingerprints: Vec<&'a str>,
}
