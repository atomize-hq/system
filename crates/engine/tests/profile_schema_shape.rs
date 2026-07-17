use handbook_engine::{DefinitionFingerprint, RegistryLoadErrorKind, SchemaRegistry};
use serde_json::{json, Value};
use std::path::Path;

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

fn write(path: &Path, bytes: impl AsRef<[u8]>) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, bytes).unwrap();
}

fn fingerprint(value: &Value) -> String {
    DefinitionFingerprint::from_json_value(value)
        .unwrap()
        .to_string()
}

fn write_entry(
    repo: &Path,
    entry_path: &str,
    identity: &str,
    document_ref: &str,
    closure: &[String],
) {
    let root_bytes = std::fs::read(repo.join(document_ref)).unwrap_or_default();
    let document_fingerprint = DefinitionFingerprint::from_bytes(&root_bytes).to_string();
    let mut members = closure
        .iter()
        .map(|path| {
            json!({
                "document_ref": path,
                "document_fingerprint": DefinitionFingerprint::from_bytes(
                    &std::fs::read(repo.join(path)).unwrap_or_default()
                ).to_string(),
            })
        })
        .collect::<Vec<_>>();
    members.sort_by(|left, right| {
        left["document_ref"]
            .as_str()
            .cmp(&right["document_ref"].as_str())
    });
    let preimage = json!({
        "schema_id": "handbook.schema-registry-entry",
        "schema_version": "1.0",
        "content_schema_id": identity,
        "content_schema_version": "1.0.0",
        "document_ref": document_ref,
        "document_fingerprint": document_fingerprint,
        "closure_fingerprint": fingerprint(&Value::Array(members)),
        "meta_schema_ref": DIALECT,
        "media_type": "application/schema+json",
        "compatibility": "exact",
        "extensions": {},
    });
    let mut record = preimage.as_object().unwrap().clone();
    record.insert("entry_fingerprint".into(), fingerprint(&preimage).into());
    write(
        repo.join(entry_path).as_path(),
        serde_yaml_bw::to_string(&record).unwrap(),
    );
}

fn write_closure(repo: &Path, prefix: &str, children: usize) -> (String, String, Vec<String>) {
    let root = format!("schemas/{prefix}/root.schema.json");
    let child_paths = (0..children)
        .map(|index| format!("schemas/{prefix}/{index:03}.schema.json"))
        .collect::<Vec<_>>();
    let refs = child_paths
        .iter()
        .map(|path| {
            let name = Path::new(path).file_name().unwrap().to_str().unwrap();
            json!({"$ref": name})
        })
        .collect::<Vec<_>>();
    write(
        repo.join(&root).as_path(),
        serde_json::to_vec(&json!({"$schema": DIALECT, "allOf": refs})).unwrap(),
    );
    for path in &child_paths {
        write(
            repo.join(path).as_path(),
            serde_json::to_vec(&json!({"$schema": DIALECT, "type": "string"})).unwrap(),
        );
    }
    let mut closure = vec![root.clone()];
    closure.extend(child_paths);
    let entry = format!("definitions/{prefix}.entry.yaml");
    write_entry(
        repo,
        &entry,
        &format!("example.schemas.{prefix}"),
        &root,
        &closure,
    );
    (entry, root, closure)
}

#[test]
fn schema_document_budget_is_request_wide_and_deduplicates_shared_identity() {
    let exact = tempfile::tempdir().unwrap();
    let (entry_a, _, _) = write_closure(exact.path(), "alpha", 63);
    let (entry_b, _, _) = write_closure(exact.path(), "bravo", 63);
    SchemaRegistry::load(exact.path(), &[entry_a, entry_b], &["schemas".into()])
        .expect("128 distinct documents admit");

    let over = tempfile::tempdir().unwrap();
    let (entry_a, _, _) = write_closure(over.path(), "alpha", 63);
    let (entry_b, _, _) = write_closure(over.path(), "bravo", 64);
    let error = SchemaRegistry::load(over.path(), &[entry_a, entry_b], &["schemas".into()])
        .expect_err("129 distinct documents refuse");
    assert_eq!(error.kind(), RegistryLoadErrorKind::DocumentLimitExceeded);

    let shared = tempfile::tempdir().unwrap();
    let (entry_a, root, closure) = write_closure(shared.path(), "shared", 127);
    let entry_b = "definitions/shared-alias.entry.yaml";
    write_entry(
        shared.path(),
        entry_b,
        "example.schemas.shared-alias",
        &root,
        &closure,
    );
    SchemaRegistry::load(
        shared.path(),
        &[entry_a, entry_b.into()],
        &["schemas".into()],
    )
    .expect("the same 128 document identities count once request-wide");
}

#[test]
fn document_and_transitive_targets_enforce_path_limits_before_open() {
    let repo = tempfile::tempdir().unwrap();
    let path_1024 = format!("{}/{}", vec!["a".repeat(15); 63].join("/"), "b".repeat(16));
    let path_1025 = format!("{path_1024}x");
    let components_64 = vec!["c"; 64].join("/");
    let components_65 = vec!["c"; 65].join("/");
    assert_eq!(path_1024.len(), 1024);

    for (document_ref, expected) in [
        (
            path_1024.as_str(),
            RegistryLoadErrorKind::LocalReferenceMissing,
        ),
        (path_1025.as_str(), RegistryLoadErrorKind::InvalidSourcePath),
        (
            components_64.as_str(),
            RegistryLoadErrorKind::LocalReferenceMissing,
        ),
        (
            components_65.as_str(),
            RegistryLoadErrorKind::InvalidSourcePath,
        ),
    ] {
        write_entry(
            repo.path(),
            "definitions/path.entry.yaml",
            "example.schemas.path-boundary",
            document_ref,
            &[document_ref.to_string()],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/path.entry.yaml".into()],
            &[document_ref.split('/').next().unwrap().to_string()],
        )
        .expect_err(document_ref);
        assert_eq!(error.kind(), expected, "{document_ref}");
    }

    let root = "schemas/root.schema.json";
    write(
        repo.path().join(root).as_path(),
        serde_json::to_vec(&json!({"$schema": DIALECT, "$ref": path_1025})).unwrap(),
    );
    write_entry(
        repo.path(),
        "definitions/transitive.entry.yaml",
        "example.schemas.transitive-boundary",
        root,
        &[root.to_string()],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/transitive.entry.yaml".into()],
        &["schemas".into()],
    )
    .expect_err("oversize transitive target refuses before open");
    assert_eq!(error.kind(), RegistryLoadErrorKind::InvalidSourcePath);
}
