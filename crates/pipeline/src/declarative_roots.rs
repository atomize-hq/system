use std::path::Path;

pub const DECLARATIVE_ROOT: &str = "core";
pub const PIPELINES_ROOT: &str = "core/pipelines";
pub const PROFILES_ROOT: &str = "core/profiles";
pub const RUNNERS_ROOT: &str = "core/runners";
pub const PIPELINES_ROOT_DISPLAY: &str = "core/pipelines/";
pub const PROFILES_ROOT_DISPLAY: &str = "core/profiles/";
pub const RUNNERS_ROOT_DISPLAY: &str = "core/runners/";

pub fn pipeline_root() -> &'static Path {
    Path::new(PIPELINES_ROOT)
}

pub fn runner_root() -> &'static Path {
    Path::new(RUNNERS_ROOT)
}

pub fn profile_root() -> &'static Path {
    Path::new(PROFILES_ROOT)
}

pub fn runner_file(runner_id: &str) -> String {
    format!("{RUNNERS_ROOT}/{runner_id}.md")
}

pub fn profile_file(profile_id: &str, file_name: &str) -> String {
    format!("{PROFILES_ROOT}/{profile_id}/{file_name}")
}

pub fn is_profile_file(path: &str, profile_id: &str) -> bool {
    path == profile_file(profile_id, "profile.yaml")
        || path == profile_file(profile_id, "commands.yaml")
        || path == profile_file(profile_id, "conventions.md")
}

pub fn is_canonical_declarative_path(path: &str) -> bool {
    path == DECLARATIVE_ROOT || path.starts_with("core/")
}
