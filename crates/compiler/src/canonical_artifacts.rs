use sha2::{Digest, Sha256};
#[cfg(unix)]
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalArtifactKind {
    Charter,
    ProjectContext,
    FeatureSpec,
}

impl CanonicalArtifactKind {
    pub(crate) fn required(self) -> bool {
        match self {
            CanonicalArtifactKind::Charter => true,
            CanonicalArtifactKind::ProjectContext => false,
            CanonicalArtifactKind::FeatureSpec => false,
        }
    }

    pub(crate) fn relative_path(self) -> &'static str {
        match self {
            CanonicalArtifactKind::Charter => ".system/charter/CHARTER.md",
            CanonicalArtifactKind::ProjectContext => ".system/project_context/PROJECT_CONTEXT.md",
            CanonicalArtifactKind::FeatureSpec => ".system/feature_spec/FEATURE_SPEC.md",
        }
    }
}

const CANONICAL_ARTIFACT_KINDS: [CanonicalArtifactKind; 3] = [
    CanonicalArtifactKind::Charter,
    CanonicalArtifactKind::ProjectContext,
    CanonicalArtifactKind::FeatureSpec,
];

const CHARTER_TEMPLATE: &str = "\
# Charter
\n\
Describe the durable operating rules for this system.\n\
\n\
## Purpose\n\
\n\
- TODO\n\
\n\
## Constraints\n\
\n\
- TODO\n\
\n\
## Review Cadence\n\
\n\
- TODO\n";

const FEATURE_SPEC_TEMPLATE: &str = "\
# Feature Spec
\n\
Describe the product behavior that trusted project truth should produce.\n\
\n\
## Problem\n\
\n\
- TODO\n\
\n\
## Outcomes\n\
\n\
- TODO\n\
\n\
## Scope\n\
\n\
- TODO\n";

const PROJECT_CONTEXT_TEMPLATE: &str = "\
# Project Context
\n\
Optional: capture surrounding architecture, constraints, and local context that help planning.\n\
\n\
## Current State\n\
\n\
- TODO\n\
\n\
## Constraints\n\
\n\
- TODO\n\
\n\
## Open Questions\n\
\n\
- TODO\n";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalArtifactDescriptor {
    pub kind: CanonicalArtifactKind,
    pub relative_path: &'static str,
    pub namespace_dir: &'static str,
    pub required: bool,
    pub setup_starter_template: &'static str,
}

const CANONICAL_ARTIFACT_DESCRIPTORS: [CanonicalArtifactDescriptor; 3] = [
    CanonicalArtifactDescriptor {
        kind: CanonicalArtifactKind::Charter,
        relative_path: ".system/charter/CHARTER.md",
        namespace_dir: ".system/charter",
        required: true,
        setup_starter_template: CHARTER_TEMPLATE,
    },
    CanonicalArtifactDescriptor {
        kind: CanonicalArtifactKind::ProjectContext,
        relative_path: ".system/project_context/PROJECT_CONTEXT.md",
        namespace_dir: ".system/project_context",
        required: false,
        setup_starter_template: PROJECT_CONTEXT_TEMPLATE,
    },
    CanonicalArtifactDescriptor {
        kind: CanonicalArtifactKind::FeatureSpec,
        relative_path: ".system/feature_spec/FEATURE_SPEC.md",
        namespace_dir: ".system/feature_spec",
        required: false,
        setup_starter_template: FEATURE_SPEC_TEMPLATE,
    },
];

pub fn canonical_artifact_descriptors() -> &'static [CanonicalArtifactDescriptor; 3] {
    &CANONICAL_ARTIFACT_DESCRIPTORS
}

pub fn setup_starter_template(kind: CanonicalArtifactKind) -> &'static str {
    descriptor_for(kind).setup_starter_template
}

pub fn setup_starter_template_bytes(kind: CanonicalArtifactKind) -> &'static [u8] {
    setup_starter_template(kind).as_bytes()
}

pub fn matches_setup_starter_template(kind: CanonicalArtifactKind, bytes: &[u8]) -> bool {
    bytes == setup_starter_template_bytes(kind)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemRootStatus {
    Ok,
    Missing,
    NotDir,
    SymlinkNotAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactPresence {
    Missing,
    PresentEmpty,
    PresentNonEmpty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalArtifactIdentity {
    pub kind: CanonicalArtifactKind,
    pub relative_path: &'static str,
    pub required: bool,
    pub presence: ArtifactPresence,
    pub byte_len: Option<u64>,
    pub content_sha256: Option<String>,
    pub matches_setup_starter_template: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalArtifact {
    pub identity: CanonicalArtifactIdentity,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalArtifacts {
    pub system_root_status: SystemRootStatus,
    pub charter: CanonicalArtifact,
    pub project_context: CanonicalArtifact,
    pub feature_spec: CanonicalArtifact,
    pub ingest_issues: Vec<ArtifactIngestIssue>,
}

impl CanonicalArtifacts {
    pub fn load(repo_root: impl AsRef<Path>) -> Result<Self, ArtifactIngestError> {
        let repo_root = repo_root.as_ref();
        let system_root = repo_root.join(".system");

        let system_root_status = match std::fs::symlink_metadata(&system_root) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    SystemRootStatus::SymlinkNotAllowed
                } else if !meta.is_dir() {
                    SystemRootStatus::NotDir
                } else if canonical_root_scaffold_exists(repo_root)? {
                    SystemRootStatus::Ok
                } else {
                    SystemRootStatus::Missing
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => SystemRootStatus::Missing,
            Err(err) => {
                return Err(ArtifactIngestError::ReadFailure {
                    path: system_root,
                    source: err,
                });
            }
        };

        let mut ingest_issues = Vec::new();

        let (charter, project_context, feature_spec) = match system_root_status {
            SystemRootStatus::Ok => (
                load_one(
                    repo_root,
                    CanonicalArtifactKind::Charter,
                    &mut ingest_issues,
                ),
                load_one(
                    repo_root,
                    CanonicalArtifactKind::ProjectContext,
                    &mut ingest_issues,
                ),
                load_one(
                    repo_root,
                    CanonicalArtifactKind::FeatureSpec,
                    &mut ingest_issues,
                ),
            ),
            SystemRootStatus::Missing
            | SystemRootStatus::NotDir
            | SystemRootStatus::SymlinkNotAllowed => (
                missing_one(CanonicalArtifactKind::Charter),
                missing_one(CanonicalArtifactKind::ProjectContext),
                missing_one(CanonicalArtifactKind::FeatureSpec),
            ),
        };

        Ok(Self {
            system_root_status,
            charter,
            project_context,
            feature_spec,
            ingest_issues,
        })
    }

    pub fn identities(&self) -> [&CanonicalArtifactIdentity; 3] {
        [
            &self.charter.identity,
            &self.project_context.identity,
            &self.feature_spec.identity,
        ]
    }
}

fn canonical_root_scaffold_exists(repo_root: &Path) -> Result<bool, ArtifactIngestError> {
    for kind in CANONICAL_ARTIFACT_KINDS {
        let artifact_path = repo_root.join(kind.relative_path());
        match std::fs::symlink_metadata(&artifact_path) {
            Ok(_) => return Ok(true),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => {
                return Err(ArtifactIngestError::ReadFailure {
                    path: artifact_path,
                    source: err,
                });
            }
        }

        let namespace_dir = repo_root.join(canonical_namespace_dir(kind));
        match std::fs::symlink_metadata(&namespace_dir) {
            Ok(meta) if meta.is_dir() => return Ok(true),
            Ok(_) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => {
                return Err(ArtifactIngestError::ReadFailure {
                    path: namespace_dir,
                    source: err,
                });
            }
        }
    }

    Ok(false)
}

fn canonical_namespace_dir(kind: CanonicalArtifactKind) -> &'static str {
    kind.relative_path()
        .rsplit_once('/')
        .map(|(parent, _)| parent)
        .expect("canonical artifact path should include parent directory")
}

#[derive(Debug)]
pub enum ArtifactIngestError {
    SystemRootMissing {
        system_root: PathBuf,
    },
    SystemRootNotDir {
        system_root: PathBuf,
    },
    SystemRootSymlinkNotAllowed {
        system_root: PathBuf,
    },
    RequiredArtifactMissing {
        kind: CanonicalArtifactKind,
        path: PathBuf,
    },
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl std::fmt::Display for ArtifactIngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactIngestError::SystemRootMissing { system_root } => {
                write!(
                    f,
                    "missing canonical .system root at {}",
                    system_root.display()
                )
            }
            ArtifactIngestError::SystemRootNotDir { system_root } => write!(
                f,
                "canonical .system root is not a directory: {}",
                system_root.display()
            ),
            ArtifactIngestError::SystemRootSymlinkNotAllowed { system_root } => write!(
                f,
                "canonical .system root must not be a symlink: {}",
                system_root.display()
            ),
            ArtifactIngestError::RequiredArtifactMissing { kind, path } => write!(
                f,
                "missing required canonical artifact {kind:?} at {}",
                path.display()
            ),
            ArtifactIngestError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read canonical artifact at {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ArtifactIngestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ArtifactIngestError::ReadFailure { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactIngestIssueKind {
    CanonicalArtifactSymlinkNotAllowed,
    CanonicalArtifactReadError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactIngestIssue {
    pub kind: ArtifactIngestIssueKind,
    pub artifact_kind: CanonicalArtifactKind,
    pub canonical_repo_relative_path: &'static str,
    pub required: bool,
}

fn record_ingest_issue(
    issues: &mut Vec<ArtifactIngestIssue>,
    kind: ArtifactIngestIssueKind,
    artifact_kind: CanonicalArtifactKind,
    canonical_repo_relative_path: &'static str,
) {
    issues.push(ArtifactIngestIssue {
        kind,
        artifact_kind,
        canonical_repo_relative_path,
        required: artifact_kind.required(),
    });
}

fn load_one(
    repo_root: &Path,
    kind: CanonicalArtifactKind,
    issues: &mut Vec<ArtifactIngestIssue>,
) -> CanonicalArtifact {
    let relative_path = kind.relative_path();
    let path = repo_root.join(relative_path);

    let required = kind.required();

    let meta = match std::fs::symlink_metadata(&path) {
        Ok(meta) => Some(meta),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(_err) => {
            record_ingest_issue(
                issues,
                ArtifactIngestIssueKind::CanonicalArtifactReadError,
                kind,
                relative_path,
            );
            return missing_one(kind);
        }
    };

    if meta.is_none() {
        return missing_one(kind);
    }

    let meta = meta.expect("meta");
    if meta.file_type().is_symlink() {
        record_ingest_issue(
            issues,
            ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed,
            kind,
            relative_path,
        );
        return missing_one(kind);
    }
    if !meta.is_file() {
        record_ingest_issue(
            issues,
            ArtifactIngestIssueKind::CanonicalArtifactReadError,
            kind,
            relative_path,
        );
        return missing_one(kind);
    }

    if let Some(parent) = path.parent() {
        if let Ok(parent_meta) = std::fs::symlink_metadata(parent) {
            if parent_meta.file_type().is_symlink() {
                record_ingest_issue(
                    issues,
                    ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed,
                    kind,
                    relative_path,
                );
                return missing_one(kind);
            }
        }
    }

    let bytes = match read_bytes_no_follow(&path) {
        Ok(bytes) => bytes,
        Err(_) => {
            record_ingest_issue(
                issues,
                ArtifactIngestIssueKind::CanonicalArtifactReadError,
                kind,
                relative_path,
            );
            return missing_one(kind);
        }
    };

    let byte_len = bytes.len() as u64;
    let matches_setup_starter_template = matches_setup_starter_template(kind, &bytes);
    let presence = if byte_len == 0 {
        ArtifactPresence::PresentEmpty
    } else {
        ArtifactPresence::PresentNonEmpty
    };

    let content_sha256 = Some(sha256_hex(&bytes));

    CanonicalArtifact {
        identity: CanonicalArtifactIdentity {
            kind,
            relative_path,
            required,
            presence,
            byte_len: Some(byte_len),
            content_sha256,
            matches_setup_starter_template,
        },
        bytes: Some(bytes),
    }
}

fn read_bytes_no_follow(path: &Path) -> std::io::Result<Vec<u8>> {
    #[cfg(unix)]
    {
        use std::fs::OpenOptions;
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    #[cfg(not(unix))]
    {
        std::fs::read(path)
    }
}

fn missing_one(kind: CanonicalArtifactKind) -> CanonicalArtifact {
    let required = kind.required();
    CanonicalArtifact {
        identity: CanonicalArtifactIdentity {
            kind,
            relative_path: kind.relative_path(),
            required,
            presence: ArtifactPresence::Missing,
            byte_len: None,
            content_sha256: None,
            matches_setup_starter_template: false,
        },
        bytes: None,
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    bytes_to_lower_hex(&digest)
}

fn bytes_to_lower_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

fn descriptor_for(kind: CanonicalArtifactKind) -> &'static CanonicalArtifactDescriptor {
    CANONICAL_ARTIFACT_DESCRIPTORS
        .iter()
        .find(|descriptor| descriptor.kind == kind)
        .expect("canonical artifact descriptor should exist")
}
