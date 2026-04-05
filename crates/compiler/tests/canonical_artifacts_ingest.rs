use system_compiler::{
    ArtifactPresence, CanonicalArtifacts, CanonicalArtifactKind, SystemRootStatus,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[test]
fn required_artifact_missing_is_reported_as_presence_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".system/charter")).expect("mkdirs");
    std::fs::create_dir_all(repo_root.join(".system/feature_spec")).expect("mkdirs");

    // Only create FEATURE_SPEC; omit required CHARTER.
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(artifacts.charter.identity.kind, CanonicalArtifactKind::Charter);
    assert_eq!(artifacts.charter.identity.presence, ArtifactPresence::Missing);
    assert!(artifacts.charter.bytes.is_none());
}

#[test]
fn optional_missing_is_distinct_from_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        b"charter",
    );
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.project_context.identity.presence, ArtifactPresence::Missing);
    assert!(artifacts.project_context.bytes.is_none());
    assert!(artifacts.project_context.identity.content_sha256.is_none());
}

#[test]
fn empty_means_exactly_zero_bytes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        b"charter",
    );
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(
        artifacts.project_context.identity.presence,
        ArtifactPresence::PresentEmpty
    );
    assert_eq!(artifacts.project_context.identity.byte_len, Some(0));
    assert_eq!(artifacts.project_context.bytes.as_deref(), Some(b"".as_slice()));
}

#[test]
fn whitespace_only_counts_as_non_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        b"charter",
    );
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b" \n\t",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(
        artifacts.project_context.identity.presence,
        ArtifactPresence::PresentNonEmpty
    );
    assert_eq!(artifacts.project_context.identity.byte_len, Some(3));
}

#[cfg(unix)]
#[test]
fn system_root_symlink_is_not_followed_and_is_reported() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let real_system = repo_root.join("real_system");
    std::fs::create_dir_all(real_system.join("charter")).expect("mkdirs");
    std::fs::create_dir_all(real_system.join("feature_spec")).expect("mkdirs");
    write_file(&real_system.join("charter/CHARTER.md"), b"charter");
    write_file(&real_system.join("feature_spec/FEATURE_SPEC.md"), b"spec");

    symlink(&real_system, repo_root.join(".system")).expect("symlink");

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::SymlinkNotAllowed);
    assert_eq!(artifacts.charter.identity.presence, ArtifactPresence::Missing);
    assert_eq!(artifacts.feature_spec.identity.presence, ArtifactPresence::Missing);
}
