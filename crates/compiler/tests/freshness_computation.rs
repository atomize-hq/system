use system_compiler::{
    compute_freshness, ArtifactPresence, CanonicalArtifactIdentity, CanonicalArtifactKind,
    FreshnessIssueKind, FreshnessStatus, InheritedDependency, OverrideTarget, OverrideWithRationale,
    C03_SCHEMA_VERSION, MANIFEST_GENERATION_VERSION,
};

fn identity(
    kind: CanonicalArtifactKind,
    presence: ArtifactPresence,
    content_sha256: Option<&str>,
    byte_len: Option<u64>,
) -> CanonicalArtifactIdentity {
    let relative_path = match kind {
        CanonicalArtifactKind::Charter => ".system/charter/CHARTER.md",
        CanonicalArtifactKind::ProjectContext => ".system/project_context/PROJECT_CONTEXT.md",
        CanonicalArtifactKind::FeatureSpec => ".system/feature_spec/FEATURE_SPEC.md",
    };
    let required = match kind {
        CanonicalArtifactKind::Charter => true,
        CanonicalArtifactKind::ProjectContext => false,
        CanonicalArtifactKind::FeatureSpec => true,
    };

    CanonicalArtifactIdentity {
        kind,
        relative_path,
        required,
        presence,
        byte_len,
        content_sha256: content_sha256.map(|s| s.to_string()),
    }
}

#[test]
fn fingerprint_excludes_diagnostics_and_is_order_invariant() {
    let a1 = identity(
        CanonicalArtifactKind::Charter,
        ArtifactPresence::PresentNonEmpty,
        Some("aaa"),
        Some(123),
    );
    let b1 = identity(
        CanonicalArtifactKind::ProjectContext,
        ArtifactPresence::Missing,
        None,
        None,
    );
    let c1 = identity(
        CanonicalArtifactKind::FeatureSpec,
        ArtifactPresence::PresentNonEmpty,
        Some("ccc"),
        Some(0),
    );

    let a2 = identity(
        CanonicalArtifactKind::Charter,
        ArtifactPresence::PresentNonEmpty,
        Some("aaa"),
        Some(999999),
    );
    let b2 = identity(
        CanonicalArtifactKind::ProjectContext,
        ArtifactPresence::Missing,
        None,
        Some(42),
    );
    let c2 = identity(
        CanonicalArtifactKind::FeatureSpec,
        ArtifactPresence::PresentNonEmpty,
        Some("ccc"),
        Some(1234),
    );

    let deps_a = vec![
        InheritedDependency {
            id: "b".to_string(),
            version: Some("2".to_string()),
            content_sha256: None,
        },
        InheritedDependency {
            id: "a".to_string(),
            version: None,
            content_sha256: Some("hash".to_string()),
        },
    ];
    let deps_b = vec![deps_a[1].clone(), deps_a[0].clone()];

    let truth_1 = compute_freshness(&[a1, b1, c1], &deps_a, &[]);
    let truth_2 = compute_freshness(&[c2, a2, b2], &deps_b, &[]);

    assert_eq!(truth_1.schema_version, C03_SCHEMA_VERSION);
    assert_eq!(truth_1.manifest_generation_version, MANIFEST_GENERATION_VERSION);
    assert_eq!(truth_1.fingerprint_sha256, truth_2.fingerprint_sha256);
    assert_eq!(truth_1.status, FreshnessStatus::Ok);
    assert!(truth_1.issues.is_empty());
}

#[test]
fn dependency_identity_changes_fingerprint() {
    let charter = identity(
        CanonicalArtifactKind::Charter,
        ArtifactPresence::PresentNonEmpty,
        Some("aaa"),
        None,
    );
    let project_context = identity(
        CanonicalArtifactKind::ProjectContext,
        ArtifactPresence::Missing,
        None,
        None,
    );
    let feature_spec = identity(
        CanonicalArtifactKind::FeatureSpec,
        ArtifactPresence::PresentNonEmpty,
        Some("ccc"),
        None,
    );

    let dep_v1 = InheritedDependency {
        id: "dep".to_string(),
        version: Some("1".to_string()),
        content_sha256: None,
    };
    let dep_v2 = InheritedDependency {
        id: "dep".to_string(),
        version: Some("2".to_string()),
        content_sha256: None,
    };

    let t1 = compute_freshness(&[charter.clone(), project_context.clone(), feature_spec.clone()], &[dep_v1], &[]);
    let t2 = compute_freshness(&[charter, project_context, feature_spec], &[dep_v2], &[]);

    assert_ne!(t1.fingerprint_sha256, t2.fingerprint_sha256);
}

#[test]
fn override_targeting_canonical_artifact_is_forbidden_and_recorded() {
    let charter = identity(
        CanonicalArtifactKind::Charter,
        ArtifactPresence::PresentNonEmpty,
        Some("aaa"),
        None,
    );
    let project_context = identity(
        CanonicalArtifactKind::ProjectContext,
        ArtifactPresence::Missing,
        None,
        None,
    );
    let feature_spec = identity(
        CanonicalArtifactKind::FeatureSpec,
        ArtifactPresence::PresentNonEmpty,
        Some("ccc"),
        None,
    );

    let no_override = compute_freshness(&[charter.clone(), project_context.clone(), feature_spec.clone()], &[], &[]);

    let override_a = OverrideWithRationale {
        target: OverrideTarget::CanonicalArtifact(CanonicalArtifactKind::Charter),
        rationale: "because".to_string(),
    };
    let override_b = OverrideWithRationale {
        target: OverrideTarget::CanonicalArtifact(CanonicalArtifactKind::Charter),
        rationale: "because, but different".to_string(),
    };

    let with_override_a = compute_freshness(
        &[charter.clone(), project_context.clone(), feature_spec.clone()],
        &[],
        &[override_a],
    );
    let with_override_b = compute_freshness(&[charter, project_context, feature_spec], &[], &[override_b]);

    assert_eq!(with_override_a.status, FreshnessStatus::Invalid);
    assert_eq!(with_override_a.override_records.len(), 1);
    assert_eq!(with_override_a.issues.len(), 1);
    assert_eq!(with_override_a.issues[0].kind, FreshnessIssueKind::ForbiddenOverride);

    assert_ne!(no_override.fingerprint_sha256, with_override_a.fingerprint_sha256);
    assert_ne!(with_override_a.fingerprint_sha256, with_override_b.fingerprint_sha256);
}
