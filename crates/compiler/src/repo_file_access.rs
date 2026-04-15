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
pub(crate) enum RepoRelativeWritePathError {
    InvalidPath(String),
    ParentNotDirectory(PathBuf),
    NotRegularFile(PathBuf),
    SymlinkNotAllowed(PathBuf),
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
}

pub(crate) fn read_repo_relative_string(
    repo_root: &Path,
    relative_path: &str,
) -> Result<String, RepoRelativeFileAccessError> {
    let path = resolve_repo_relative_regular_file(repo_root, relative_path)?;
    read_string_no_follow(&path)
        .map_err(|source| RepoRelativeFileAccessError::ReadFailure { path, source })
}

pub(crate) fn sha256_repo_relative_file(
    repo_root: &Path,
    relative_path: &str,
) -> Result<String, RepoRelativeFileAccessError> {
    let path = resolve_repo_relative_regular_file(repo_root, relative_path)?;
    let bytes = read_bytes_no_follow(&path)
        .map_err(|source| RepoRelativeFileAccessError::ReadFailure { path, source })?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

pub(crate) fn resolve_repo_relative_write_path(
    repo_root: &Path,
    relative_path: &str,
) -> Result<PathBuf, RepoRelativeWritePathError> {
    let relative_path = validate_repo_relative_path(relative_path)
        .map_err(RepoRelativeWritePathError::InvalidPath)?;

    let mut current = repo_root.to_path_buf();
    let mut components = relative_path.components().peekable();

    while let Some(component) = components.next() {
        let Component::Normal(part) = component else {
            continue;
        };
        current.push(part);
        let is_last = components.peek().is_none();

        match fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    return Err(RepoRelativeWritePathError::SymlinkNotAllowed(
                        current.clone(),
                    ));
                }

                if is_last {
                    if metadata.is_dir() {
                        return Err(RepoRelativeWritePathError::NotRegularFile(current.clone()));
                    }
                    if !metadata.is_file() {
                        return Err(RepoRelativeWritePathError::NotRegularFile(current.clone()));
                    }
                } else if !metadata.is_dir() {
                    return Err(RepoRelativeWritePathError::ParentNotDirectory(
                        current.clone(),
                    ));
                }
            }
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                continue;
            }
            Err(source) => {
                return Err(RepoRelativeWritePathError::ReadFailure {
                    path: current.clone(),
                    source,
                });
            }
        }
    }

    Ok(current)
}

pub(crate) fn read_bytes_no_follow_path(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    read_bytes_no_follow(path)
}

pub(crate) fn read_string_no_follow_path(path: &Path) -> Result<String, std::io::Error> {
    read_string_no_follow(path)
}

fn resolve_repo_relative_regular_file(
    repo_root: &Path,
    relative_path: &str,
) -> Result<PathBuf, RepoRelativeFileAccessError> {
    let relative_path = validate_repo_relative_path(relative_path)
        .map_err(RepoRelativeFileAccessError::InvalidPath)?;

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

pub(crate) fn validate_repo_relative_path(path: &str) -> Result<&Path, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("path must not be empty".to_string());
    }

    let path = Path::new(trimmed);
    if path.is_absolute() {
        return Err("path must be repo-relative".to_string());
    }

    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            Component::ParentDir => return Err("path must not escape the repo root".to_string()),
            Component::RootDir | Component::Prefix(_) => {
                return Err("path must be repo-relative".to_string())
            }
        }
    }

    Ok(path)
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

pub(crate) fn read_bytes_no_follow(path: &Path) -> Result<Vec<u8>, std::io::Error> {
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
