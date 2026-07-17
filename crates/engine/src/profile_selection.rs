use crate::artifact_instance::ArtifactInstanceRegistry;
use crate::artifact_kind_registry::{
    admitted_artifact_kind_exact_ref, load_artifact_kind_registry_admitted, ArtifactKindRegistry,
};
use crate::context_resolution_registry::{
    admitted_context_resolution_policy_exact_ref, admitted_context_resolution_stack_exact_ref,
    ContextResolutionPolicyRegistry, ContextResolutionStackDefinition,
};
use crate::definition_identity::{
    fingerprint_serializable, DefinitionFingerprint, ExactDefinitionRef, RegistryLoadErrorKind,
};
use crate::instance_profile::{
    admit_selection_request, layer_profile_sources, parse_profile_source, AdmittedDefinitionSource,
    AdmittedSourceIdentity, AuthoredProfileSource, DefinitionClass, LayeredProfile, ProfileField,
    ProfileLoadError, ProfileLoadErrorKind, ProfileSelectionRequest,
};
use crate::project_condition_registry::{
    admitted_project_condition_exact_ref, ProjectConditionRegistry,
};
use crate::schema_registry::{
    admitted_schema_entry_exact_ref, AdmittedSchemaEntrySource, SchemaRegistry, SchemaSourceKind,
};
use crate::semantic_capability_registry::{
    admitted_semantic_capability_exact_ref, admitted_semantic_validator_exact_ref,
    SemanticCapabilityRegistry,
};
use crate::stable_role_registry::{admitted_stable_role_registry_exact_ref, StableRoleRegistry};
use crate::vocabulary_registry::{
    admitted_vocabulary_exact_ref, admitted_vocabulary_stable_role_selection, VocabularyDefinition,
};
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
    let admitted = admit_selection_request(repo, request)?;
    let (request, admitted_sources, mut source_budget) = admitted.into_parts();

    // Stage 5: decode every retained source as its exact closed record class,
    // then bind its derived identity before any ancestry or closure work.
    validate_admitted_source_identities(&admitted_sources)?;
    let profile_sources = load_profile_sources(&admitted_sources)?;

    // Stage 6: select and validate one bounded ancestry before resolving any
    // dependency fingerprint.
    let layered = layer_profile_sources(&request.selected_profile_ref, profile_sources.clone())?;
    let ancestry = layered.ancestry().iter().cloned().collect::<BTreeSet<_>>();
    let selected_sources = layered
        .ancestry()
        .iter()
        .map(|reference| {
            profile_sources
                .iter()
                .find(|source| source.exact_ref() == reference)
                .expect("layering retained every selected source")
        })
        .collect::<Vec<_>>();
    let stable_selections = selected_sources
        .iter()
        .filter_map(|source| source.field(ProfileField::StableRoleRegistry))
        .map(stable_selection)
        .collect::<Result<Vec<_>, _>>()?;
    let mut stable_refs = stable_selections
        .iter()
        .map(|selection| selection.0.clone())
        .collect::<BTreeSet<_>>();
    let schema_refs = union_refs(&selected_sources, ProfileField::SchemaRegistrySources)?;
    let kind_refs = union_refs(&selected_sources, ProfileField::ArtifactKindSources)?;
    let vocabulary_refs = union_scalar_refs(&selected_sources, ProfileField::VocabularyRef)?;
    let stack_refs = union_scalar_refs(&selected_sources, ProfileField::ContextResolutionRef)?;

    // Stage 7: resolve the complete request-wide schema closure from retained
    // entry bytes and one shared definition-plus-document byte budget.
    let schema_sources = admitted_for_refs(
        &admitted_sources,
        DefinitionClass::SchemaEntry,
        &schema_refs,
    )?;
    let schema_inputs = schema_sources
        .iter()
        .map(|source| AdmittedSchemaEntrySource {
            declared_ref: source.definition_ref(),
            source_kind: match source.identity() {
                AdmittedSourceIdentity::BuiltIn { .. } => SchemaSourceKind::BuiltIn,
                AdmittedSourceIdentity::Repository { .. } => SchemaSourceKind::Repository,
            },
            bytes: source.bytes(),
        })
        .collect::<Vec<_>>();
    let union_schema_registry = SchemaRegistry::load_admitted_deferred_fingerprints(
        repo,
        &schema_inputs,
        &request.allowed_schema_roots,
        &mut source_budget,
    )
    .map_err(registry_error)?;

    // Stage 8: recompute the typed dependency graph in its frozen topological
    // order. No source is reopened after admission.
    let vocabulary_sources = admitted_for_refs(
        &admitted_sources,
        DefinitionClass::Vocabulary,
        &vocabulary_refs,
    )?;
    for source in &vocabulary_sources {
        let (reference, _) =
            admitted_vocabulary_stable_role_selection(source.bytes()).map_err(registry_error)?;
        stable_refs.insert(reference);
    }
    let mut stable_registries = BTreeMap::new();
    for reference in &stable_refs {
        let source = admitted_for_ref(
            &admitted_sources,
            DefinitionClass::StableRoleRegistry,
            reference,
        )?;
        if !matches!(source.identity(), AdmittedSourceIdentity::BuiltIn { .. }) {
            return Err(unsupported(
                "HCM-1.2 stable role registry must be exact built-in",
            ));
        }
        let registry = StableRoleRegistry::load_builtin(reference).map_err(registry_error)?;
        stable_registries.insert(reference.clone(), registry);
    }
    for (reference, supplied) in &stable_selections {
        let loaded = stable_registries
            .get(reference)
            .ok_or_else(|| missing("stable role registry"))?;
        if loaded.fingerprint() != supplied {
            return Err(fingerprint("stable role registry"));
        }
    }
    union_schema_registry
        .validate_fingerprints()
        .map_err(registry_error)?;

    let validator_refs = BTreeSet::from([exact(
        "handbook.semantic-validation.constitutional-root@1.0.0",
    )?]);
    let capability_refs =
        BTreeSet::from([exact("handbook.capabilities.constitutional-root@1.0.0")?]);
    let validator_sources = admitted_for_refs(
        &admitted_sources,
        DefinitionClass::SemanticValidator,
        &validator_refs.iter().cloned().collect::<Vec<_>>(),
    )?;
    let capability_sources = admitted_for_refs(
        &admitted_sources,
        DefinitionClass::SemanticCapability,
        &capability_refs.iter().cloned().collect::<Vec<_>>(),
    )?;
    let validator_inputs = validator_sources
        .iter()
        .map(|source| (source.definition_ref(), source.bytes()))
        .collect::<Vec<_>>();
    let capability_inputs = capability_sources
        .iter()
        .map(|source| (source.definition_ref(), source.bytes()))
        .collect::<Vec<_>>();
    let semantic_registry =
        SemanticCapabilityRegistry::load_admitted(&capability_inputs, &validator_inputs)
            .map_err(registry_error)?;

    let selected_stable = stable_selection(layered.field(ProfileField::StableRoleRegistry))?;
    let stable_role_registry = stable_registries
        .get(&selected_stable.0)
        .cloned()
        .ok_or_else(|| missing("selected stable role registry"))?;

    // Schema fingerprints are frozen by Stage 7. Resolve each root-to-leaf
    // replacement of the effective schema and kind registries now so the kind
    // producer remains ahead of conditions, vocabularies, policies, stacks,
    // descriptors, and authored-profile fingerprints in the Stage 8 DAG.
    let mut effective_stable_selection = None;
    let mut effective_schema_refs = None;
    let mut effective_kind_refs = None;
    let mut source_schema_registries = BTreeMap::new();
    let mut source_kind_registries = BTreeMap::new();
    for source in &selected_sources {
        if let Some(value) = source.field(ProfileField::StableRoleRegistry) {
            effective_stable_selection = Some(stable_selection(value)?);
        }
        if let Some(value) = source.field(ProfileField::SchemaRegistrySources) {
            effective_schema_refs = Some(replacement_refs(
                value,
                ProfileField::SchemaRegistrySources,
            )?);
        }
        if let Some(value) = source.field(ProfileField::ArtifactKindSources) {
            effective_kind_refs = Some(replacement_refs(value, ProfileField::ArtifactKindSources)?);
        }
        let effective_stable = effective_stable_selection
            .as_ref()
            .ok_or_else(|| missing("effective stable-role selection"))?;
        let effective_schemas = effective_schema_refs
            .as_ref()
            .ok_or_else(|| missing("effective schema-registry replacement"))?;
        let effective_kinds = effective_kind_refs
            .as_ref()
            .ok_or_else(|| missing("effective artifact-kind replacement"))?;
        let stable_registry = stable_registries
            .get(&effective_stable.0)
            .cloned()
            .ok_or_else(|| missing("effective stable-role registry"))?;
        if stable_registry.fingerprint() != &effective_stable.1 {
            return Err(fingerprint("effective stable-role registry"));
        }
        let schema_registry = union_schema_registry
            .select_entries(effective_schemas)
            .map_err(registry_error)?;
        let kind_sources = admitted_for_refs(
            &admitted_sources,
            DefinitionClass::ArtifactKind,
            effective_kinds,
        )?;
        let kind_inputs = kind_sources
            .iter()
            .map(|source| (source.definition_ref(), source.bytes()))
            .collect::<Vec<_>>();
        let kind_registry = load_artifact_kind_registry_admitted(
            stable_registry,
            schema_registry.clone(),
            semantic_registry.clone(),
            &kind_inputs,
        )
        .map_err(registry_error)?;
        source_schema_registries.insert(source.exact_ref().clone(), schema_registry);
        source_kind_registries.insert(source.exact_ref().clone(), kind_registry);
    }

    let kind_registry = source_kind_registries
        .get(layered.selected_profile_ref())
        .cloned()
        .ok_or_else(|| missing("final effective artifact-kind registry"))?;

    let condition_refs = descriptor_condition_refs(&selected_sources)?;
    let condition_sources = admitted_for_refs(
        &admitted_sources,
        DefinitionClass::ProjectCondition,
        &condition_refs,
    )?;
    let condition_inputs = condition_sources
        .iter()
        .map(|source| (source.definition_ref(), source.bytes()))
        .collect::<Vec<_>>();
    let conditions =
        ProjectConditionRegistry::load_admitted(&condition_inputs).map_err(registry_error)?;

    let mut vocabularies = BTreeMap::new();
    for source in vocabulary_sources {
        let vocabulary =
            VocabularyDefinition::load_bytes(source.bytes()).map_err(registry_error)?;
        require_derived_ref(
            "vocabulary",
            vocabulary.exact_ref(),
            source.definition_ref(),
        )?;
        let vocabulary_roles = stable_registries
            .get(vocabulary.stable_role_registry_ref())
            .ok_or_else(|| missing("vocabulary stable-role registry producer"))?;
        if vocabulary_roles.fingerprint() != vocabulary.stable_role_registry_fingerprint()
            || vocabulary.stable_role_registry_ref() != &selected_stable.0
            || vocabulary.stable_role_registry_fingerprint() != &selected_stable.1
        {
            return Err(stable_registry_mismatch(
                "vocabulary stable-role selection does not match the resolved profile registry",
            ));
        }
        vocabularies.insert(vocabulary.exact_ref().clone(), vocabulary);
    }

    let policy_refs = BTreeSet::from([
        exact("handbook.mutation-matcher.core@1.0.0")?,
        exact("handbook.resolution-escalation.core@1.0.0")?,
        exact("handbook.memory-promotion.core@1.0.0")?,
    ]);
    let policy_sources = admitted_for_refs(
        &admitted_sources,
        DefinitionClass::ContextResolutionPolicy,
        &policy_refs.iter().cloned().collect::<Vec<_>>(),
    )?;
    let policy_inputs = policy_sources
        .iter()
        .map(|source| (source.definition_ref(), source.bytes()))
        .collect::<Vec<_>>();
    let policies =
        ContextResolutionPolicyRegistry::load_admitted(&policy_inputs).map_err(registry_error)?;

    let stack_sources = admitted_for_refs(
        &admitted_sources,
        DefinitionClass::ContextResolution,
        &stack_refs,
    )?;
    let mut stacks = BTreeMap::new();
    for source in stack_sources {
        let stack = ContextResolutionStackDefinition::load_bytes(source.bytes(), &policies)
            .map_err(registry_error)?;
        require_derived_ref(
            "Context Resolution stack",
            stack.exact_ref(),
            source.definition_ref(),
        )?;
        stacks.insert(stack.exact_ref().clone(), stack);
    }

    let condition_values = conditions.values().collect::<Vec<_>>();
    let mut descriptor_registries = BTreeMap::new();
    for source in &selected_sources {
        if let Some(value) = source.field(ProfileField::ArtifactInstances) {
            let values = value.as_array().ok_or_else(|| {
                unsupported("profile artifact_instances replacement must be an array")
            })?;
            let effective_kinds = source_kind_registries
                .get(source.exact_ref())
                .ok_or_else(|| missing("descriptor artifact-kind registry"))?;
            let registry =
                ArtifactInstanceRegistry::resolve(values, effective_kinds, &condition_values)
                    .map_err(registry_error)?;
            descriptor_registries.insert(source.exact_ref().clone(), registry);
        }
    }

    validate_authored_profile_fingerprints(
        &selected_sources,
        &stable_registries,
        &source_schema_registries,
        &source_kind_registries,
        &vocabularies,
        &policies,
        &stacks,
        &descriptor_registries,
    )?;

    // Stage 9: require literal source-use equality. Shadowed selected-ancestor
    // dependencies are in the unions above; genuinely unrelated sources are
    // refused here rather than silently ignored.
    ensure_source_set(
        "profile",
        &admitted_sources,
        DefinitionClass::Profile,
        &ancestry,
    )?;
    ensure_source_set(
        "stable role registry",
        &admitted_sources,
        DefinitionClass::StableRoleRegistry,
        &stable_refs,
    )?;
    ensure_source_set(
        "schema entry",
        &admitted_sources,
        DefinitionClass::SchemaEntry,
        &schema_refs.iter().cloned().collect(),
    )?;
    ensure_source_set(
        "artifact kind",
        &admitted_sources,
        DefinitionClass::ArtifactKind,
        &kind_refs.iter().cloned().collect(),
    )?;
    ensure_source_set(
        "semantic capability",
        &admitted_sources,
        DefinitionClass::SemanticCapability,
        &capability_refs,
    )?;
    ensure_source_set(
        "semantic validator",
        &admitted_sources,
        DefinitionClass::SemanticValidator,
        &validator_refs,
    )?;
    ensure_source_set(
        "project condition",
        &admitted_sources,
        DefinitionClass::ProjectCondition,
        &condition_refs.iter().cloned().collect(),
    )?;
    ensure_source_set(
        "vocabulary",
        &admitted_sources,
        DefinitionClass::Vocabulary,
        &vocabulary_refs.iter().cloned().collect(),
    )?;
    ensure_source_set(
        "Context Resolution stack",
        &admitted_sources,
        DefinitionClass::ContextResolution,
        &stack_refs.iter().cloned().collect(),
    )?;
    ensure_source_set(
        "Context Resolution policy",
        &admitted_sources,
        DefinitionClass::ContextResolutionPolicy,
        &policy_refs,
    )?;

    // Stage 10: reject later-owned values on every selected ancestry source
    // before checking the final winner, so shadowing cannot hide them while
    // preserving the frozen Stage-9 source-use precedence.
    validate_selected_later_owned_fields(&selected_sources)?;

    // Final winning-field and resolved-profile invariants.
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
        .is_none_or(|map| !map.is_empty())
    {
        return Err(unsupported("profile extensions must be empty"));
    }

    let artifact_instance_values = layered
        .field(ProfileField::ArtifactInstances)
        .as_array()
        .ok_or_else(|| unsupported("winning artifact_instances must be an array"))?;
    let artifact_instances = ArtifactInstanceRegistry::resolve(
        artifact_instance_values,
        &kind_registry,
        &condition_values,
    )
    .map_err(registry_error)?;
    let vocabulary_ref = scalar_ref(layered.field(ProfileField::VocabularyRef), "vocabulary_ref")?;
    let vocabulary = vocabularies
        .get(&vocabulary_ref)
        .cloned()
        .ok_or_else(|| missing("winning vocabulary"))?;
    let stack_ref = scalar_ref(
        layered.field(ProfileField::ContextResolutionRef),
        "context_resolution_ref",
    )?;
    let context_resolution = stacks
        .get(&stack_ref)
        .cloned()
        .ok_or_else(|| missing("winning Context Resolution stack"))?;

    let ancestry_closure = layered
        .ancestry()
        .iter()
        .map(|reference| {
            let source = profile_sources
                .iter()
                .find(|source| source.exact_ref() == reference)
                .expect("layered ancestry came from admitted sources");
            AncestryMember {
                profile_ref: source.exact_ref().as_str(),
                profile_fingerprint: source.profile_fingerprint().as_str(),
            }
        })
        .collect();
    let decisions = layered
        .decisions()
        .iter()
        .map(|decision| DecisionMember {
            field: decision.field().key(),
            source_profile_ref: decision.source_profile_ref().as_str(),
            disposition: match decision.disposition() {
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
            .values()
            .map(|condition| condition.definition_fingerprint().as_str())
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

fn validate_admitted_source_identities(
    admitted: &[AdmittedDefinitionSource],
) -> Result<(), ProfileLoadError> {
    let mut sources = admitted.iter().collect::<Vec<_>>();
    sources.sort_by(|left, right| {
        (left.class(), left.definition_ref()).cmp(&(right.class(), right.definition_ref()))
    });
    for source in sources {
        let derived = match source.class() {
            DefinitionClass::Profile => parse_profile_source(source.bytes())?.exact_ref().clone(),
            DefinitionClass::StableRoleRegistry => {
                admitted_stable_role_registry_exact_ref(source.bytes()).map_err(registry_error)?
            }
            DefinitionClass::SchemaEntry => {
                admitted_schema_entry_exact_ref(source.bytes()).map_err(registry_error)?
            }
            DefinitionClass::ArtifactKind => {
                admitted_artifact_kind_exact_ref(source.bytes()).map_err(registry_error)?
            }
            DefinitionClass::SemanticCapability => {
                admitted_semantic_capability_exact_ref(source.bytes()).map_err(registry_error)?
            }
            DefinitionClass::SemanticValidator => {
                admitted_semantic_validator_exact_ref(source.bytes()).map_err(registry_error)?
            }
            DefinitionClass::ProjectCondition => {
                admitted_project_condition_exact_ref(source.bytes()).map_err(registry_error)?
            }
            DefinitionClass::Vocabulary => {
                admitted_vocabulary_exact_ref(source.bytes()).map_err(registry_error)?
            }
            DefinitionClass::ContextResolution => {
                admitted_context_resolution_stack_exact_ref(source.bytes())
                    .map_err(registry_error)?
            }
            DefinitionClass::ContextResolutionPolicy => {
                admitted_context_resolution_policy_exact_ref(source.bytes())
                    .map_err(registry_error)?
            }
        };
        require_derived_ref("definition source", &derived, source.definition_ref())?;
    }
    Ok(())
}

fn load_profile_sources(
    admitted: &[AdmittedDefinitionSource],
) -> Result<Vec<AuthoredProfileSource>, ProfileLoadError> {
    admitted
        .iter()
        .filter(|source| source.class() == DefinitionClass::Profile)
        .map(|source| {
            let profile = parse_profile_source(source.bytes())?;
            require_derived_ref("profile", profile.exact_ref(), source.definition_ref())?;
            Ok(profile)
        })
        .collect()
}

fn admitted_for_ref<'a>(
    sources: &'a [AdmittedDefinitionSource],
    class: DefinitionClass,
    reference: &ExactDefinitionRef,
) -> Result<&'a AdmittedDefinitionSource, ProfileLoadError> {
    sources
        .iter()
        .find(|source| source.class() == class && source.definition_ref() == reference)
        .ok_or_else(|| missing("exact typed definition source"))
}

fn admitted_for_refs<'a>(
    sources: &'a [AdmittedDefinitionSource],
    class: DefinitionClass,
    references: &[ExactDefinitionRef],
) -> Result<Vec<&'a AdmittedDefinitionSource>, ProfileLoadError> {
    references
        .iter()
        .map(|reference| admitted_for_ref(sources, class, reference))
        .collect()
}

fn ensure_source_set(
    label: &str,
    sources: &[AdmittedDefinitionSource],
    class: DefinitionClass,
    expected: &BTreeSet<ExactDefinitionRef>,
) -> Result<(), ProfileLoadError> {
    let actual = sources
        .iter()
        .filter(|source| source.class() == class)
        .map(|source| source.definition_ref().clone())
        .collect::<BTreeSet<_>>();
    if &actual == expected {
        Ok(())
    } else {
        Err(ProfileLoadError::new(
            ProfileLoadErrorKind::UnreferencedSource,
            format!("{label} sources contain a missing or genuinely unrelated identity"),
        ))
    }
}

fn require_derived_ref(
    label: &str,
    derived: &ExactDefinitionRef,
    declared: &ExactDefinitionRef,
) -> Result<(), ProfileLoadError> {
    if derived == declared {
        Ok(())
    } else {
        Err(ProfileLoadError::new(
            ProfileLoadErrorKind::SourceIdentityMismatch,
            format!("{label} derived exact ref does not match its typed source binding"),
        ))
    }
}

fn stable_selection(
    value: &Value,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), ProfileLoadError> {
    let object = value
        .as_object()
        .ok_or_else(|| unsupported("stable role registry selection must be an object"))?;
    if object.len() != 2 {
        return Err(unsupported("stable role registry selection must be closed"));
    }
    let reference = scalar_ref(
        object
            .get("ref")
            .ok_or_else(|| missing("stable role ref"))?,
        "stable role ref",
    )?;
    let fingerprint = object
        .get("fingerprint")
        .and_then(Value::as_str)
        .ok_or_else(|| missing("stable role fingerprint"))?;
    Ok((
        reference,
        DefinitionFingerprint::parse(fingerprint).map_err(registry_error)?,
    ))
}

fn union_refs(
    sources: &[&AuthoredProfileSource],
    field: ProfileField,
) -> Result<Vec<ExactDefinitionRef>, ProfileLoadError> {
    let mut references = BTreeSet::new();
    for source in sources {
        if let Some(value) = source.field(field) {
            for reference in replacement_refs(value, field)? {
                references.insert(reference);
            }
        }
    }
    Ok(references.into_iter().collect())
}

fn replacement_refs(
    value: &Value,
    field: ProfileField,
) -> Result<Vec<ExactDefinitionRef>, ProfileLoadError> {
    let mut references = BTreeSet::new();
    for item in value
        .as_array()
        .ok_or_else(|| unsupported("profile definition source field must be an array"))?
    {
        let reference = scalar_ref(item, field.key())?;
        if !references.insert(reference) {
            return Err(ProfileLoadError::at(
                ProfileLoadErrorKind::DuplicateProfileDependency,
                field.key(),
                "profile replacement contains a duplicate exact ref",
            ));
        }
    }
    Ok(references.into_iter().collect())
}

fn union_scalar_refs(
    sources: &[&AuthoredProfileSource],
    field: ProfileField,
) -> Result<Vec<ExactDefinitionRef>, ProfileLoadError> {
    let mut references = BTreeSet::new();
    for source in sources {
        if let Some(value) = source.field(field) {
            references.insert(scalar_ref(value, field.key())?);
        }
    }
    Ok(references.into_iter().collect())
}

fn descriptor_condition_refs(
    sources: &[&AuthoredProfileSource],
) -> Result<Vec<ExactDefinitionRef>, ProfileLoadError> {
    let mut references = BTreeSet::new();
    for source in sources {
        let Some(descriptors) = source.field(ProfileField::ArtifactInstances) else {
            continue;
        };
        let Some(descriptors) = descriptors.as_array() else {
            continue;
        };
        for descriptor in descriptors {
            let Some(requiredness) = descriptor.get("requiredness").and_then(Value::as_object)
            else {
                continue;
            };
            if requiredness.get("mode").and_then(Value::as_str) != Some("conditional") {
                continue;
            }
            if let Some(reference) = requiredness.get("condition_ref").and_then(Value::as_str) {
                references.insert(ExactDefinitionRef::parse(reference).map_err(registry_error)?);
            }
        }
    }
    Ok(references.into_iter().collect())
}

fn scalar_ref(value: &Value, label: &str) -> Result<ExactDefinitionRef, ProfileLoadError> {
    ExactDefinitionRef::parse(
        value
            .as_str()
            .ok_or_else(|| unsupported(format!("{label} must be an exact ref")))?,
    )
    .map_err(registry_error)
}

#[allow(clippy::too_many_arguments)]
fn validate_authored_profile_fingerprints(
    selected_sources: &[&AuthoredProfileSource],
    stable_registries: &BTreeMap<ExactDefinitionRef, StableRoleRegistry>,
    schemas: &BTreeMap<ExactDefinitionRef, SchemaRegistry>,
    kinds: &BTreeMap<ExactDefinitionRef, ArtifactKindRegistry>,
    vocabularies: &BTreeMap<ExactDefinitionRef, VocabularyDefinition>,
    policies: &ContextResolutionPolicyRegistry,
    stacks: &BTreeMap<ExactDefinitionRef, ContextResolutionStackDefinition>,
    descriptors: &BTreeMap<ExactDefinitionRef, ArtifactInstanceRegistry>,
) -> Result<(), ProfileLoadError> {
    let mut validated_profiles = BTreeMap::new();
    for source in selected_sources {
        let source_schemas = schemas
            .get(source.exact_ref())
            .ok_or_else(|| missing("profile effective schema registry"))?;
        let source_kinds = kinds
            .get(source.exact_ref())
            .ok_or_else(|| missing("profile effective artifact-kind registry"))?;
        let mut dependencies = Vec::new();
        if let Some(parent_ref) = source.parent_ref() {
            let parent_fingerprint = validated_profiles.get(parent_ref).ok_or_else(|| {
                ProfileLoadError::new(
                    ProfileLoadErrorKind::InvalidProfileAncestry,
                    "profile parent fingerprint is absent from root-to-leaf validation",
                )
            })?;
            dependencies.push(ProfileDependencyFingerprint::new(
                "profile",
                parent_ref,
                parent_fingerprint,
            ));
        }
        if let Some(value) = source.field(ProfileField::StableRoleRegistry) {
            let (reference, supplied) = stable_selection(value)?;
            let registry = stable_registries
                .get(&reference)
                .ok_or_else(|| missing("profile stable-role dependency"))?;
            if registry.fingerprint() != &supplied {
                return Err(fingerprint("profile stable-role dependency"));
            }
            dependencies.push(ProfileDependencyFingerprint::new(
                "stable_role_registry",
                &reference,
                registry.fingerprint(),
            ));
        }
        append_definition_dependencies(
            source,
            ProfileField::SchemaRegistrySources,
            "schema_entry",
            &mut dependencies,
            |reference| {
                source_schemas
                    .entry(reference)
                    .map(|entry| entry.entry_fingerprint().clone())
            },
        )?;
        append_definition_dependencies(
            source,
            ProfileField::ArtifactKindSources,
            "artifact_kind",
            &mut dependencies,
            |reference| {
                source_kinds
                    .kind(reference)
                    .map(|kind| kind.definition_fingerprint().clone())
            },
        )?;
        if source.field(ProfileField::ArtifactInstances).is_some() {
            let registry = descriptors
                .get(source.exact_ref())
                .ok_or_else(|| missing("profile artifact descriptor closure"))?;
            dependencies.push(ProfileDependencyFingerprint::new(
                "artifact_instance_registry",
                source.exact_ref(),
                registry.fingerprint(),
            ));
        }
        if let Some(value) = source.field(ProfileField::VocabularyRef) {
            let reference = scalar_ref(value, ProfileField::VocabularyRef.key())?;
            let vocabulary = vocabularies
                .get(&reference)
                .ok_or_else(|| missing("profile vocabulary dependency"))?;
            dependencies.push(ProfileDependencyFingerprint::new(
                "vocabulary",
                &reference,
                vocabulary.vocabulary_fingerprint(),
            ));
        }
        if let Some(value) = source.field(ProfileField::ContextResolutionRef) {
            let reference = scalar_ref(value, ProfileField::ContextResolutionRef.key())?;
            let stack = stacks
                .get(&reference)
                .ok_or_else(|| missing("profile Context Resolution dependency"))?;
            dependencies.push(ProfileDependencyFingerprint::new(
                "context_resolution",
                &reference,
                stack.definition_fingerprint(),
            ));
        }
        // Policy producers are transitive stack dependencies. Touch the
        // registry here so an absent producer cannot be hidden by a stack.
        if source.field(ProfileField::ContextResolutionRef).is_some() && policies.refs().is_empty()
        {
            return Err(missing("profile Context Resolution policy dependency"));
        }
        dependencies.sort_by(|left, right| {
            (&left.definition_class, &left.reference)
                .cmp(&(&right.definition_class, &right.reference))
        });
        let computed = fingerprint_serializable(&ProfileSourceFingerprintClosure {
            definition: source.fingerprint_definition(),
            dependencies,
        })
        .map_err(registry_error)?;
        if &computed != source.profile_fingerprint() {
            return Err(ProfileLoadError::new(
                ProfileLoadErrorKind::FingerprintMismatch,
                "authored profile fingerprint does not match its complete typed closure",
            ));
        }
        validated_profiles.insert(source.exact_ref().clone(), computed);
    }
    Ok(())
}

fn validate_selected_later_owned_fields(
    selected_sources: &[&AuthoredProfileSource],
) -> Result<(), ProfileLoadError> {
    for source in selected_sources {
        for (field, label) in [
            (
                ProfileField::ProjectionCatalogRefs,
                "projection_catalog_refs",
            ),
            (ProfileField::DockRequirementRefs, "dock_requirement_refs"),
            (ProfileField::AdapterOverlayRefs, "adapter_overlay_refs"),
        ] {
            if let Some(value) = source.field(field) {
                require_empty_array(value, label)?;
            }
        }
        if source
            .field(ProfileField::PostureEvaluationPolicy)
            .is_some_and(|value| !value.is_null())
        {
            return Err(unsupported("posture_evaluation_policy must be null"));
        }
        if source
            .field(ProfileField::Extensions)
            .is_some_and(|value| value.as_object().is_none_or(|map| !map.is_empty()))
        {
            return Err(unsupported("profile extensions must be empty"));
        }
    }
    Ok(())
}

fn append_definition_dependencies(
    source: &AuthoredProfileSource,
    field: ProfileField,
    class: &'static str,
    dependencies: &mut Vec<ProfileDependencyFingerprint>,
    lookup: impl Fn(&ExactDefinitionRef) -> Option<DefinitionFingerprint>,
) -> Result<(), ProfileLoadError> {
    if let Some(value) = source.field(field) {
        for item in value
            .as_array()
            .ok_or_else(|| unsupported("profile definition source field must be an array"))?
        {
            let reference = scalar_ref(item, field.key())?;
            let fingerprint =
                lookup(&reference).ok_or_else(|| missing("profile exact definition dependency"))?;
            dependencies.push(ProfileDependencyFingerprint::new(
                class,
                &reference,
                &fingerprint,
            ));
        }
    }
    Ok(())
}

fn require_empty_array(value: &Value, label: &str) -> Result<(), ProfileLoadError> {
    if value.as_array().is_some_and(Vec::is_empty) {
        Ok(())
    } else {
        Err(unsupported(format!("{label} must be empty")))
    }
}

fn exact(value: &str) -> Result<ExactDefinitionRef, ProfileLoadError> {
    ExactDefinitionRef::parse(value).map_err(registry_error)
}

fn registry_error(error: crate::definition_identity::RegistryLoadError) -> ProfileLoadError {
    let kind = ProfileLoadErrorKind::Registry(error.kind());
    match error.location() {
        Some(location) => ProfileLoadError::at(kind, location, error.detail()),
        None => ProfileLoadError::new(kind, error.detail()),
    }
}

fn unsupported(value: impl Into<String>) -> ProfileLoadError {
    ProfileLoadError::new(
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::UnsupportedDependency),
        value,
    )
}

fn missing(value: &str) -> ProfileLoadError {
    ProfileLoadError::new(
        ProfileLoadErrorKind::MissingSource,
        format!("missing {value}"),
    )
}

fn fingerprint(value: &str) -> ProfileLoadError {
    ProfileLoadError::new(
        ProfileLoadErrorKind::FingerprintMismatch,
        format!("{value} fingerprint mismatch"),
    )
}

fn stable_registry_mismatch(value: &str) -> ProfileLoadError {
    ProfileLoadError::new(
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::StableRoleRegistryMismatch),
        value,
    )
}

#[derive(Serialize)]
struct ProfileDependencyFingerprint {
    definition_class: &'static str,
    reference: String,
    fingerprint: String,
}

impl ProfileDependencyFingerprint {
    fn new(
        definition_class: &'static str,
        reference: &ExactDefinitionRef,
        fingerprint: &DefinitionFingerprint,
    ) -> Self {
        Self {
            definition_class,
            reference: reference.as_str().to_owned(),
            fingerprint: fingerprint.as_str().to_owned(),
        }
    }
}

#[derive(Serialize)]
struct ProfileSourceFingerprintClosure<'a> {
    definition: &'a Value,
    dependencies: Vec<ProfileDependencyFingerprint>,
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
