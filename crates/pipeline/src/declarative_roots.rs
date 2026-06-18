use std::path::Path;

pub const DECLARATIVE_ROOT: &str = "core";
pub const PIPELINES_ROOT: &str = "core/pipelines";
pub const PROFILES_ROOT: &str = "core/profiles";
pub const RUNNERS_ROOT: &str = "core/runners";
pub const STAGES_ROOT: &str = "core/stages";
pub const PIPELINES_ROOT_DISPLAY: &str = "core/pipelines/";
pub const PROFILES_ROOT_DISPLAY: &str = "core/profiles/";
pub const RUNNERS_ROOT_DISPLAY: &str = "core/runners/";
pub const STAGES_ROOT_DISPLAY: &str = "core/stages/";

/// Public owner for declarative repo-relative path roots.
///
/// Packet 1.1 keeps this contract bounded to path ownership and explicit
/// handbook-product defaults. Catalog discovery, pipeline loading, and stage
/// validation still follow the existing handbook-product `core/**` behavior
/// until Packet 1.2 adopts the active contract structurally.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PipelineDeclarativeRootsContract {
    pipeline_root_relative: &'static str,
    profile_root_relative: &'static str,
    runner_root_relative: &'static str,
    stage_root_relative: &'static str,
}

impl PipelineDeclarativeRootsContract {
    pub const fn from_paths(
        pipeline_root_relative: &'static str,
        profile_root_relative: &'static str,
        runner_root_relative: &'static str,
        stage_root_relative: &'static str,
    ) -> Self {
        Self {
            pipeline_root_relative,
            profile_root_relative,
            runner_root_relative,
            stage_root_relative,
        }
    }

    pub const fn pipeline_root_relative(self) -> &'static str {
        self.pipeline_root_relative
    }

    pub const fn profile_root_relative(self) -> &'static str {
        self.profile_root_relative
    }

    pub const fn runner_root_relative(self) -> &'static str {
        self.runner_root_relative
    }

    pub const fn stage_root_relative(self) -> &'static str {
        self.stage_root_relative
    }

    pub fn pipeline_root(self) -> &'static Path {
        Path::new(self.pipeline_root_relative())
    }

    pub fn profile_root(self) -> &'static Path {
        Path::new(self.profile_root_relative())
    }

    pub fn runner_root(self) -> &'static Path {
        Path::new(self.runner_root_relative())
    }

    pub fn stage_root(self) -> &'static Path {
        Path::new(self.stage_root_relative())
    }

    pub fn pipeline_file(self, file_name: &str) -> String {
        format!("{}/{file_name}", self.pipeline_root_relative())
    }

    pub fn stage_file(self, file_name: &str) -> String {
        format!("{}/{file_name}", self.stage_root_relative())
    }

    pub fn runner_file(self, runner_id: &str) -> String {
        format!("{}/{runner_id}.md", self.runner_root_relative())
    }

    pub fn profile_file(self, profile_id: &str, file_name: &str) -> String {
        format!("{}/{profile_id}/{file_name}", self.profile_root_relative())
    }

    pub fn is_profile_file(self, path: &str, profile_id: &str) -> bool {
        path == self.profile_file(profile_id, "profile.yaml")
            || path == self.profile_file(profile_id, "commands.yaml")
            || path == self.profile_file(profile_id, "conventions.md")
    }
}

pub const HANDBOOK_PRODUCT_PIPELINE_DECLARATIVE_ROOTS: PipelineDeclarativeRootsContract =
    PipelineDeclarativeRootsContract::from_paths(
        PIPELINES_ROOT,
        PROFILES_ROOT,
        RUNNERS_ROOT,
        STAGES_ROOT,
    );

/// Explicit handbook-product defaults for the current supported loader/catalog
/// boundary.
pub fn handbook_product_declarative_roots() -> &'static PipelineDeclarativeRootsContract {
    &HANDBOOK_PRODUCT_PIPELINE_DECLARATIVE_ROOTS
}

pub fn pipeline_root() -> &'static Path {
    Path::new(PIPELINES_ROOT)
}

pub fn runner_root() -> &'static Path {
    handbook_product_declarative_roots().runner_root()
}

pub fn profile_root() -> &'static Path {
    Path::new(PROFILES_ROOT)
}

pub fn stage_root() -> &'static Path {
    handbook_product_declarative_roots().stage_root()
}

pub fn pipeline_file(file_name: &str) -> String {
    handbook_product_declarative_roots().pipeline_file(file_name)
}

pub fn stage_file(file_name: &str) -> String {
    handbook_product_declarative_roots().stage_file(file_name)
}

pub fn runner_file(runner_id: &str) -> String {
    handbook_product_declarative_roots().runner_file(runner_id)
}

pub fn profile_file(profile_id: &str, file_name: &str) -> String {
    handbook_product_declarative_roots().profile_file(profile_id, file_name)
}

pub fn is_profile_file(path: &str, profile_id: &str) -> bool {
    handbook_product_declarative_roots().is_profile_file(path, profile_id)
}

pub fn is_canonical_declarative_path(path: &str) -> bool {
    path == DECLARATIVE_ROOT || path.starts_with("core/")
}
