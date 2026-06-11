use crate::repo_file_access::{CompilerWorkspace, NormalizedRepoRelativePath};
use std::path::Path;

pub(crate) const RUNTIME_STATE_ROOT_RELATIVE: &str = ".handbook/state";
const RUNTIME_STATE_PIPELINE_DIR_RELATIVE: &str = ".handbook/state/pipeline";

#[derive(Clone, Copy, Debug)]
pub(crate) struct RepoLayoutRoot<'a> {
    workspace: CompilerWorkspace<'a>,
}

impl<'a> RepoLayoutRoot<'a> {
    pub(crate) fn new(repo_root: &'a Path) -> Self {
        Self {
            workspace: CompilerWorkspace::new(repo_root),
        }
    }

    pub(crate) fn runtime_state(self) -> RuntimeStateLayout<'a> {
        RuntimeStateLayout {
            workspace: self.workspace,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RuntimeStateLayout<'a> {
    workspace: CompilerWorkspace<'a>,
}

impl<'a> RuntimeStateLayout<'a> {
    pub(crate) fn state_root_relative(self) -> &'static str {
        RUNTIME_STATE_ROOT_RELATIVE
    }

    pub(crate) fn state_root(self) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(self.state_root_relative())
            .expect("runtime-state root should stay repo-relative")
    }

    pub(crate) fn route_state_relative_path(self, pipeline_id: &str) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(&format!(
                "{RUNTIME_STATE_PIPELINE_DIR_RELATIVE}/{pipeline_id}.yaml"
            ))
            .expect("runtime-state route-state path should stay repo-relative")
    }
}
