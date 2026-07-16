use handbook_engine::{
    DefinitionFingerprint, RegistryLoadErrorKind, SchemaRegistry, StructuralValidationError,
};
use serde_json::{json, Value};
use std::path::Path;

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

fn write(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, bytes).expect("write fixture");
}

fn canonical_fingerprint(value: &Value) -> String {
    DefinitionFingerprint::from_json_value(value)
        .expect("canonical fingerprint")
        .to_string()
}

fn schema_bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec_pretty(value).expect("schema JSON")
}

fn write_entry(
    repo: &Path,
    entry_path: &str,
    content_schema_id: &str,
    document_ref: &str,
    closure_documents: &[(&str, &[u8])],
) {
    let root_bytes = closure_documents
        .iter()
        .find_map(|(path, bytes)| (*path == document_ref).then_some(*bytes))
        .expect("root document bytes");
    let document_fingerprint = DefinitionFingerprint::from_bytes(root_bytes).to_string();

    let mut closure = closure_documents
        .iter()
        .map(|(path, bytes)| {
            json!({
                "document_ref": path,
                "document_fingerprint": DefinitionFingerprint::from_bytes(bytes).to_string(),
            })
        })
        .collect::<Vec<_>>();
    closure.sort_by(|left, right| {
        left["document_ref"]
            .as_str()
            .cmp(&right["document_ref"].as_str())
    });
    let closure_fingerprint = canonical_fingerprint(&Value::Array(closure));
    let preimage = json!({
        "schema_id": "handbook.schema-registry-entry",
        "schema_version": "1.0",
        "content_schema_id": content_schema_id,
        "content_schema_version": "1.0.0",
        "document_ref": document_ref,
        "document_fingerprint": document_fingerprint,
        "closure_fingerprint": closure_fingerprint,
        "meta_schema_ref": DIALECT,
        "media_type": "application/schema+json",
        "compatibility": "exact",
        "extensions": {},
    });
    let entry_fingerprint = canonical_fingerprint(&preimage);
    let yaml = format!(
        "schema_id: handbook.schema-registry-entry\n\
         schema_version: \"1.0\"\n\
         content_schema_id: {content_schema_id}\n\
         content_schema_version: \"1.0.0\"\n\
         document_ref: {document_ref}\n\
         document_fingerprint: {document_fingerprint}\n\
         closure_fingerprint: {closure_fingerprint}\n\
         meta_schema_ref: {DIALECT}\n\
         media_type: application/schema+json\n\
         compatibility: exact\n\
         extensions: {{}}\n\
         entry_fingerprint: {entry_fingerprint}\n"
    );
    write(&repo.join(entry_path), yaml.as_bytes());
}

fn valid_schema_repo() -> (tempfile::TempDir, String) {
    let repo = tempfile::tempdir().expect("repo");
    let root = json!({
        "$schema": DIALECT,
        "type": "object",
        "properties": {
            "title": {"$ref": "fields.schema.json#/$defs/title"},
            "metadata": {
                "const": {"unknown_keyword_is_instance_data": true}
            }
        },
        "required": ["title"],
        "additionalProperties": false
    });
    let fields = json!({
        "$schema": DIALECT,
        "$defs": {
            "title": {"type": "string", "minLength": 3}
        }
    });
    let root_bytes = schema_bytes(&root);
    let fields_bytes = schema_bytes(&fields);
    write(&repo.path().join("schemas/root.schema.json"), &root_bytes);
    write(
        &repo.path().join("schemas/fields.schema.json"),
        &fields_bytes,
    );
    let entry_path = "definitions/incident-schema.yaml".to_string();
    write_entry(
        repo.path(),
        &entry_path,
        "example.schemas.incident-brief",
        "schemas/root.schema.json",
        &[
            ("schemas/root.schema.json", root_bytes.as_slice()),
            ("schemas/fields.schema.json", fields_bytes.as_slice()),
        ],
    );
    (repo, entry_path)
}

#[test]
fn safe_local_closure_loads_and_reports_deterministic_structural_locations() {
    let (repo, entry_path) = valid_schema_repo();
    let registry = SchemaRegistry::load(repo.path(), &[entry_path], &["schemas".to_string()])
        .expect("schema registry");
    let schema_ref =
        handbook_engine::ExactDefinitionRef::parse("example.schemas.incident-brief@1.0.0").unwrap();
    let resolved = registry.resolved(&schema_ref).expect("resolved schema");

    assert_eq!(
        resolved.closure_document_refs(),
        ["schemas/fields.schema.json", "schemas/root.schema.json"]
    );
    assert!(resolved
        .validate_json(&json!({"title": "Incident"}))
        .is_ok());

    let errors = resolved
        .validate_json(&json!({"title": "x"}))
        .expect_err("invalid data");
    assert_eq!(errors[0].instance_location(), "/title");
    assert!(errors[0]
        .schema_location()
        .contains("schemas/fields.schema.json"));
}

#[test]
fn registry_fingerprints_and_lookup_sets_are_source_order_independent() {
    let (repo, first_entry) = valid_schema_repo();
    let companion = json!({"$schema": DIALECT, "type": "boolean"});
    let companion_bytes = schema_bytes(&companion);
    write(
        &repo.path().join("schemas/companion.schema.json"),
        &companion_bytes,
    );
    let second_entry = "definitions/companion-schema.yaml".to_string();
    write_entry(
        repo.path(),
        &second_entry,
        "example.schemas.companion",
        "schemas/companion.schema.json",
        &[("schemas/companion.schema.json", companion_bytes.as_slice())],
    );

    let forward = SchemaRegistry::load(
        repo.path(),
        &[first_entry.clone(), second_entry.clone()],
        &["schemas".to_string()],
    )
    .expect("forward");
    let reverse = SchemaRegistry::load(
        repo.path(),
        &[second_entry, first_entry],
        &["schemas".to_string()],
    )
    .expect("reverse");
    assert_eq!(forward.fingerprint(), reverse.fingerprint());
    assert_eq!(forward.entry_refs(), reverse.entry_refs());
    assert_eq!(forward.entry_refs().len(), 2);
}

#[test]
fn schema_entries_are_closed_and_extensions_must_be_empty() {
    let (repo, entry_path) = valid_schema_repo();
    let path = repo.path().join(&entry_path);
    let base = std::fs::read_to_string(&path).unwrap();

    write(
        &path,
        base.replace(
            "schema_version: \"1.0\"",
            "instance_path: forbidden\nschema_version: \"1.0\"",
        )
        .as_bytes(),
    );
    let error = SchemaRegistry::load(
        repo.path(),
        std::slice::from_ref(&entry_path),
        &["schemas".to_string()],
    )
    .expect_err("wrong-record field");
    assert_eq!(error.kind(), RegistryLoadErrorKind::UnknownField);

    write(
        &path,
        base.replace("extensions: {}", "extensions: {example: true}")
            .as_bytes(),
    );
    let error = SchemaRegistry::load(repo.path(), &[entry_path], &["schemas".to_string()])
        .expect_err("non-empty extensions");
    assert_eq!(error.kind(), RegistryLoadErrorKind::UnsupportedDependency);
}

#[test]
fn schema_profile_refuses_ambient_or_rebased_resolution_and_unknown_keywords() {
    for (mutation, expected) in [
        (
            json!({"$schema": DIALECT, "$ref": "https://example.com/x"}),
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            json!({"$schema": DIALECT, "$ref": "../outside.schema.json"}),
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            json!({"$schema": DIALECT, "$id": "relative-base", "type": "string"}),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
        ),
        (
            json!({"$schema": DIALECT, "$anchor": "name", "type": "string"}),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
        ),
        (
            json!({"$schema": DIALECT, "$dynamicRef": "#name"}),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
        ),
        (
            json!({"$schema": DIALECT, "unknownAnnotation": true}),
            RegistryLoadErrorKind::UnsupportedSchemaKeyword,
        ),
        (
            json!({"$schema": DIALECT, "format": "email"}),
            RegistryLoadErrorKind::UnsupportedSchemaKeyword,
        ),
        (
            json!({"type": "string"}),
            RegistryLoadErrorKind::UnsupportedDialect,
        ),
        (
            json!({"$schema": "https://json-schema.org/draft/2019-09/schema", "type": "string"}),
            RegistryLoadErrorKind::UnsupportedDialect,
        ),
        (
            json!({"$schema": DIALECT, "properties": {"x": {"$schema": DIALECT, "type": "string"}}}),
            RegistryLoadErrorKind::UnsupportedDialect,
        ),
    ] {
        let repo = tempfile::tempdir().unwrap();
        let bytes = schema_bytes(&mutation);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/entry.yaml",
            "example.schemas.invalid",
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/entry.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err("invalid schema profile");
        assert_eq!(error.kind(), expected, "schema: {mutation}");
    }
}

#[test]
fn missing_cycle_and_conflicting_identity_fail_closed() {
    let repo = tempfile::tempdir().unwrap();
    let first = json!({"$schema": DIALECT, "$ref": "second.schema.json"});
    let second = json!({"$schema": DIALECT, "$ref": "first.schema.json"});
    let first_bytes = schema_bytes(&first);
    let second_bytes = schema_bytes(&second);
    write(&repo.path().join("schemas/first.schema.json"), &first_bytes);
    write(
        &repo.path().join("schemas/second.schema.json"),
        &second_bytes,
    );
    write_entry(
        repo.path(),
        "definitions/cycle.yaml",
        "example.schemas.cycle",
        "schemas/first.schema.json",
        &[
            ("schemas/first.schema.json", first_bytes.as_slice()),
            ("schemas/second.schema.json", second_bytes.as_slice()),
        ],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/cycle.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("cycle");
    assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceCycle);

    let (repo, entry_path) = valid_schema_repo();
    let duplicate = "definitions/duplicate.yaml";
    std::fs::copy(repo.path().join(&entry_path), repo.path().join(duplicate)).unwrap();
    for sources in [
        vec![entry_path.clone(), duplicate.to_string()],
        vec![duplicate.to_string(), entry_path.clone()],
    ] {
        let error = SchemaRegistry::load(repo.path(), &sources, &["schemas".to_string()])
            .expect_err("duplicate identity");
        assert_eq!(error.kind(), RegistryLoadErrorKind::DuplicateIdentity);
    }
}

#[test]
fn duplicate_json_missing_refs_symlinks_and_non_schema_pointer_targets_refuse() {
    let repo = tempfile::tempdir().unwrap();
    let duplicate = br#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"string","type":"number"}"#;
    write(
        &repo.path().join("schemas/duplicate.schema.json"),
        duplicate,
    );
    write_entry(
        repo.path(),
        "definitions/duplicate-json.yaml",
        "example.schemas.duplicate-json",
        "schemas/duplicate.schema.json",
        &[("schemas/duplicate.schema.json", duplicate.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/duplicate-json.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("duplicate JSON key");
    assert_eq!(error.kind(), RegistryLoadErrorKind::DuplicateKey);

    let root = json!({"$schema": DIALECT, "$ref": "missing.schema.json"});
    let root_bytes = schema_bytes(&root);
    write(
        &repo.path().join("schemas/missing-root.schema.json"),
        &root_bytes,
    );
    write_entry(
        repo.path(),
        "definitions/missing.yaml",
        "example.schemas.missing",
        "schemas/missing-root.schema.json",
        &[("schemas/missing-root.schema.json", root_bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/missing.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("missing local ref");
    assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceMissing);

    let pointer_to_data = json!({
        "$schema": DIALECT,
        "$ref": "#/const/not-a-schema",
        "const": {"not-a-schema": {"type": "string"}}
    });
    let bytes = schema_bytes(&pointer_to_data);
    write(
        &repo.path().join("schemas/data-pointer.schema.json"),
        &bytes,
    );
    write_entry(
        repo.path(),
        "definitions/data-pointer.yaml",
        "example.schemas.data-pointer",
        "schemas/data-pointer.schema.json",
        &[("schemas/data-pointer.schema.json", bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/data-pointer.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("pointer target must be a schema position");
    assert_eq!(error.kind(), RegistryLoadErrorKind::ValidatorTargetMismatch);

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let target = json!({"$schema": DIALECT, "type": "string"});
        let target_bytes = schema_bytes(&target);
        write(
            &repo.path().join("outside/target.schema.json"),
            &target_bytes,
        );
        symlink(
            "../outside/target.schema.json",
            repo.path().join("schemas/link.schema.json"),
        )
        .unwrap();
        let root = json!({"$schema": DIALECT, "$ref": "link.schema.json"});
        let root_bytes = schema_bytes(&root);
        write(
            &repo.path().join("schemas/link-root.schema.json"),
            &root_bytes,
        );
        write_entry(
            repo.path(),
            "definitions/symlink.yaml",
            "example.schemas.symlink",
            "schemas/link-root.schema.json",
            &[
                ("schemas/link-root.schema.json", root_bytes.as_slice()),
                ("schemas/link.schema.json", target_bytes.as_slice()),
            ],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/symlink.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err("symlinked ref");
        assert_eq!(error.kind(), RegistryLoadErrorKind::SymlinkSource);
    }
}

#[test]
fn every_ambient_ref_form_and_invalid_pointer_fragment_refuses() {
    for (reference, expected) in [
        (
            "file:///tmp/schema.json",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "data:application/json,{}",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "unknown:child",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "child.schema.json?revision=1",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "child\\schema.json",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "%2e%2e/outside.json",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        ("#plain-anchor", RegistryLoadErrorKind::InvalidJsonPointer),
        ("#/$defs/~2bad", RegistryLoadErrorKind::InvalidJsonPointer),
    ] {
        let repo = tempfile::tempdir().unwrap();
        let root = json!({"$schema": DIALECT, "$ref": reference});
        let bytes = schema_bytes(&root);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/entry.yaml",
            "example.schemas.bad-ref",
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/entry.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err(reference);
        assert_eq!(error.kind(), expected, "ref: {reference}");
    }
}

#[test]
fn same_document_reference_cycles_and_over_depth_closures_refuse() {
    let repo = tempfile::tempdir().unwrap();
    let cycle = json!({
        "$schema": DIALECT,
        "$defs": {
            "a": {"$ref": "#/$defs/b"},
            "b": {"$ref": "#/$defs/a"}
        },
        "$ref": "#/$defs/a"
    });
    let cycle_bytes = schema_bytes(&cycle);
    write(&repo.path().join("schemas/cycle.schema.json"), &cycle_bytes);
    write_entry(
        repo.path(),
        "definitions/cycle.yaml",
        "example.schemas.same-document-cycle",
        "schemas/cycle.schema.json",
        &[("schemas/cycle.schema.json", cycle_bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/cycle.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("same-document cycle");
    assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceCycle);

    let repo = tempfile::tempdir().unwrap();
    let mut documents = Vec::new();
    for index in 0..34 {
        let value = if index == 33 {
            json!({"$schema": DIALECT, "type": "string"})
        } else {
            json!({"$schema": DIALECT, "$ref": format!("{next}.schema.json", next = index + 1)})
        };
        let bytes = schema_bytes(&value);
        let path = format!("schemas/{index}.schema.json");
        write(&repo.path().join(&path), &bytes);
        documents.push((path, bytes));
    }
    let borrowed = documents
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect::<Vec<_>>();
    write_entry(
        repo.path(),
        "definitions/deep.yaml",
        "example.schemas.too-deep",
        "schemas/0.schema.json",
        &borrowed,
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/deep.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("over-depth closure");
    assert_eq!(error.kind(), RegistryLoadErrorKind::ReferenceDepthExceeded);
}

#[test]
fn boolean_subschemas_are_valid_targets_but_alias_paths_refuse() {
    let repo = tempfile::tempdir().unwrap();
    let root = json!({
        "$schema": DIALECT,
        "$defs": {"allowed": true},
        "$ref": "#/$defs/allowed"
    });
    let bytes = schema_bytes(&root);
    write(&repo.path().join("schemas/root.schema.json"), &bytes);
    write_entry(
        repo.path(),
        "definitions/boolean.yaml",
        "example.schemas.boolean-target",
        "schemas/root.schema.json",
        &[("schemas/root.schema.json", bytes.as_slice())],
    );
    SchemaRegistry::load(
        repo.path(),
        &["definitions/boolean.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect("boolean subschema target");

    for alias in ["./child.schema.json", "nested//child.schema.json"] {
        let repo = tempfile::tempdir().unwrap();
        let root = json!({"$schema": DIALECT, "$ref": alias});
        let bytes = schema_bytes(&root);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/alias.yaml",
            "example.schemas.alias",
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/alias.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err(alias);
        assert_eq!(error.kind(), RegistryLoadErrorKind::RemoteReferenceRefused);
    }
}

#[allow(dead_code)]
fn assert_structural_error_is_public(_: StructuralValidationError) {}
