use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalArtifactKind {
    Charter,
    ProjectContext,
    FeatureSpec,
}

impl CanonicalArtifactKind {
    fn required(self) -> bool {
        match self {
            CanonicalArtifactKind::Charter => true,
            CanonicalArtifactKind::ProjectContext => false,
            CanonicalArtifactKind::FeatureSpec => true,
        }
    }

    fn relative_path(self) -> &'static str {
        match self {
            CanonicalArtifactKind::Charter => ".system/charter/CHARTER.md",
            CanonicalArtifactKind::ProjectContext => ".system/project_context/PROJECT_CONTEXT.md",
            CanonicalArtifactKind::FeatureSpec => ".system/feature_spec/FEATURE_SPEC.md",
        }
    }
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
                } else {
                    SystemRootStatus::Ok
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

        let (charter, project_context, feature_spec) = match system_root_status {
            SystemRootStatus::Ok => (
                load_one(repo_root, CanonicalArtifactKind::Charter)?,
                load_one(repo_root, CanonicalArtifactKind::ProjectContext)?,
                load_one(repo_root, CanonicalArtifactKind::FeatureSpec)?,
            ),
            SystemRootStatus::Missing | SystemRootStatus::NotDir | SystemRootStatus::SymlinkNotAllowed => (
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

#[derive(Debug)]
pub enum ArtifactIngestError {
    SystemRootMissing { system_root: PathBuf },
    SystemRootNotDir { system_root: PathBuf },
    SystemRootSymlinkNotAllowed { system_root: PathBuf },
    RequiredArtifactMissing {
        kind: CanonicalArtifactKind,
        path: PathBuf,
    },
    ReadFailure { path: PathBuf, source: std::io::Error },
}

impl std::fmt::Display for ArtifactIngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactIngestError::SystemRootMissing { system_root } => {
                write!(f, "missing canonical .system root at {}", system_root.display())
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
                write!(f, "failed to read canonical artifact at {}: {source}", path.display())
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

fn load_one(repo_root: &Path, kind: CanonicalArtifactKind) -> Result<CanonicalArtifact, ArtifactIngestError> {
    let relative_path = kind.relative_path();
    let path = repo_root.join(relative_path);

    let meta = match std::fs::metadata(&path) {
        Ok(meta) => Some(meta),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(err) => {
            return Err(ArtifactIngestError::ReadFailure {
                path,
                source: err,
            })
        }
    };

    let required = kind.required();
    if meta.is_none() {
        return Ok(missing_one(kind));
    }

    let bytes = std::fs::read(&path).map_err(|err| ArtifactIngestError::ReadFailure {
        path: path.clone(),
        source: err,
    })?;

    let byte_len = bytes.len() as u64;
    let presence = if byte_len == 0 {
        ArtifactPresence::PresentEmpty
    } else {
        ArtifactPresence::PresentNonEmpty
    };

    let content_sha256 = Some(sha256_hex(&bytes));

    Ok(CanonicalArtifact {
        identity: CanonicalArtifactIdentity {
            kind,
            relative_path,
            required,
            presence,
            byte_len: Some(byte_len),
            content_sha256,
        },
        bytes: Some(bytes),
    })
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
