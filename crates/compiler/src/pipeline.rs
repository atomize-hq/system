use serde::Deserialize;
use std::collections::BTreeSet;
use std::fmt;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineDefinition {
    pub source_path: PathBuf,
    pub header: PipelineHeader,
    pub body: PipelineBody,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHeader {
    pub kind: String,
    pub id: String,
    pub version: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineBody {
    pub defaults: PipelineDefaults,
    pub stages: Vec<PipelineStage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineDefaults {
    pub runner: String,
    pub profile: String,
    pub enable_complexity: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineStage {
    pub id: String,
    pub file: String,
    pub sets: Option<Vec<String>>,
    pub activation: Option<StageActivation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageActivation {
    pub when: ActivationConditionSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivationConditionSet {
    pub clauses: Vec<ActivationClause>,
    pub operator: ActivationOperator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationOperator {
    Any,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivationClause {
    pub variable: String,
    pub value: ActivationScalar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivationScalar {
    Bool(bool),
    String(String),
    Number(String),
}

#[derive(Debug)]
pub enum PipelineLoadError {
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    UnsupportedPipelinePath {
        path: PathBuf,
        reason: &'static str,
    },
    WrongDocumentCount {
        path: PathBuf,
        actual: usize,
    },
    HeaderParse {
        path: PathBuf,
        source: serde_yaml_bw::Error,
    },
    BodyParse {
        path: PathBuf,
        source: serde_yaml_bw::Error,
    },
    Validation {
        path: PathBuf,
        error: PipelineValidationError,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineValidationError {
    EmptyField {
        field: &'static str,
    },
    UnsupportedKind {
        actual: String,
    },
    EmptyStages,
    DuplicateStageId {
        stage_id: String,
    },
    EmptySetsList {
        stage_id: String,
    },
    EmptySetVariable {
        stage_id: String,
        index: usize,
    },
    StageFileOutsideRepoRoot {
        stage_id: String,
        file: String,
    },
    InvalidActivation {
        stage_id: String,
        reason: ActivationValidationError,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivationValidationError {
    EmptyConditionList {
        operator: ActivationOperator,
    },
    UnsupportedClause {
        clause: String,
        detail: &'static str,
    },
}

pub fn load_pipeline_definition(
    repo_root: impl AsRef<Path>,
    pipeline_path: impl AsRef<Path>,
) -> Result<PipelineDefinition, PipelineLoadError> {
    let repo_root = repo_root.as_ref();
    let pipeline_path = pipeline_path.as_ref();
    let relative_pipeline_path = validate_repo_relative_path(pipeline_path).map_err(|reason| {
        PipelineLoadError::UnsupportedPipelinePath {
            path: pipeline_path.to_path_buf(),
            reason,
        }
    })?;
    let full_path = repo_root.join(relative_pipeline_path);
    let contents =
        std::fs::read_to_string(&full_path).map_err(|source| PipelineLoadError::ReadFailure {
            path: full_path.clone(),
            source,
        })?;

    let mut docs = serde_yaml_bw::Deserializer::from_str(&contents);
    let header_doc = docs.next();
    let body_doc = docs.next();

    if header_doc.is_none() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 0,
        });
    }
    if body_doc.is_none() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 1,
        });
    }

    let header =
        PipelineHeader::deserialize(header_doc.expect("header doc present")).map_err(|source| {
            PipelineLoadError::HeaderParse {
                path: full_path.clone(),
                source,
            }
        })?;
    let body =
        PipelineBody::deserialize(body_doc.expect("body doc present")).map_err(|source| {
            PipelineLoadError::BodyParse {
                path: full_path.clone(),
                source,
            }
        })?;

    if docs.next().is_some() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 3,
        });
    }

    validate_pipeline_definition(repo_root, &full_path, &header, &body)?;

    Ok(PipelineDefinition {
        source_path: full_path,
        header,
        body,
    })
}

impl<'de> Deserialize<'de> for ActivationConditionSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ActivationWhenRaw {
            any: Option<Vec<String>>,
            all: Option<Vec<String>>,
        }

        let raw = ActivationWhenRaw::deserialize(deserializer)?;
        match (raw.any, raw.all) {
            (Some(clauses), None) => Ok(Self {
                clauses: clauses
                    .into_iter()
                    .map(parse_activation_clause)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(serde::de::Error::custom)?,
                operator: ActivationOperator::Any,
            }),
            (None, Some(clauses)) => Ok(Self {
                clauses: clauses
                    .into_iter()
                    .map(parse_activation_clause)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(serde::de::Error::custom)?,
                operator: ActivationOperator::All,
            }),
            (Some(_), Some(_)) => Err(serde::de::Error::custom(
                "activation.when must contain exactly one of `any` or `all`",
            )),
            (None, None) => Err(serde::de::Error::custom(
                "activation.when must contain exactly one of `any` or `all`",
            )),
        }
    }
}

impl fmt::Display for PipelineLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineLoadError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read pipeline YAML at {}: {source}",
                    path.display()
                )
            }
            PipelineLoadError::UnsupportedPipelinePath { path, reason } => {
                write!(f, "unsupported pipeline path {}: {reason}", path.display())
            }
            PipelineLoadError::WrongDocumentCount { path, actual } => write!(
                f,
                "pipeline YAML at {} must contain exactly 2 documents; found {}",
                path.display(),
                actual
            ),
            PipelineLoadError::HeaderParse { path, source } => {
                write!(
                    f,
                    "failed to parse pipeline header in {}: {source}",
                    path.display()
                )
            }
            PipelineLoadError::BodyParse { path, source } => {
                write!(
                    f,
                    "failed to parse pipeline body in {}: {source}",
                    path.display()
                )
            }
            PipelineLoadError::Validation { path, error } => {
                write!(
                    f,
                    "invalid pipeline definition at {}: {error}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for PipelineLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PipelineLoadError::ReadFailure { source, .. } => Some(source),
            PipelineLoadError::HeaderParse { source, .. } => Some(source),
            PipelineLoadError::BodyParse { source, .. } => Some(source),
            PipelineLoadError::UnsupportedPipelinePath { .. }
            | PipelineLoadError::WrongDocumentCount { .. }
            | PipelineLoadError::Validation { .. } => None,
        }
    }
}

impl fmt::Display for PipelineValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineValidationError::EmptyField { field } => {
                write!(f, "field `{field}` must not be empty")
            }
            PipelineValidationError::UnsupportedKind { actual } => {
                write!(f, "field `kind` must be `pipeline`, got `{actual}`")
            }
            PipelineValidationError::EmptyStages => {
                write!(f, "pipeline must declare at least one stage")
            }
            PipelineValidationError::DuplicateStageId { stage_id } => {
                write!(f, "duplicate stage id `{stage_id}`")
            }
            PipelineValidationError::EmptySetsList { stage_id } => {
                write!(
                    f,
                    "stage `{stage_id}` must not declare an empty `sets` list"
                )
            }
            PipelineValidationError::EmptySetVariable { stage_id, index } => write!(
                f,
                "stage `{stage_id}` has an empty `sets` entry at index {index}"
            ),
            PipelineValidationError::StageFileOutsideRepoRoot { stage_id, file } => write!(
                f,
                "stage `{stage_id}` file `{file}` resolves outside the repo root"
            ),
            PipelineValidationError::InvalidActivation { stage_id, reason } => {
                write!(f, "stage `{stage_id}` has invalid activation: {reason}")
            }
        }
    }
}

impl fmt::Display for ActivationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActivationValidationError::EmptyConditionList { operator } => write!(
                f,
                "`activation.when.{}` must contain at least one clause",
                operator.label()
            ),
            ActivationValidationError::UnsupportedClause { clause, detail } => {
                write!(f, "unsupported clause `{clause}`: {detail}")
            }
        }
    }
}

fn validate_pipeline_definition(
    repo_root: &Path,
    path: &Path,
    header: &PipelineHeader,
    body: &PipelineBody,
) -> Result<(), PipelineLoadError> {
    validate_non_empty(path, "kind", &header.kind)?;
    if header.kind != "pipeline" {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::UnsupportedKind {
                actual: header.kind.clone(),
            },
        });
    }
    validate_non_empty(path, "id", &header.id)?;
    validate_non_empty(path, "version", &header.version)?;
    validate_non_empty(path, "title", &header.title)?;
    validate_non_empty(path, "description", &header.description)?;
    validate_non_empty(path, "defaults.runner", &body.defaults.runner)?;
    validate_non_empty(path, "defaults.profile", &body.defaults.profile)?;

    if body.stages.is_empty() {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::EmptyStages,
        });
    }

    let mut stage_ids = BTreeSet::new();
    for stage in &body.stages {
        validate_non_empty(path, "stage.id", &stage.id)?;
        validate_non_empty(path, "stage.file", &stage.file)?;

        if !stage_ids.insert(stage.id.clone()) {
            return Err(PipelineLoadError::Validation {
                path: path.to_path_buf(),
                error: PipelineValidationError::DuplicateStageId {
                    stage_id: stage.id.clone(),
                },
            });
        }

        if let Some(sets) = &stage.sets {
            if sets.is_empty() {
                return Err(PipelineLoadError::Validation {
                    path: path.to_path_buf(),
                    error: PipelineValidationError::EmptySetsList {
                        stage_id: stage.id.clone(),
                    },
                });
            }

            for (index, entry) in sets.iter().enumerate() {
                if entry.trim().is_empty() {
                    return Err(PipelineLoadError::Validation {
                        path: path.to_path_buf(),
                        error: PipelineValidationError::EmptySetVariable {
                            stage_id: stage.id.clone(),
                            index,
                        },
                    });
                }
            }
        }

        validate_stage_file(repo_root, path, stage)?;
        validate_stage_activation(path, stage)?;
    }

    Ok(())
}

fn validate_non_empty(
    path: &Path,
    field: &'static str,
    value: &str,
) -> Result<(), PipelineLoadError> {
    if value.trim().is_empty() {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::EmptyField { field },
        });
    }

    Ok(())
}

fn validate_stage_file(
    repo_root: &Path,
    path: &Path,
    stage: &PipelineStage,
) -> Result<(), PipelineLoadError> {
    let file_path = Path::new(&stage.file);
    if validate_repo_relative_path(file_path).is_err() {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::StageFileOutsideRepoRoot {
                stage_id: stage.id.clone(),
                file: stage.file.clone(),
            },
        });
    }

    let resolved = repo_root.join(file_path);
    if !resolved.starts_with(repo_root) {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::StageFileOutsideRepoRoot {
                stage_id: stage.id.clone(),
                file: stage.file.clone(),
            },
        });
    }

    Ok(())
}

fn validate_stage_activation(path: &Path, stage: &PipelineStage) -> Result<(), PipelineLoadError> {
    let Some(activation) = &stage.activation else {
        return Ok(());
    };

    if activation.when.clauses.is_empty() {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::InvalidActivation {
                stage_id: stage.id.clone(),
                reason: ActivationValidationError::EmptyConditionList {
                    operator: activation.when.operator,
                },
            },
        });
    }

    Ok(())
}

fn validate_repo_relative_path(path: &Path) -> Result<&Path, &'static str> {
    if path.as_os_str().is_empty() {
        return Err("path must not be empty");
    }

    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            Component::ParentDir => return Err("path must not escape the repo root"),
            Component::RootDir | Component::Prefix(_) => return Err("path must be repo-relative"),
        }
    }

    Ok(path)
}

fn parse_activation_clause(input: String) -> Result<ActivationClause, String> {
    let clause = input.trim();
    let mut parts = clause.split("==");
    let left = parts
        .next()
        .ok_or_else(|| "missing left-hand side".to_string())?
        .trim();
    let right = parts
        .next()
        .ok_or_else(|| "missing right-hand side".to_string())?
        .trim();

    if parts.next().is_some() {
        return Err(format!(
            "unsupported clause `{clause}`: only one equality operator is supported"
        ));
    }

    if !is_supported_variable_path(left) {
        return Err(format!(
            "unsupported clause `{clause}`: left-hand side must match `variables.<name>`"
        ));
    }

    let value = parse_activation_scalar(right).ok_or_else(|| {
        format!(
            "unsupported clause `{clause}`: right-hand side must be a boolean, quoted string, or number"
        )
    })?;

    Ok(ActivationClause {
        variable: left.trim_start_matches("variables.").to_string(),
        value,
    })
}

fn is_supported_variable_path(input: &str) -> bool {
    let Some(rest) = input.strip_prefix("variables.") else {
        return false;
    };

    let mut chars = rest.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }

    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn parse_activation_scalar(input: &str) -> Option<ActivationScalar> {
    match input {
        "true" => Some(ActivationScalar::Bool(true)),
        "false" => Some(ActivationScalar::Bool(false)),
        _ if is_quoted_string(input) => Some(ActivationScalar::String(
            input[1..input.len() - 1].to_string(),
        )),
        _ if is_supported_number(input) => Some(ActivationScalar::Number(input.to_string())),
        _ => None,
    }
}

fn is_quoted_string(input: &str) -> bool {
    let bytes = input.as_bytes();
    if bytes.len() < 2 {
        return false;
    }

    (bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
        || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\'')
}

fn is_supported_number(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    let stripped = if let Some(rest) = input.strip_prefix(['+', '-']) {
        rest
    } else {
        input
    };

    let mut parts = stripped.split('.');
    let int_part = parts.next().unwrap_or_default();
    let frac_part = parts.next();

    if parts.next().is_some()
        || int_part.is_empty()
        || !int_part.chars().all(|ch| ch.is_ascii_digit())
    {
        return false;
    }

    match frac_part {
        Some(part) => !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()),
        None => true,
    }
}

impl ActivationOperator {
    fn label(self) -> &'static str {
        match self {
            ActivationOperator::Any => "any",
            ActivationOperator::All => "all",
        }
    }
}
