use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SelectorGrammar {
    grammar_id: String,
    encoding: String,
    min_bytes: u16,
    max_bytes: u16,
    min_segments: u8,
    max_segments: u8,
    separator: String,
    normal_segment_character_class: String,
    single_segment_wildcard: String,
    recursive_wildcard: String,
    recursive_position: String,
    disallowed: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Matcher {
    schema_id: String,
    schema_version: String,
    matcher_id: String,
    matcher_version: String,
    target_kinds: Vec<String>,
    selector_grammar: SelectorGrammar,
    case_mode: String,
    deny_precedence: bool,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    definition_fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Escalation {
    schema_id: String,
    schema_version: String,
    policy_id: String,
    policy_version: String,
    trigger_classes: Vec<String>,
    proposal_relation: String,
    required_request_bindings: Vec<String>,
    terminal_outcomes: Vec<String>,
    terminal_cardinality: String,
    preapproval_effect: String,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    policy_fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Promotion {
    schema_id: String,
    schema_version: String,
    policy_id: String,
    policy_version: String,
    source_requirement: String,
    target_authority: String,
    horizon_relation: String,
    write_precondition: String,
    required_request_bindings: Vec<String>,
    terminal_outcomes: Vec<String>,
    terminal_cardinality: String,
    forbidden_authorities: Vec<String>,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    policy_fingerprint: String,
}
#[derive(Clone, Debug, Default)]
pub struct ContextResolutionPolicyRegistry {
    policies: BTreeMap<ExactDefinitionRef, DefinitionFingerprint>,
}
impl ContextResolutionPolicyRegistry {
    pub fn load(repo: impl AsRef<Path>, paths: &[String]) -> Result<Self, RegistryLoadError> {
        let mut b = SourceByteBudget::default();
        let mut policies = BTreeMap::new();
        for path in paths {
            let (_, bytes) = read_trusted_repo_source(repo.as_ref(), path, &mut b)?;
            let value = parse_definition_yaml(&bytes)?;
            let schema = value.get("schema_id").and_then(Value::as_str).unwrap_or("");
            let (r, f) = match schema {
                "handbook.mutation-matcher-definition" => resolve_matcher(decode(value)?)?,
                "handbook.resolution-escalation-policy-definition" => {
                    resolve_escalation(decode(value)?)?
                }
                "handbook.memory-promotion-policy-definition" => resolve_promotion(decode(value)?)?,
                _ => {
                    return Err(RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "unsupported Context Resolution policy record",
                    ))
                }
            };
            if policies.insert(r, f).is_some() {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "Context Resolution policy identity is duplicated",
                ));
            }
        }
        Ok(Self { policies })
    }
    pub fn fingerprint(&self, r: &ExactDefinitionRef) -> Option<&DefinitionFingerprint> {
        self.policies.get(r)
    }
    pub fn refs(&self) -> BTreeSet<ExactDefinitionRef> {
        self.policies.keys().cloned().collect()
    }
}
fn decode<T: for<'de> Deserialize<'de>>(v: Value) -> Result<T, RegistryLoadError> {
    serde_json::from_value(v).map_err(|e| {
        RegistryLoadError::new(
            if e.to_string().contains("unknown field") {
                RegistryLoadErrorKind::UnknownField
            } else {
                RegistryLoadErrorKind::SyntaxError
            },
            "Context Resolution producer does not match its closed record",
        )
    })
}
fn finish<T: Serialize>(
    value: &T,
    supplied: &str,
    id: &str,
    version: &str,
    expected: Value,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let actual = serde_json::to_value(value).map_err(|_| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::SyntaxError,
            "producer serialization failed",
        )
    })?;
    if actual != expected {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "Context Resolution producer differs from the exact shipped definition",
        ));
    }
    let r = ExactDefinitionRef::new(id, version)?;
    let supplied = DefinitionFingerprint::parse(supplied)?;
    let computed = fingerprint_serializable(value)?;
    if supplied != computed {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::FingerprintMismatch,
            "Context Resolution producer fingerprint mismatch",
        ));
    }
    Ok((r, computed))
}
fn resolve_matcher(
    v: Matcher,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let expected = json!({"schema_id":"handbook.mutation-matcher-definition","schema_version":"1.0","matcher_id":"handbook.mutation-matcher.core","matcher_version":"1.0.0","target_kinds":["repository_path"],"selector_grammar":{"grammar_id":"normalized_repo_relative_glob_v1","encoding":"ascii","min_bytes":1,"max_bytes":1024,"min_segments":1,"max_segments":64,"separator":"/","normal_segment_character_class":"[A-Za-z0-9._*-]","single_segment_wildcard":"*","recursive_wildcard":"**","recursive_position":"terminal_segment_only","disallowed":["leading_slash","trailing_slash","empty_segment","dot_segment","dotdot_segment","backslash","nul","uri_prefix","adjacent_double_star_in_normal_segment"]},"case_mode":"sensitive","deny_precedence":true,"extensions":{}});
    finish(
        &v,
        &v.definition_fingerprint,
        &v.matcher_id,
        &v.matcher_version,
        expected,
    )
}
fn resolve_escalation(
    v: Escalation,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let expected = json!({"schema_id":"handbook.resolution-escalation-policy-definition","schema_version":"1.0","policy_id":"handbook.resolution-escalation.core","policy_version":"1.0.0","trigger_classes":["dimension_rank_increase","mutation_allow_expansion","missing_context","missing_authority"],"proposal_relation":"same_profile_stack_strict_widening","required_request_bindings":["current_envelope_ref_fingerprint","proposed_envelope_ref_fingerprint","trigger_ref_fingerprint","missing_condition","requested_authority_ref","evidence_refs"],"terminal_outcomes":["approved","refused","superseded"],"terminal_cardinality":"exactly_one","preapproval_effect":"request_only_no_authority_change","extensions":{}});
    finish(
        &v,
        &v.policy_fingerprint,
        &v.policy_id,
        &v.policy_version,
        expected,
    )
}
fn resolve_promotion(
    v: Promotion,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let expected = json!({"schema_id":"handbook.memory-promotion-policy-definition","schema_version":"1.0","policy_id":"handbook.memory-promotion.core","policy_version":"1.0.0","source_requirement":"nonempty_exact_ref_fingerprint_pairs","target_authority":"semantic_memory","horizon_relation":"strictly_higher_memory_rank","write_precondition":"expected_target_fingerprint_compare_and_write","required_request_bindings":["source_inputs","source_envelope_ref_fingerprint","target_memory_horizon","target_record_ref","expected_target_fingerprint","requested_authority_ref"],"terminal_outcomes":["applied","refused","stale"],"terminal_cardinality":"exactly_one","forbidden_authorities":["canonical_artifact","contract","posture"],"extensions":{}});
    finish(
        &v,
        &v.policy_fingerprint,
        &v.policy_id,
        &v.policy_version,
        expected,
    )
}
