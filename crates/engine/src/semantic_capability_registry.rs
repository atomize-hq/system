use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::instance_profile::SymbolicId;
use crate::stable_role_registry::read_trusted_repo_source;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const CAPABILITY_SCHEMA_ID: &str = "handbook.semantic-capability-contract";
const VALIDATOR_SCHEMA_ID: &str = "handbook.semantic-validation-profile-definition";
const RECORD_SCHEMA_VERSION: &str = "1.0";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingJsonType {
    Object,
    Array,
    String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingCardinality {
    Singular,
    Plural,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingEmptyPolicy {
    Forbidden,
    Allowed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredBindingRule {
    rule_id: String,
    binding_key: String,
    json_type: BindingJsonType,
    cardinality: BindingCardinality,
    empty_policy: BindingEmptyPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticBindingRule {
    rule_id: SymbolicId,
    binding_key: SymbolicId,
    json_type: BindingJsonType,
    cardinality: BindingCardinality,
    empty_policy: BindingEmptyPolicy,
}

impl SemanticBindingRule {
    pub fn rule_id(&self) -> &SymbolicId {
        &self.rule_id
    }
    pub fn binding_key(&self) -> &SymbolicId {
        &self.binding_key
    }
    pub fn json_type(&self) -> BindingJsonType {
        self.json_type
    }
    pub fn cardinality(&self) -> BindingCardinality {
        self.cardinality
    }
    pub fn empty_policy(&self) -> BindingEmptyPolicy {
        self.empty_policy
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredValidatorProfile {
    schema_id: String,
    schema_version: String,
    profile_id: String,
    profile_version: String,
    capability_id: String,
    binding_rules: Vec<AuthoredBindingRule>,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    profile_fingerprint: String,
}

#[derive(Clone, Debug)]
pub struct SemanticValidationProfileDefinition {
    exact_ref: ExactDefinitionRef,
    capability_id: SymbolicId,
    binding_rules: Vec<SemanticBindingRule>,
    profile_fingerprint: DefinitionFingerprint,
}

impl SemanticValidationProfileDefinition {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }
    pub fn capability_id(&self) -> &SymbolicId {
        &self.capability_id
    }
    pub fn binding_rules(&self) -> &[SemanticBindingRule] {
        &self.binding_rules
    }
    pub fn profile_fingerprint(&self) -> &DefinitionFingerprint {
        &self.profile_fingerprint
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedInstanceCardinality {
    ExactlyOne,
    AtLeastOne,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCapabilityContract {
    schema_id: String,
    schema_version: String,
    contract_id: String,
    contract_version: String,
    capability_id: String,
    required_bindings: Vec<String>,
    semantic_validation_profile_refs: Vec<String>,
    allowed_instance_cardinality: AllowedInstanceCardinality,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    capability_fingerprint: String,
}

#[derive(Clone, Debug)]
pub struct SemanticCapabilityDefinition {
    exact_ref: ExactDefinitionRef,
    capability_id: SymbolicId,
    required_bindings: Vec<SymbolicId>,
    semantic_validation_profile_refs: Vec<ExactDefinitionRef>,
    allowed_instance_cardinality: AllowedInstanceCardinality,
    capability_fingerprint: DefinitionFingerprint,
}

impl SemanticCapabilityDefinition {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }
    pub fn capability_id(&self) -> &SymbolicId {
        &self.capability_id
    }
    pub fn required_bindings(&self) -> &[SymbolicId] {
        &self.required_bindings
    }
    pub fn semantic_validation_profile_refs(&self) -> &[ExactDefinitionRef] {
        &self.semantic_validation_profile_refs
    }
    pub fn allowed_instance_cardinality(&self) -> AllowedInstanceCardinality {
        self.allowed_instance_cardinality
    }
    pub fn capability_fingerprint(&self) -> &DefinitionFingerprint {
        &self.capability_fingerprint
    }
}

#[derive(Clone, Debug, Default)]
pub struct SemanticCapabilityRegistry {
    validators: BTreeMap<ExactDefinitionRef, SemanticValidationProfileDefinition>,
    capabilities: BTreeMap<ExactDefinitionRef, SemanticCapabilityDefinition>,
}

impl SemanticCapabilityRegistry {
    pub fn load(
        repo_root: impl AsRef<Path>,
        capability_paths: &[String],
        validator_paths: &[String],
    ) -> Result<Self, RegistryLoadError> {
        let mut budget = SourceByteBudget::default();
        let mut validators = BTreeMap::new();
        for path in validator_paths {
            let (_, bytes) = read_trusted_repo_source(repo_root.as_ref(), path, &mut budget)?;
            let authored: AuthoredValidatorProfile = decode(&bytes, "semantic validator")?;
            let definition = resolve_validator(authored)?;
            insert_unique(
                &mut validators,
                definition.exact_ref.clone(),
                definition,
                "semantic validator",
            )?;
        }
        let mut capabilities = BTreeMap::new();
        for path in capability_paths {
            let (_, bytes) = read_trusted_repo_source(repo_root.as_ref(), path, &mut budget)?;
            let authored: AuthoredCapabilityContract = decode(&bytes, "semantic capability")?;
            let definition = resolve_capability(authored, &validators)?;
            insert_unique(
                &mut capabilities,
                definition.exact_ref.clone(),
                definition,
                "semantic capability",
            )?;
        }
        Ok(Self {
            validators,
            capabilities,
        })
    }
    pub fn capability(&self, r: &ExactDefinitionRef) -> Option<&SemanticCapabilityDefinition> {
        self.capabilities.get(r)
    }
    pub fn validator(
        &self,
        r: &ExactDefinitionRef,
    ) -> Option<&SemanticValidationProfileDefinition> {
        self.validators.get(r)
    }
}

fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8], label: &str) -> Result<T, RegistryLoadError> {
    let value = parse_definition_yaml(bytes)?;
    serde_json::from_value(value).map_err(|error| {
        RegistryLoadError::new(
            if error.to_string().contains("unknown field") {
                RegistryLoadErrorKind::UnknownField
            } else {
                RegistryLoadErrorKind::SyntaxError
            },
            format!("{label} does not match its closed typed record"),
        )
    })
}

fn resolve_validator(
    authored: AuthoredValidatorProfile,
) -> Result<SemanticValidationProfileDefinition, RegistryLoadError> {
    if authored.schema_id != VALIDATOR_SCHEMA_ID
        || authored.schema_version != RECORD_SCHEMA_VERSION
        || !authored.extensions.is_empty()
    {
        return Err(unsupported("semantic validator record/schema/extensions"));
    }
    let exact_ref = ExactDefinitionRef::new(&authored.profile_id, &authored.profile_version)?;
    let capability_id = SymbolicId::parse(&authored.capability_id).map_err(profile_error)?;
    let mut seen = BTreeSet::new();
    let mut binding_rules = Vec::new();
    for rule in &authored.binding_rules {
        let rule_id = SymbolicId::parse(&rule.rule_id).map_err(profile_error)?;
        let binding_key = SymbolicId::parse(&rule.binding_key).map_err(profile_error)?;
        if rule_id != binding_key || !seen.insert(binding_key.clone()) {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::DuplicateIdentity,
                "validator rules must be unique and rule_id must equal binding_key",
            ));
        }
        binding_rules.push(SemanticBindingRule {
            rule_id,
            binding_key,
            json_type: rule.json_type,
            cardinality: rule.cardinality,
            empty_policy: rule.empty_policy,
        });
    }
    let supplied = DefinitionFingerprint::parse(&authored.profile_fingerprint)?;
    let computed = fingerprint_serializable(&authored)?;
    if supplied != computed {
        return Err(fingerprint_mismatch("semantic validator"));
    }
    Ok(SemanticValidationProfileDefinition {
        exact_ref,
        capability_id,
        binding_rules,
        profile_fingerprint: computed,
    })
}

fn resolve_capability(
    authored: AuthoredCapabilityContract,
    validators: &BTreeMap<ExactDefinitionRef, SemanticValidationProfileDefinition>,
) -> Result<SemanticCapabilityDefinition, RegistryLoadError> {
    if authored.schema_id != CAPABILITY_SCHEMA_ID
        || authored.schema_version != RECORD_SCHEMA_VERSION
        || !authored.extensions.is_empty()
    {
        return Err(unsupported("semantic capability record/schema/extensions"));
    }
    let exact_ref = ExactDefinitionRef::new(&authored.contract_id, &authored.contract_version)?;
    let capability_id = SymbolicId::parse(&authored.capability_id).map_err(profile_error)?;
    let mut seen = BTreeSet::new();
    let mut required_bindings = Vec::new();
    for value in &authored.required_bindings {
        let id = SymbolicId::parse(value).map_err(profile_error)?;
        if !seen.insert(id.clone()) {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::DuplicateIdentity,
                "capability binding key is duplicated",
            ));
        }
        required_bindings.push(id);
    }
    let mut validator_refs = Vec::new();
    let mut validator_fingerprints = Vec::new();
    for value in &authored.semantic_validation_profile_refs {
        let r = ExactDefinitionRef::parse(value)?;
        let v = validators.get(&r).ok_or_else(|| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "semantic validator source is absent",
            )
        })?;
        if v.capability_id() != &capability_id {
            return Err(unsupported("validator capability ID mismatch"));
        }
        if v.binding_rules()
            .iter()
            .map(|r| r.binding_key())
            .ne(required_bindings.iter())
        {
            return Err(unsupported(
                "validator binding rule order does not match capability",
            ));
        }
        validator_fingerprints.push(v.profile_fingerprint().as_str());
        validator_refs.push(r);
    }
    let supplied = DefinitionFingerprint::parse(&authored.capability_fingerprint)?;
    let computed = fingerprint_serializable(&CapabilityClosure {
        definition: &authored,
        semantic_validator_fingerprints: validator_fingerprints,
    })?;
    if supplied != computed {
        return Err(fingerprint_mismatch("semantic capability"));
    }
    Ok(SemanticCapabilityDefinition {
        exact_ref,
        capability_id,
        required_bindings,
        semantic_validation_profile_refs: validator_refs,
        allowed_instance_cardinality: authored.allowed_instance_cardinality,
        capability_fingerprint: computed,
    })
}

#[derive(Serialize)]
struct CapabilityClosure<'a> {
    definition: &'a AuthoredCapabilityContract,
    semantic_validator_fingerprints: Vec<&'a str>,
}
fn insert_unique<T>(
    map: &mut BTreeMap<ExactDefinitionRef, T>,
    key: ExactDefinitionRef,
    value: T,
    label: &str,
) -> Result<(), RegistryLoadError> {
    if map.insert(key, value).is_some() {
        Err(RegistryLoadError::new(
            RegistryLoadErrorKind::DuplicateIdentity,
            format!("{label} exact identity is duplicated"),
        ))
    } else {
        Ok(())
    }
}
fn unsupported(detail: &str) -> RegistryLoadError {
    RegistryLoadError::new(RegistryLoadErrorKind::UnsupportedDependency, detail)
}
fn fingerprint_mismatch(label: &str) -> RegistryLoadError {
    RegistryLoadError::new(
        RegistryLoadErrorKind::FingerprintMismatch,
        format!("{label} fingerprint does not match its exact typed closure"),
    )
}
fn profile_error(_: crate::instance_profile::ProfileLoadError) -> RegistryLoadError {
    RegistryLoadError::new(
        RegistryLoadErrorKind::InvalidExactDefinitionRef,
        "semantic definition contains an invalid SymbolicId",
    )
}
