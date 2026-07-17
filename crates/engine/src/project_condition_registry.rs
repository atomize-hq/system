use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCondition {
    schema_id: String,
    schema_version: String,
    condition_id: String,
    condition_version: String,
    outcomes: Vec<String>,
    accepted_input_classes: Vec<String>,
    freshness_requirement: String,
    minimum_independent_current_bases: u8,
    self_reference_exclusions: Vec<String>,
    outcome_precedence: Vec<String>,
    effects: Vec<String>,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    definition_fingerprint: String,
}
#[derive(Clone, Debug)]
pub struct ProjectConditionDefinition {
    exact_ref: ExactDefinitionRef,
    definition_fingerprint: DefinitionFingerprint,
}
impl ProjectConditionDefinition {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }
    pub fn definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.definition_fingerprint
    }
    pub fn load(repo: impl AsRef<Path>, path: &str) -> Result<Self, RegistryLoadError> {
        let mut budget = SourceByteBudget::default();
        let (_, bytes) = read_trusted_repo_source(repo.as_ref(), path, &mut budget)?;
        let value = parse_definition_yaml(&bytes)?;
        let authored: AuthoredCondition = serde_json::from_value(value).map_err(|e| {
            RegistryLoadError::new(
                if e.to_string().contains("unknown field") {
                    RegistryLoadErrorKind::UnknownField
                } else {
                    RegistryLoadErrorKind::SyntaxError
                },
                "project condition does not match its closed record",
            )
        })?;
        authored.resolve()
    }
}
impl AuthoredCondition {
    fn resolve(self) -> Result<ProjectConditionDefinition, RegistryLoadError> {
        let literal = self.schema_id == "handbook.project-condition-definition"
            && self.schema_version == "1.0"
            && self.outcomes == ["true", "false", "unknown", "unresolved", "stale", "refused"]
            && self.accepted_input_classes == ["authoritative_fact_ref", "admitted_evidence_ref"]
            && self.freshness_requirement == "explicit_current_basis_required"
            && self.minimum_independent_current_bases == 1
            && self.self_reference_exclusions == ["environment_context"]
            && self.outcome_precedence
                == [
                    "refused_on_contradiction_or_disallowed_input",
                    "unresolved_on_missing_definition_or_required_input",
                    "stale_on_expired_basis",
                    "unknown_on_insufficient_proof",
                    "false_on_current_affirmative_no_responsibility",
                    "true_on_current_affirmative_qualifying_responsibility",
                ]
            && self.effects
                == [
                    "metadata_only",
                    "no_boolean_coercion",
                    "no_create",
                    "no_scaffold",
                    "no_delete",
                ]
            && self.extensions.is_empty();
        if !literal {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "project condition differs from the exact managed-operational-surface definition",
            ));
        }
        let exact_ref = ExactDefinitionRef::new(&self.condition_id, &self.condition_version)?;
        if exact_ref.as_str() != "handbook.condition.project.managed-operational-surface@1.0.0" {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "unsupported project condition identity",
            ));
        }
        let supplied = DefinitionFingerprint::parse(&self.definition_fingerprint)?;
        let computed = fingerprint_serializable(&self)?;
        if supplied != computed {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::FingerprintMismatch,
                "project condition fingerprint mismatch",
            ));
        }
        Ok(ProjectConditionDefinition {
            exact_ref,
            definition_fingerprint: computed,
        })
    }
}
