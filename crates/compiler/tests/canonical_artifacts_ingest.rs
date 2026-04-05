use system_compiler::{ArtifactIngestError, ArtifactPresence, CanonicalArtifacts, CanonicalArtifactKind};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[test]
fn required_artifact_missing_errors() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".system/charter")).expect("mkdirs");
    std::fs::create_dir_all(repo_root.join(".system/feature_spec")).expect("mkdirs");

    // Only create FEATURE_SPEC; omit required CHARTER.
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let err = CanonicalArtifacts::load(repo_root).expect_err("should fail");
    match err {
        ArtifactIngestError::RequiredArtifactMissing { kind, .. } => {
            assert_eq!(kind, CanonicalArtifactKind::Charter);
        }
        other => panic!("unexpected error: {other:?}"),
    }
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
fn system_root_symlink_is_refused() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let real_system = repo_root.join("real_system");
    std::fs::create_dir_all(real_system.join("charter")).expect("mkdirs");
    std::fs::create_dir_all(real_system.join("feature_spec")).expect("mkdirs");
    write_file(&real_system.join("charter/CHARTER.md"), b"charter");
    write_file(&real_system.join("feature_spec/FEATURE_SPEC.md"), b"spec");

    symlink(&real_system, repo_root.join(".system")).expect("symlink");

    let err = CanonicalArtifacts::load(repo_root).expect_err("should fail");
    match err {
        ArtifactIngestError::SystemRootSymlinkNotAllowed { .. } => {}
        other => panic!("unexpected error: {other:?}"),
    }
}
