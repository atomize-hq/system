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
struct Selection {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredVocabulary {
    schema_id: String,
    schema_version: String,
    vocabulary_id: String,
    vocabulary_version: String,
    stable_role_registry: Selection,
    labels: BTreeMap<String, Value>,
    aliases: BTreeMap<String, Value>,
    absorptions: Vec<Value>,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    vocabulary_fingerprint: String,
}
#[derive(Clone, Debug)]
pub struct VocabularyDefinition {
    exact_ref: ExactDefinitionRef,
    vocabulary_fingerprint: DefinitionFingerprint,
}
impl VocabularyDefinition {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }
    pub fn vocabulary_fingerprint(&self) -> &DefinitionFingerprint {
        &self.vocabulary_fingerprint
    }
    pub fn load(repo: impl AsRef<Path>, path: &str) -> Result<Self, RegistryLoadError> {
        let mut b = SourceByteBudget::default();
        let (_, bytes) = read_trusted_repo_source(repo.as_ref(), path, &mut b)?;
        let value = parse_definition_yaml(&bytes)?;
        let a: AuthoredVocabulary = serde_json::from_value(value).map_err(|e| {
            RegistryLoadError::new(
                if e.to_string().contains("unknown field") {
                    RegistryLoadErrorKind::UnknownField
                } else {
                    RegistryLoadErrorKind::SyntaxError
                },
                "vocabulary does not match its closed record",
            )
        })?;
        a.resolve()
    }
}
impl AuthoredVocabulary {
    fn resolve(self) -> Result<VocabularyDefinition, RegistryLoadError> {
        let literal = self.schema_id == "handbook.vocabulary-profile"
            && self.schema_version == "1.0"
            && self.stable_role_registry.reference == "handbook.roles.core@1.1.0"
            && self.stable_role_registry.fingerprint
                == "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029"
            && self.labels.is_empty()
            && self.aliases.is_empty()
            && self.absorptions.is_empty()
            && self.extensions.is_empty();
        if !literal {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "vocabulary differs from the exact shipped root vocabulary",
            ));
        }
        let exact_ref = ExactDefinitionRef::new(&self.vocabulary_id, &self.vocabulary_version)?;
        if exact_ref.as_str() != "handbook.vocabulary.shipped-root@1.0.0" {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "unsupported vocabulary identity",
            ));
        }
        let supplied = DefinitionFingerprint::parse(&self.vocabulary_fingerprint)?;
        let computed = fingerprint_serializable(&self)?;
        if supplied != computed {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::FingerprintMismatch,
                "vocabulary fingerprint mismatch",
            ));
        }
        Ok(VocabularyDefinition {
            exact_ref,
            vocabulary_fingerprint: computed,
        })
    }
}
