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

pub type SemanticCapabilityContract = SemanticCapabilityDefinition;

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

    pub(crate) fn load_admitted(
        capability_sources: &[(&ExactDefinitionRef, &[u8])],
        validator_sources: &[(&ExactDefinitionRef, &[u8])],
    ) -> Result<Self, RegistryLoadError> {
        let mut validators = BTreeMap::new();
        for (declared_ref, bytes) in validator_sources {
            let definition = resolve_validator(decode(bytes, "semantic validator")?)?;
            if definition.exact_ref() != *declared_ref {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::ConflictingIdentity,
                    "semantic validator derived exact ref does not match its typed source binding",
                ));
            }
            insert_unique(
                &mut validators,
                definition.exact_ref.clone(),
                definition,
                "semantic validator",
            )?;
        }
        let mut capabilities = BTreeMap::new();
        for (declared_ref, bytes) in capability_sources {
            let definition =
                resolve_capability(decode(bytes, "semantic capability")?, &validators)?;
            if definition.exact_ref() != *declared_ref {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::ConflictingIdentity,
                    "semantic capability derived exact ref does not match its typed source binding",
                ));
            }
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

fn record_location(label: &str) -> &'static str {
    match label {
        "semantic validator" => "semantic_validator",
        "semantic capability" => "semantic_capability",
        _ => "semantic_definition",
    }
}

fn record_object<'a>(
    value: &'a Value,
    location: &str,
) -> Result<&'a serde_json::Map<String, Value>, RegistryLoadError> {
    value.as_object().ok_or_else(|| {
        RegistryLoadError::at(
            RegistryLoadErrorKind::SyntaxError,
            location,
            "semantic definition must be an object record",
        )
    })
}

fn reject_unknown_fields(
    object: &serde_json::Map<String, Value>,
    allowed: &[&str],
    location: &str,
) -> Result<(), RegistryLoadError> {
    if let Some(field) = object
        .keys()
        .filter(|field| !allowed.contains(&field.as_str()))
        .min()
    {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::UnknownField,
            format!("{location}/{field}"),
            "semantic definition contains an unknown field",
        ));
    }
    Ok(())
}

fn required_field<'a>(
    object: &'a serde_json::Map<String, Value>,
    field: &str,
    location: &str,
) -> Result<&'a Value, RegistryLoadError> {
    object.get(field).ok_or_else(|| {
        RegistryLoadError::at(
            RegistryLoadErrorKind::SyntaxError,
            format!("{location}/{field}"),
            "semantic definition is missing a required field",
        )
    })
}

fn require_string_field<'a>(
    object: &'a serde_json::Map<String, Value>,
    field: &str,
    location: &str,
) -> Result<&'a str, RegistryLoadError> {
    required_field(object, field, location)?
        .as_str()
        .ok_or_else(|| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::SyntaxError,
                format!("{location}/{field}"),
                "semantic definition field must be a string",
            )
        })
}

fn require_string_array(
    object: &serde_json::Map<String, Value>,
    field: &str,
    location: &str,
) -> Result<(), RegistryLoadError> {
    let values = required_field(object, field, location)?
        .as_array()
        .ok_or_else(|| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::SyntaxError,
                format!("{location}/{field}"),
                "semantic definition field must be an array",
            )
        })?;
    if let Some((index, _)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_string())
    {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::SyntaxError,
            format!("{location}/{field}/{index}"),
            "semantic definition array member must be a string",
        ));
    }
    Ok(())
}

fn require_object_field(
    object: &serde_json::Map<String, Value>,
    field: &str,
    location: &str,
) -> Result<(), RegistryLoadError> {
    if !required_field(object, field, location)?.is_object() {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::SyntaxError,
            format!("{location}/{field}"),
            "semantic definition field must be an object",
        ));
    }
    Ok(())
}

fn require_enum_field(
    object: &serde_json::Map<String, Value>,
    field: &str,
    allowed: &[&str],
    location: &str,
) -> Result<(), RegistryLoadError> {
    let value = require_string_field(object, field, location)?;
    if !allowed.contains(&value) {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::UnsupportedRecord,
            format!("{location}/{field}"),
            "semantic definition field value is unsupported",
        ));
    }
    Ok(())
}

fn validate_validator_record_shape(value: &Value) -> Result<(), RegistryLoadError> {
    const LOCATION: &str = "semantic_validator";
    const FIELDS: &[&str] = &[
        "schema_id",
        "schema_version",
        "profile_id",
        "profile_version",
        "capability_id",
        "binding_rules",
        "extensions",
        "profile_fingerprint",
    ];
    let object = record_object(value, LOCATION)?;
    reject_unknown_fields(object, FIELDS, LOCATION)?;
    for field in [
        "schema_id",
        "schema_version",
        "profile_id",
        "profile_version",
        "capability_id",
        "profile_fingerprint",
    ] {
        require_string_field(object, field, LOCATION)?;
    }
    require_object_field(object, "extensions", LOCATION)?;
    let rules = required_field(object, "binding_rules", LOCATION)?
        .as_array()
        .ok_or_else(|| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::SyntaxError,
                format!("{LOCATION}/binding_rules"),
                "semantic validator binding_rules must be an array",
            )
        })?;
    for (index, rule) in rules.iter().enumerate() {
        let location = format!("{LOCATION}/binding_rules/{index}");
        let rule = record_object(rule, &location)?;
        reject_unknown_fields(
            rule,
            &[
                "rule_id",
                "binding_key",
                "json_type",
                "cardinality",
                "empty_policy",
            ],
            &location,
        )?;
        require_string_field(rule, "rule_id", &location)?;
        require_string_field(rule, "binding_key", &location)?;
        require_enum_field(rule, "json_type", &["object", "array", "string"], &location)?;
        require_enum_field(rule, "cardinality", &["singular", "plural"], &location)?;
        require_enum_field(rule, "empty_policy", &["forbidden", "allowed"], &location)?;
    }
    Ok(())
}

fn validate_capability_record_shape(value: &Value) -> Result<(), RegistryLoadError> {
    const LOCATION: &str = "semantic_capability";
    const FIELDS: &[&str] = &[
        "schema_id",
        "schema_version",
        "contract_id",
        "contract_version",
        "capability_id",
        "required_bindings",
        "semantic_validation_profile_refs",
        "allowed_instance_cardinality",
        "extensions",
        "capability_fingerprint",
    ];
    let object = record_object(value, LOCATION)?;
    reject_unknown_fields(object, FIELDS, LOCATION)?;
    for field in [
        "schema_id",
        "schema_version",
        "contract_id",
        "contract_version",
        "capability_id",
        "capability_fingerprint",
    ] {
        require_string_field(object, field, LOCATION)?;
    }
    require_string_array(object, "required_bindings", LOCATION)?;
    require_string_array(object, "semantic_validation_profile_refs", LOCATION)?;
    require_enum_field(
        object,
        "allowed_instance_cardinality",
        &["exactly_one", "at_least_one"],
        LOCATION,
    )?;
    require_object_field(object, "extensions", LOCATION)?;
    Ok(())
}

fn validate_validator_record_header(
    authored: &AuthoredValidatorProfile,
) -> Result<(), RegistryLoadError> {
    if authored.schema_id != VALIDATOR_SCHEMA_ID {
        return Err(unsupported_record(
            "semantic_validator/schema_id",
            "semantic validator schema id is unsupported",
        ));
    }
    if authored.schema_version != RECORD_SCHEMA_VERSION {
        return Err(unsupported_record(
            "semantic_validator/schema_version",
            "semantic validator schema version is unsupported",
        ));
    }
    if !authored.extensions.is_empty() {
        return Err(unsupported_record(
            "semantic_validator/extensions",
            "semantic validator extensions must be empty",
        ));
    }
    Ok(())
}

fn validate_capability_record_header(
    authored: &AuthoredCapabilityContract,
) -> Result<(), RegistryLoadError> {
    if authored.schema_id != CAPABILITY_SCHEMA_ID {
        return Err(unsupported_record(
            "semantic_capability/schema_id",
            "semantic capability schema id is unsupported",
        ));
    }
    if authored.schema_version != RECORD_SCHEMA_VERSION {
        return Err(unsupported_record(
            "semantic_capability/schema_version",
            "semantic capability schema version is unsupported",
        ));
    }
    if !authored.extensions.is_empty() {
        return Err(unsupported_record(
            "semantic_capability/extensions",
            "semantic capability extensions must be empty",
        ));
    }
    Ok(())
}

fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8], label: &str) -> Result<T, RegistryLoadError> {
    let location = record_location(label);
    let value = parse_definition_yaml(bytes)
        .map_err(|error| RegistryLoadError::at(error.kind(), location, error.detail()))?;
    match label {
        "semantic validator" => validate_validator_record_shape(&value)?,
        "semantic capability" => validate_capability_record_shape(&value)?,
        _ => {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedRecord,
                location,
                "semantic definition record class is unsupported",
            ))
        }
    }
    serde_json::from_value(value).map_err(|error| {
        RegistryLoadError::at(
            if error.to_string().contains("unknown field") {
                RegistryLoadErrorKind::UnknownField
            } else {
                RegistryLoadErrorKind::SyntaxError
            },
            location,
            format!("{label} does not match its closed typed record"),
        )
    })
}

pub(crate) fn admitted_semantic_validator_exact_ref(
    bytes: &[u8],
) -> Result<ExactDefinitionRef, RegistryLoadError> {
    let authored: AuthoredValidatorProfile = decode(bytes, "semantic validator")?;
    validate_validator_record_header(&authored)?;
    ExactDefinitionRef::new(&authored.profile_id, &authored.profile_version)
}

pub(crate) fn admitted_semantic_capability_exact_ref(
    bytes: &[u8],
) -> Result<ExactDefinitionRef, RegistryLoadError> {
    let authored: AuthoredCapabilityContract = decode(bytes, "semantic capability")?;
    validate_capability_record_header(&authored)?;
    ExactDefinitionRef::new(&authored.contract_id, &authored.contract_version)
}

fn resolve_validator(
    authored: AuthoredValidatorProfile,
) -> Result<SemanticValidationProfileDefinition, RegistryLoadError> {
    validate_validator_record_header(&authored)?;
    let exact_ref = ExactDefinitionRef::new(&authored.profile_id, &authored.profile_version)?;
    let capability_id = SymbolicId::parse(&authored.capability_id).map_err(profile_error)?;
    if exact_ref.as_str() != "handbook.semantic-validation.constitutional-root@1.0.0"
        || capability_id.as_str() != "constitutional_root"
        || authored.binding_rules.len() != CONSTITUTIONAL_BINDINGS.len()
    {
        return Err(unsupported(
            "HCM-1.2 admits only the exact constitutional validator profile",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut binding_rules = Vec::new();
    for (index, rule) in authored.binding_rules.iter().enumerate() {
        let rule_id = SymbolicId::parse(&rule.rule_id).map_err(profile_error)?;
        let binding_key = SymbolicId::parse(&rule.binding_key).map_err(profile_error)?;
        if rule_id != binding_key || !seen.insert(binding_key.clone()) {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::DuplicateIdentity,
                "validator rules must be unique and rule_id must equal binding_key",
            ));
        }
        let (expected_type, expected_cardinality) = if index == 0 {
            (BindingJsonType::Object, BindingCardinality::Singular)
        } else if matches!(index, 2 | 3 | 5 | 6 | 7 | 8) {
            (BindingJsonType::Array, BindingCardinality::Plural)
        } else {
            (BindingJsonType::String, BindingCardinality::Singular)
        };
        if binding_key.as_str() != CONSTITUTIONAL_BINDINGS[index]
            || rule.json_type != expected_type
            || rule.cardinality != expected_cardinality
            || rule.empty_policy != BindingEmptyPolicy::Forbidden
        {
            return Err(unsupported(
                "constitutional validator rule differs from the exact nine-rule table",
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
    validate_capability_record_header(&authored)?;
    let exact_ref = ExactDefinitionRef::new(&authored.contract_id, &authored.contract_version)?;
    let capability_id = SymbolicId::parse(&authored.capability_id).map_err(profile_error)?;
    if exact_ref.as_str() != "handbook.capabilities.constitutional-root@1.0.0"
        || capability_id.as_str() != "constitutional_root"
        || authored.required_bindings != CONSTITUTIONAL_BINDINGS
        || authored.semantic_validation_profile_refs
            != ["handbook.semantic-validation.constitutional-root@1.0.0"]
        || authored.allowed_instance_cardinality != AllowedInstanceCardinality::ExactlyOne
    {
        return Err(unsupported(
            "HCM-1.2 admits only the exact constitutional capability contract",
        ));
    }
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
const CONSTITUTIONAL_BINDINGS: [&str; 9] = [
    "policy_root",
    "policy_revision",
    "decision_authority",
    "required_approvals",
    "exception_policy",
    "engineering_posture_dimensions",
    "red_lines",
    "review_triggers",
    "reassessment_triggers",
];
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
fn unsupported_record(location: &str, detail: &str) -> RegistryLoadError {
    RegistryLoadError::at(RegistryLoadErrorKind::UnsupportedRecord, location, detail)
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

#[cfg(test)]
mod record_error_tests {
    use super::{admitted_semantic_capability_exact_ref, admitted_semantic_validator_exact_ref};
    use crate::RegistryLoadErrorKind;
    use serde_json::Value;

    const VALIDATOR: &[u8] = include_bytes!(
        "../definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml"
    );
    const CAPABILITY: &[u8] = include_bytes!(
        "../definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml"
    );

    #[test]
    fn validator_stage_five_record_errors_precede_stale_fingerprints() {
        for case in [
            "schema_id",
            "schema_version",
            "extensions",
            "nested_unknown",
            "nested_wrong_type",
        ] {
            let mut value: Value = serde_yaml_bw::from_slice(VALIDATOR).unwrap();
            let (kind, location) = match case {
                "schema_id" => {
                    value["schema_id"] = Value::String("handbook.wrong".into());
                    (
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "semantic_validator/schema_id",
                    )
                }
                "schema_version" => {
                    value["schema_version"] = Value::String("2.0".into());
                    (
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "semantic_validator/schema_version",
                    )
                }
                "extensions" => {
                    value["extensions"]["future"] = Value::Bool(true);
                    (
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "semantic_validator/extensions",
                    )
                }
                "nested_unknown" => {
                    value["binding_rules"][0]["unexpected"] = Value::Bool(true);
                    (
                        RegistryLoadErrorKind::UnknownField,
                        "semantic_validator/binding_rules/0/unexpected",
                    )
                }
                _ => {
                    value["binding_rules"][0]["json_type"] = Value::Bool(true);
                    (
                        RegistryLoadErrorKind::SyntaxError,
                        "semantic_validator/binding_rules/0/json_type",
                    )
                }
            };
            let bytes = serde_yaml_bw::to_string(&value).unwrap();
            let error = admitted_semantic_validator_exact_ref(bytes.as_bytes()).unwrap_err();
            assert_eq!(error.kind(), kind, "{case}");
            assert_eq!(error.location(), Some(location), "{case}");
        }
    }

    #[test]
    fn capability_stage_five_record_errors_precede_stale_fingerprints() {
        for case in [
            "schema_id",
            "schema_version",
            "extensions",
            "unknown",
            "nested_wrong_type",
        ] {
            let mut value: Value = serde_yaml_bw::from_slice(CAPABILITY).unwrap();
            let (kind, location) = match case {
                "schema_id" => {
                    value["schema_id"] = Value::String("handbook.wrong".into());
                    (
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "semantic_capability/schema_id",
                    )
                }
                "schema_version" => {
                    value["schema_version"] = Value::String("2.0".into());
                    (
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "semantic_capability/schema_version",
                    )
                }
                "extensions" => {
                    value["extensions"]["future"] = Value::Bool(true);
                    (
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "semantic_capability/extensions",
                    )
                }
                "unknown" => {
                    value["unexpected"] = Value::Bool(true);
                    (
                        RegistryLoadErrorKind::UnknownField,
                        "semantic_capability/unexpected",
                    )
                }
                _ => {
                    value["required_bindings"][0] = Value::Bool(true);
                    (
                        RegistryLoadErrorKind::SyntaxError,
                        "semantic_capability/required_bindings/0",
                    )
                }
            };
            let bytes = serde_yaml_bw::to_string(&value).unwrap();
            let error = admitted_semantic_capability_exact_ref(bytes.as_bytes()).unwrap_err();
            assert_eq!(error.kind(), kind, "{case}");
            assert_eq!(error.location(), Some(location), "{case}");
        }
    }
}
