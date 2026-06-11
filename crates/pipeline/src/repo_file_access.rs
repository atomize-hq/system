use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub(crate) enum RepoRelativeFileAccessError {
    Missing(PathBuf),
    InvalidPath(String),
    SymlinkNotAllowed(PathBuf),
    NotRegularFile(PathBuf),
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Debug)]
pub(crate) enum RepoRelativeDirectoryAccessError {
    Missing(PathBuf),
    SymlinkNotAllowed(PathBuf),
    NotDirectory(PathBuf),
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NormalizedRepoRelativePath(String);

impl NormalizedRepoRelativePath {
    pub(crate) fn parse(path: &str) -> Result<Self, String> {
        let path = validate_repo_relative_path(path)?;
        let normalized = normalize_validated_repo_relative_path(path)?;
        Ok(Self(normalized))
    }

    pub(crate) fn parse_path(path: &Path) -> Result<Self, String> {
        let path = validate_repo_relative_path_from_path(path)?;
        let normalized = normalize_validated_repo_relative_path(path)?;
        Ok(Self(normalized))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    fn as_path(&self) -> &Path {
        Path::new(self.as_str())
    }
}

#[derive(Debug)]
pub(crate) struct RepoRelativeMetadataReadError {
    pub(crate) path: PathBuf,
    pub(crate) source: std::io::Error,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CompilerWorkspace<'a> {
    repo_root: &'a Path,
}

impl<'a> CompilerWorkspace<'a> {
    pub(crate) fn new(repo_root: &'a Path) -> Self {
        Self { repo_root }
    }

    pub(crate) fn normalize_repo_relative(
        &self,
        path: &str,
    ) -> Result<NormalizedRepoRelativePath, String> {
        NormalizedRepoRelativePath::parse(path)
    }

    pub(crate) fn normalize_repo_relative_path(
        &self,
        path: &Path,
    ) -> Result<NormalizedRepoRelativePath, String> {
        NormalizedRepoRelativePath::parse_path(path)
    }

    pub(crate) fn metadata_no_follow(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<Option<fs::Metadata>, RepoRelativeMetadataReadError> {
        let absolute_path = self.absolute_path(relative_path);
        match fs::symlink_metadata(&absolute_path) {
            Ok(metadata) => Ok(Some(metadata)),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(RepoRelativeMetadataReadError {
                path: absolute_path,
                source,
            }),
        }
    }

    pub(crate) fn trusted_read(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<TrustedRepoFile, RepoRelativeFileAccessError> {
        let absolute_path =
            resolve_repo_relative_regular_file_path(self.repo_root, relative_path.as_path())?;
        Ok(TrustedRepoFile {
            repo_relative: relative_path.clone(),
            absolute_path,
        })
    }

    pub(crate) fn trusted_directory(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<TrustedRepoDirectory, RepoRelativeDirectoryAccessError> {
        let absolute_path =
            resolve_repo_relative_directory_path(self.repo_root, relative_path.as_path())?;
        Ok(TrustedRepoDirectory { absolute_path })
    }

    pub(crate) fn read_string(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<String, RepoRelativeFileAccessError> {
        let trusted_file = self.trusted_read(relative_path)?;
        trusted_file
            .read_string()
            .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
                path: trusted_file.absolute_path.clone(),
                source,
            })
    }

    pub(crate) fn sha256_file(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<String, RepoRelativeFileAccessError> {
        let trusted_file = self.trusted_read(relative_path)?;
        trusted_file
            .sha256_hex()
            .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
                path: trusted_file.absolute_path.clone(),
                source,
            })
    }

    fn absolute_path(&self, relative_path: &NormalizedRepoRelativePath) -> PathBuf {
        self.repo_root.join(relative_path.as_path())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TrustedRepoDirectory {
    absolute_path: PathBuf,
}

impl TrustedRepoDirectory {
    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TrustedRepoFile {
    repo_relative: NormalizedRepoRelativePath,
    absolute_path: PathBuf,
}

impl TrustedRepoFile {
    #[cfg(test)]
    pub(crate) fn repo_relative(&self) -> &NormalizedRepoRelativePath {
        &self.repo_relative
    }

    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    pub(crate) fn read_string(&self) -> Result<String, std::io::Error> {
        read_string_no_follow(&self.absolute_path)
    }

    pub(crate) fn read_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        read_bytes_no_follow(&self.absolute_path)
    }

    pub(crate) fn sha256_hex(&self) -> Result<String, std::io::Error> {
        let mut hasher = Sha256::new();
        hasher.update(self.read_bytes()?);
        Ok(format!("{:x}", hasher.finalize()))
    }
}

pub(crate) fn sha256_repo_relative_file(
    repo_root: &Path,
    relative_path: &str,
) -> Result<String, RepoRelativeFileAccessError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative(relative_path)
        .map_err(RepoRelativeFileAccessError::InvalidPath)?;
    workspace.sha256_file(&relative_path)
}

fn resolve_repo_relative_regular_file_path(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<PathBuf, RepoRelativeFileAccessError> {
    let mut current = repo_root.to_path_buf();
    let mut components = relative_path.components().peekable();

    while let Some(component) = components.next() {
        let Component::Normal(part) = component else {
            continue;
        };
        current.push(part);

        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(RepoRelativeFileAccessError::Missing(current.clone()))
            }
            Err(source) => {
                return Err(RepoRelativeFileAccessError::ReadFailure {
                    path: current.clone(),
                    source,
                })
            }
        };

        if metadata.file_type().is_symlink() {
            return Err(RepoRelativeFileAccessError::SymlinkNotAllowed(
                current.clone(),
            ));
        }

        let is_last = components.peek().is_none();
        if is_last {
            if !metadata.is_file() {
                return Err(RepoRelativeFileAccessError::NotRegularFile(current.clone()));
            }
        } else if !metadata.is_dir() {
            return Err(RepoRelativeFileAccessError::NotRegularFile(current.clone()));
        }
    }

    Ok(current)
}

fn resolve_repo_relative_directory_path(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<PathBuf, RepoRelativeDirectoryAccessError> {
    let mut current = repo_root.to_path_buf();

    for component in relative_path.components() {
        let Component::Normal(part) = component else {
            continue;
        };
        current.push(part);

        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(RepoRelativeDirectoryAccessError::Missing(current.clone()))
            }
            Err(source) => {
                return Err(RepoRelativeDirectoryAccessError::ReadFailure {
                    path: current.clone(),
                    source,
                })
            }
        };

        if metadata.file_type().is_symlink() {
            return Err(RepoRelativeDirectoryAccessError::SymlinkNotAllowed(
                current.clone(),
            ));
        }

        if !metadata.is_dir() {
            return Err(RepoRelativeDirectoryAccessError::NotDirectory(
                current.clone(),
            ));
        }
    }

    Ok(current)
}

fn validate_repo_relative_path(path: &str) -> Result<&Path, String> {
    let trimmed = path.trim();
    validate_repo_relative_path_from_path(Path::new(trimmed))
}

fn validate_repo_relative_path_from_path(path: &Path) -> Result<&Path, String> {
    if path.as_os_str().is_empty() {
        return Err("path must not be empty".to_string());
    }

    if path.is_absolute() {
        return Err("path must be repo-relative".to_string());
    }

    for component in path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            Component::ParentDir => return Err("path must not escape the repo root".to_string()),
            Component::RootDir | Component::Prefix(_) => {
                return Err("path must be repo-relative".to_string())
            }
        }
    }

    Ok(path)
}

fn normalize_validated_repo_relative_path(path: &Path) -> Result<String, String> {
    let normalized = path
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            Component::CurDir => None,
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/");
    if normalized.is_empty() {
        return Err("path must not be empty".to_string());
    }
    Ok(normalized)
}

fn read_string_no_follow(path: &Path) -> Result<String, std::io::Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[cfg(not(unix))]
    {
        fs::read_to_string(path)
    }
}

fn read_bytes_no_follow(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    #[cfg(unix)]
    {
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
        fs::read(path)
    }
}

#[cfg(test)]
mod tests {
    use super::{CompilerWorkspace, RepoRelativeDirectoryAccessError, RepoRelativeFileAccessError};
    use std::fs;

    #[test]
    fn workspace_normalizes_repo_relative_paths() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let workspace = CompilerWorkspace::new(repo_root.path());

        assert_eq!(
            workspace
                .normalize_repo_relative("./nested/./output.txt")
                .expect("normalize repo-relative path")
                .as_str(),
            "nested/output.txt"
        );
    }

    #[cfg(unix)]
    #[test]
    fn workspace_trusted_read_refuses_symlink_targets() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        let link_path = repo_root.path().join("nested/output.txt");
        fs::create_dir_all(link_path.parent().expect("link parent")).expect("create parent");
        fs::write(external_root.path().join("output.txt"), "outside\n").expect("seed outside");
        std::os::unix::fs::symlink(external_root.path().join("output.txt"), &link_path)
            .expect("symlink file");
        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested/output.txt")
            .expect("normalize repo-relative path");

        let err = workspace.trusted_read(&path).expect_err("symlink refusal");

        assert!(
            matches!(err, RepoRelativeFileAccessError::SymlinkNotAllowed(ref found) if found == &link_path),
            "expected symlink refusal, got: {err:?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn trusted_repo_file_read_uses_no_follow_after_resolution() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        let file_path = repo_root.path().join("nested/output.txt");
        fs::create_dir_all(file_path.parent().expect("file parent")).expect("create parent");
        fs::write(&file_path, "inside\n").expect("seed repo file");
        fs::write(external_root.path().join("output.txt"), "outside\n").expect("seed outside");

        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested/output.txt")
            .expect("normalize repo-relative path");
        let trusted_file = workspace.trusted_read(&path).expect("resolve regular file");

        fs::remove_file(&file_path).expect("remove repo file");
        std::os::unix::fs::symlink(external_root.path().join("output.txt"), &file_path)
            .expect("replace repo file with symlink");

        let err = trusted_file.read_string().expect_err("no-follow refusal");

        assert_eq!(trusted_file.repo_relative().as_str(), "nested/output.txt");
        assert_eq!(trusted_file.absolute_path(), file_path.as_path());
        assert_eq!(
            err.raw_os_error(),
            Some(libc::ELOOP),
            "expected O_NOFOLLOW refusal, got: {err}"
        );
        assert_eq!(
            fs::read_to_string(external_root.path().join("output.txt")).expect("read outside"),
            "outside\n"
        );
    }

    #[cfg(unix)]
    #[test]
    fn workspace_trusted_directory_refuses_symlink_targets() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        std::os::unix::fs::symlink(external_root.path(), repo_root.path().join("nested"))
            .expect("symlink directory");
        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested")
            .expect("normalize repo-relative path");

        let err = workspace
            .trusted_directory(&path)
            .expect_err("symlink refusal");

        assert!(
            matches!(err, RepoRelativeDirectoryAccessError::SymlinkNotAllowed(_)),
            "expected symlink refusal, got: {err:?}"
        );
    }
}
