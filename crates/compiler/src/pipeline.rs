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

impl PipelineDefinition {
    /// Returns the declared stage list in source order.
    ///
    /// Downstream routing code should consume this typed slice instead of
    /// reparsing the pipeline YAML or inferring order from the filesystem.
    pub fn declared_stages(&self) -> &[PipelineStage] {
        &self.body.stages
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalog {
    pipelines: std::collections::BTreeMap<String, PipelineCatalogEntry>,
    stages: std::collections::BTreeMap<String, StageCatalogEntry>,
}

impl PipelineCatalog {
    pub fn pipelines(&self) -> impl Iterator<Item = &PipelineCatalogEntry> {
        self.pipelines.values()
    }

    pub fn stages(&self) -> impl Iterator<Item = &StageCatalogEntry> {
        self.stages.values()
    }

    pub fn pipeline_count(&self) -> usize {
        self.pipelines.len()
    }

    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    pub fn resolve_selector(
        &self,
        selector: &str,
    ) -> Result<PipelineSelection, PipelineLookupError> {
        resolve_pipeline_selector(self, selector)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalogEntry {
    pub definition: PipelineDefinition,
    pub stages: Vec<PipelineCatalogStageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalogStageEntry {
    pub stage_id: String,
    pub title: String,
    pub work_level: Option<String>,
    pub source_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageCatalogEntry {
    pub id: String,
    pub kind: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub work_level: Option<String>,
    pub source_path: PathBuf,
    pub pipelines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineSelection {
    Pipeline(PipelineCatalogEntry),
    Stage(StageCatalogEntry),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineLookupError {
    UnsupportedSelector {
        selector: String,
        reason: &'static str,
    },
    AmbiguousSelector {
        selector: String,
        matches: Vec<String>,
    },
    UnknownSelector {
        selector: String,
    },
}

#[derive(Debug)]
pub enum PipelineCatalogError {
    ReadPipelineCatalog {
        path: PathBuf,
        source: std::io::Error,
    },
    ReadStageCatalog {
        path: PathBuf,
        source: std::io::Error,
    },
    PipelineLoad {
        path: PathBuf,
        source: PipelineLoadError,
    },
    StageFrontMatter {
        path: PathBuf,
        source: serde_yaml_bw::Error,
    },
    StageKindMismatch {
        path: PathBuf,
        actual: String,
    },
    StageIdMismatch {
        path: PathBuf,
        expected: String,
        actual: String,
    },
    DuplicatePipelineId {
        id: String,
    },
    DuplicateStageId {
        id: String,
    },
}

impl fmt::Display for PipelineCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineCatalogError::ReadPipelineCatalog { path, source } => {
                write!(
                    f,
                    "failed to read pipeline catalog entry {}: {source}",
                    path.display()
                )
            }
            PipelineCatalogError::ReadStageCatalog { path, source } => {
                write!(
                    f,
                    "failed to read stage catalog entry {}: {source}",
                    path.display()
                )
            }
            PipelineCatalogError::PipelineLoad { path, source } => {
                write!(
                    f,
                    "failed to load pipeline definition {}: {source}",
                    path.display()
                )
            }
            PipelineCatalogError::StageFrontMatter { path, source } => {
                write!(
                    f,
                    "failed to load stage front matter {}: {source}",
                    path.display()
                )
            }
            PipelineCatalogError::StageKindMismatch { path, actual } => {
                write!(
                    f,
                    "stage front matter {} must declare kind `stage`, got `{actual}`",
                    path.display()
                )
            }
            PipelineCatalogError::StageIdMismatch {
                path,
                expected,
                actual,
            } => write!(
                f,
                "stage front matter {} expected canonical id `{expected}` but found `{actual}`",
                path.display()
            ),
            PipelineCatalogError::DuplicatePipelineId { id } => {
                write!(f, "duplicate pipeline id `{id}` in pipeline catalog")
            }
            PipelineCatalogError::DuplicateStageId { id } => {
                write!(f, "duplicate stage id `{id}` in stage catalog")
            }
        }
    }
}

impl std::error::Error for PipelineCatalogError {}

impl fmt::Display for PipelineLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineLookupError::UnsupportedSelector { selector, reason } => {
                write!(f, "unsupported selector `{selector}`: {reason}")
            }
            PipelineLookupError::AmbiguousSelector { selector, matches } => {
                write!(
                    f,
                    "ambiguous selector `{selector}` matched multiple canonical ids: {}",
                    matches.join(", ")
                )
            }
            PipelineLookupError::UnknownSelector { selector } => {
                write!(
                    f,
                    "unknown pipeline selector `{selector}`; use a canonical id or `pipeline list` to inspect available inventory"
                )
            }
        }
    }
}

impl std::error::Error for PipelineLookupError {}

#[derive(Debug, Clone, Deserialize)]
struct StageFrontMatter {
    pub kind: String,
    pub id: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub work_level: Option<String>,
}

pub fn load_pipeline_catalog(
    repo_root: impl AsRef<Path>,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    let repo_root = repo_root.as_ref();
    let mut pipelines = std::collections::BTreeMap::new();
    let mut stages = std::collections::BTreeMap::new();
    let stage_catalog = load_stage_catalog(repo_root)?;
    let mut stage_memberships: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();

    for pipeline_path in discover_repo_relative_files(repo_root, Path::new("pipelines"), "yaml")? {
        let definition = load_pipeline_definition(repo_root, &pipeline_path).map_err(|source| {
            PipelineCatalogError::PipelineLoad {
                path: repo_root.join(&pipeline_path),
                source,
            }
        })?;
        let pipeline_id = definition.header.id.clone();

        let mut stage_entries = Vec::with_capacity(definition.declared_stages().len());
        for stage in definition.declared_stages() {
            let stage_catalog_entry =
                stage_catalog.get(Path::new(&stage.file)).ok_or_else(|| {
                    PipelineCatalogError::StageFrontMatter {
                        path: repo_root.join(&stage.file),
                        source: <serde_yaml_bw::Error as serde::de::Error>::custom(format!(
                            "stage file {} is missing front matter or is not cataloged",
                            stage.file
                        )),
                    }
                })?;

            if stage_catalog_entry.id != stage.id {
                return Err(PipelineCatalogError::StageIdMismatch {
                    path: stage_catalog_entry.source_path.clone(),
                    expected: stage.id.clone(),
                    actual: stage_catalog_entry.id.clone(),
                });
            }

            stage_entries.push(PipelineCatalogStageEntry {
                stage_id: stage_catalog_entry.id.clone(),
                title: stage_catalog_entry.title.clone(),
                work_level: stage_catalog_entry.work_level.clone(),
                source_path: stage_catalog_entry.source_path.clone(),
            });

            stage_memberships
                .entry(stage_catalog_entry.id.clone())
                .or_default()
                .push(definition.header.id.clone());
        }

        if pipelines
            .insert(
                pipeline_id.clone(),
                PipelineCatalogEntry {
                    definition,
                    stages: stage_entries,
                },
            )
            .is_some()
        {
            return Err(PipelineCatalogError::DuplicatePipelineId { id: pipeline_id });
        }
    }

    for (stage_path, mut entry) in stage_catalog {
        entry.pipelines = stage_memberships.remove(&entry.id).unwrap_or_default();
        let stage_id = entry.id.clone();
        if stages.insert(stage_id.clone(), entry).is_some() {
            return Err(PipelineCatalogError::DuplicateStageId { id: stage_id });
        }
        let _ = stage_path;
    }

    Ok(PipelineCatalog { pipelines, stages })
}

pub fn render_pipeline_list(catalog: &PipelineCatalog) -> String {
    let mut out = String::new();
    out.push_str("PIPELINE INVENTORY\n");
    out.push_str(&format!("PIPELINE COUNT: {}\n", catalog.pipeline_count()));
    out.push_str(&format!("STAGE COUNT: {}\n", catalog.stage_count()));

    for pipeline in catalog.pipelines() {
        out.push_str(&format!("\nPIPELINE: {}\n", pipeline.definition.header.id));
        out.push_str(&format!("TITLE: {}\n", pipeline.definition.header.title));
        out.push_str(&format!(
            "SOURCE: {}\n",
            pipeline.definition.source_path.display()
        ));
        out.push_str(&format!("STAGES: {}\n", pipeline.stages.len()));
    }

    out.trim_end().to_string()
}

pub fn render_pipeline_show(selection: &PipelineSelection) -> String {
    match selection {
        PipelineSelection::Pipeline(pipeline) => render_pipeline_definition(pipeline),
        PipelineSelection::Stage(stage) => render_stage_definition(stage),
    }
}

pub fn resolve_pipeline_selector(
    catalog: &PipelineCatalog,
    selector: &str,
) -> Result<PipelineSelection, PipelineLookupError> {
    let selector = selector.trim();
    if selector.is_empty() {
        return Err(PipelineLookupError::UnknownSelector {
            selector: selector.to_string(),
        });
    }

    if looks_like_repo_path(selector) {
        return Err(PipelineLookupError::UnsupportedSelector {
            selector: selector.to_string(),
            reason: "raw file paths are evidence only; use a canonical pipeline or stage id",
        });
    }

    if let Some(pipeline) = catalog.pipelines.get(selector) {
        return Ok(PipelineSelection::Pipeline(pipeline.clone()));
    }
    if let Some(stage) = catalog.stages.get(selector) {
        return Ok(PipelineSelection::Stage(stage.clone()));
    }

    let shorthand_matches = catalog
        .pipelines
        .values()
        .filter(|pipeline| {
            pipeline.definition.header.id.strip_prefix("pipeline.") == Some(selector)
        })
        .map(|pipeline| pipeline.definition.header.id.clone())
        .chain(
            catalog
                .stages
                .values()
                .filter(|stage| stage.id.strip_prefix("stage.") == Some(selector))
                .map(|stage| stage.id.clone()),
        )
        .collect::<Vec<_>>();

    if shorthand_matches.len() > 1 {
        return Err(PipelineLookupError::AmbiguousSelector {
            selector: selector.to_string(),
            matches: shorthand_matches,
        });
    }

    if let Some(canonical_id) = shorthand_matches.into_iter().next() {
        if let Some(pipeline) = catalog.pipelines.get(&canonical_id) {
            return Ok(PipelineSelection::Pipeline(pipeline.clone()));
        }
        if let Some(stage) = catalog.stages.get(&canonical_id) {
            return Ok(PipelineSelection::Stage(stage.clone()));
        }
    }

    Err(PipelineLookupError::UnknownSelector {
        selector: selector.to_string(),
    })
}

fn render_pipeline_definition(pipeline: &PipelineCatalogEntry) -> String {
    let mut out = String::new();
    out.push_str(&format!("PIPELINE: {}\n", pipeline.definition.header.id));
    out.push_str(&format!("TITLE: {}\n", pipeline.definition.header.title));
    out.push_str(&format!(
        "DESCRIPTION: {}\n",
        pipeline.definition.header.description.trim()
    ));
    out.push_str(&format!(
        "SOURCE: {}\n",
        pipeline.definition.source_path.display()
    ));
    out.push_str("DEFAULTS:\n");
    out.push_str(&format!(
        "  runner: {}\n",
        pipeline.definition.body.defaults.runner
    ));
    out.push_str(&format!(
        "  profile: {}\n",
        pipeline.definition.body.defaults.profile
    ));
    out.push_str(&format!(
        "  enable_complexity: {}\n",
        pipeline.definition.body.defaults.enable_complexity
    ));
    out.push_str("STAGES:\n");
    for (index, stage) in pipeline.stages.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {}",
            index + 1,
            stage.stage_id,
            stage.source_path.display()
        ));
        out.push_str(&format!(" | {}\n", stage.title));
        if let Some(work_level) = &stage.work_level {
            out.push_str(&format!("     work_level: {}\n", work_level));
        }
    }

    out.trim_end().to_string()
}

fn render_stage_definition(stage: &StageCatalogEntry) -> String {
    let mut out = String::new();
    out.push_str(&format!("STAGE: {}\n", stage.id));
    out.push_str(&format!("KIND: {}\n", stage.kind));
    out.push_str(&format!("VERSION: {}\n", stage.version));
    out.push_str(&format!("TITLE: {}\n", stage.title));
    out.push_str(&format!("DESCRIPTION: {}\n", stage.description.trim()));
    if let Some(work_level) = &stage.work_level {
        out.push_str(&format!("WORK_LEVEL: {}\n", work_level));
    }
    out.push_str(&format!("SOURCE: {}\n", stage.source_path.display()));
    out.push_str("PIPELINES:\n");
    for pipeline in &stage.pipelines {
        out.push_str(&format!("  - {}\n", pipeline));
    }

    out.trim_end().to_string()
}

fn load_stage_catalog(
    repo_root: &Path,
) -> Result<std::collections::BTreeMap<PathBuf, StageCatalogEntry>, PipelineCatalogError> {
    let mut out = std::collections::BTreeMap::new();
    for stage_path in discover_repo_relative_files(repo_root, Path::new("core/stages"), "md")? {
        let full_path = repo_root.join(&stage_path);
        let contents = std::fs::read_to_string(&full_path).map_err(|source| {
            PipelineCatalogError::ReadStageCatalog {
                path: full_path.clone(),
                source,
            }
        })?;

        let Some(front_matter_text) = extract_front_matter_block(&contents) else {
            continue;
        };

        let front_matter = serde_yaml_bw::from_str::<StageFrontMatter>(&front_matter_text)
            .map_err(|source| PipelineCatalogError::StageFrontMatter {
                path: full_path.clone(),
                source,
            })?;

        if front_matter.kind != "stage" {
            return Err(PipelineCatalogError::StageKindMismatch {
                path: full_path.clone(),
                actual: front_matter.kind,
            });
        }

        out.insert(
            stage_path.clone(),
            StageCatalogEntry {
                id: front_matter.id,
                kind: front_matter.kind,
                version: front_matter.version,
                title: front_matter.title,
                description: front_matter.description,
                work_level: front_matter.work_level,
                source_path: stage_path.clone(),
                pipelines: Vec::new(),
            },
        );
    }

    Ok(out)
}

fn extract_front_matter_block(contents: &str) -> Option<String> {
    let mut lines = contents.lines();
    if lines.next()? != "---" {
        return None;
    }

    let mut front_matter = String::new();
    for line in lines {
        if line == "---" {
            return Some(front_matter);
        }
        front_matter.push_str(line);
        front_matter.push('\n');
    }

    None
}

fn discover_repo_relative_files(
    repo_root: &Path,
    relative_dir: &Path,
    extension: &str,
) -> Result<Vec<PathBuf>, PipelineCatalogError> {
    let full_dir = repo_root.join(relative_dir);
    let entries = std::fs::read_dir(&full_dir).map_err(|source| {
        if relative_dir == Path::new("pipelines") {
            PipelineCatalogError::ReadPipelineCatalog {
                path: full_dir.clone(),
                source,
            }
        } else {
            PipelineCatalogError::ReadStageCatalog {
                path: full_dir.clone(),
                source,
            }
        }
    })?;

    let mut out = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| {
            if relative_dir == Path::new("pipelines") {
                PipelineCatalogError::ReadPipelineCatalog {
                    path: full_dir.clone(),
                    source,
                }
            } else {
                PipelineCatalogError::ReadStageCatalog {
                    path: full_dir.clone(),
                    source,
                }
            }
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some(extension) {
            continue;
        }

        let relative = path.strip_prefix(repo_root).unwrap_or(&path).to_path_buf();
        out.push(relative);
    }

    out.sort();
    Ok(out)
}

fn looks_like_repo_path(selector: &str) -> bool {
    selector.contains('/')
        || selector.contains('\\')
        || selector.ends_with(".yaml")
        || selector.ends_with(".yml")
        || selector.ends_with(".md")
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
    pub value: bool,
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
    InvalidStageFile {
        stage_id: String,
        file: String,
        reason: StageFileValidationError,
    },
    InvalidActivation {
        stage_id: String,
        reason: ActivationValidationError,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageFileValidationError {
    OutsideStageDirectory,
    WrongExtension,
    Missing,
    NotRegularFile,
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
        source_path: relative_pipeline_path.to_path_buf(),
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
            PipelineValidationError::InvalidStageFile {
                stage_id,
                file,
                reason,
            } => write!(f, "stage `{stage_id}` file `{file}` is invalid: {reason}"),
            PipelineValidationError::InvalidActivation { stage_id, reason } => {
                write!(f, "stage `{stage_id}` has invalid activation: {reason}")
            }
        }
    }
}

impl fmt::Display for StageFileValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StageFileValidationError::OutsideStageDirectory => {
                write!(f, "must live under `core/stages/`")
            }
            StageFileValidationError::WrongExtension => {
                write!(f, "must use the `.md` extension")
            }
            StageFileValidationError::Missing => {
                write!(f, "must reference an existing file")
            }
            StageFileValidationError::NotRegularFile => {
                write!(f, "must reference an existing regular file")
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

    if !file_path.starts_with(Path::new("core/stages")) {
        return Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::OutsideStageDirectory,
        ));
    }

    if file_path.extension().and_then(|ext| ext.to_str()) != Some("md") {
        return Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::WrongExtension,
        ));
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

    let meta = match std::fs::symlink_metadata(&resolved) {
        Ok(meta) => meta,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(invalid_stage_file_error(
                path,
                stage,
                StageFileValidationError::Missing,
            ));
        }
        Err(_) => {
            return Err(invalid_stage_file_error(
                path,
                stage,
                StageFileValidationError::NotRegularFile,
            ));
        }
    };

    if !meta.is_file() {
        return Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::NotRegularFile,
        ));
    }

    Ok(())
}

fn invalid_stage_file_error(
    path: &Path,
    stage: &PipelineStage,
    reason: StageFileValidationError,
) -> PipelineLoadError {
    PipelineLoadError::Validation {
        path: path.to_path_buf(),
        error: PipelineValidationError::InvalidStageFile {
            stage_id: stage.id.clone(),
            file: stage.file.clone(),
            reason,
        },
    }
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
    let Some((left, right)) = clause.split_once("==") else {
        return Err(format!(
            "unsupported clause `{clause}`: exactly one equality operator is required"
        ));
    };
    let left = left.trim();
    let right = right.trim();

    if right.contains("==") {
        return Err(format!(
            "unsupported clause `{clause}`: only one equality operator is supported"
        ));
    }

    if !is_supported_variable_path(left) {
        return Err(format!(
            "unsupported clause `{clause}`: left-hand side must match `variables.<name>`"
        ));
    }

    let value = parse_activation_bool(right).ok_or_else(|| {
        format!(
            "unsupported clause `{clause}`: reduced v1 supports only boolean activation values (`true` or `false`)"
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

fn parse_activation_bool(input: &str) -> Option<bool> {
    match input {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
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
