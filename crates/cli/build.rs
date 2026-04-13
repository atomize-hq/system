use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let version_path = manifest_dir.join("../../VERSION");

    println!("cargo:rerun-if-changed={}", version_path.display());

    let version = fs::read_to_string(&version_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", version_path.display()));
    let version = version.trim();

    if version.is_empty() {
        panic!("{} must not be empty", version_path.display());
    }

    println!("cargo:rustc-env=SYSTEM_RELEASE_VERSION={version}");
}
