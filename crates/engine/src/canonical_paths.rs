use crate::canonical_artifacts::CanonicalArtifactKind;
use crate::canonical_repo_support::{CanonicalWorkspace, NormalizedRepoRelativePath};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CanonicalArtifactPathContract {
    namespace_dir: &'static str,
    relative_path: &'static str,
}

impl CanonicalArtifactPathContract {
    pub(crate) const fn new(namespace_dir: &'static str, relative_path: &'static str) -> Self {
        Self {
            namespace_dir,
            relative_path,
        }
    }

    pub(crate) const fn namespace_dir(self) -> &'static str {
        self.namespace_dir
    }

    pub(crate) const fn relative_path(self) -> &'static str {
        self.relative_path
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CanonicalLayoutContract {
    system_root_relative: &'static str,
    charter: CanonicalArtifactPathContract,
    project_context: CanonicalArtifactPathContract,
    environment_inventory: CanonicalArtifactPathContract,
    feature_spec: CanonicalArtifactPathContract,
}

impl CanonicalLayoutContract {
    pub(crate) const fn new(
        system_root_relative: &'static str,
        charter: CanonicalArtifactPathContract,
        project_context: CanonicalArtifactPathContract,
        environment_inventory: CanonicalArtifactPathContract,
        feature_spec: CanonicalArtifactPathContract,
    ) -> Self {
        Self {
            system_root_relative,
            charter,
            project_context,
            environment_inventory,
            feature_spec,
        }
    }

    pub(crate) const fn system_root_relative(self) -> &'static str {
        self.system_root_relative
    }

    pub(crate) const fn artifact(
        self,
        kind: CanonicalArtifactKind,
    ) -> CanonicalArtifactPathContract {
        match kind {
            CanonicalArtifactKind::Charter => self.charter,
            CanonicalArtifactKind::ProjectContext => self.project_context,
            CanonicalArtifactKind::EnvironmentInventory => self.environment_inventory,
            CanonicalArtifactKind::FeatureSpec => self.feature_spec,
        }
    }
}

pub(crate) const HANDBOOK_PRODUCT_CANONICAL_LAYOUT: CanonicalLayoutContract =
    CanonicalLayoutContract::new(
        ".handbook",
        CanonicalArtifactPathContract::new(".handbook/charter", ".handbook/charter/CHARTER.md"),
        CanonicalArtifactPathContract::new(
            ".handbook/project_context",
            ".handbook/project_context/PROJECT_CONTEXT.md",
        ),
        CanonicalArtifactPathContract::new(
            ".handbook/environment_inventory",
            ".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md",
        ),
        CanonicalArtifactPathContract::new(
            ".handbook/feature_spec",
            ".handbook/feature_spec/FEATURE_SPEC.md",
        ),
    );

pub(crate) const CANONICAL_CHARTER_RELATIVE_PATH: &str = HANDBOOK_PRODUCT_CANONICAL_LAYOUT
    .artifact(CanonicalArtifactKind::Charter)
    .relative_path();
pub(crate) const CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH: &str = HANDBOOK_PRODUCT_CANONICAL_LAYOUT
    .artifact(CanonicalArtifactKind::ProjectContext)
    .relative_path();
pub(crate) const CANONICAL_ENVIRONMENT_INVENTORY_RELATIVE_PATH: &str =
    HANDBOOK_PRODUCT_CANONICAL_LAYOUT
        .artifact(CanonicalArtifactKind::EnvironmentInventory)
        .relative_path();
pub(crate) const CANONICAL_FEATURE_SPEC_RELATIVE_PATH: &str = HANDBOOK_PRODUCT_CANONICAL_LAYOUT
    .artifact(CanonicalArtifactKind::FeatureSpec)
    .relative_path();

pub(crate) const CANONICAL_CHARTER_NAMESPACE_DIR: &str = HANDBOOK_PRODUCT_CANONICAL_LAYOUT
    .artifact(CanonicalArtifactKind::Charter)
    .namespace_dir();
pub(crate) const CANONICAL_PROJECT_CONTEXT_NAMESPACE_DIR: &str = HANDBOOK_PRODUCT_CANONICAL_LAYOUT
    .artifact(CanonicalArtifactKind::ProjectContext)
    .namespace_dir();
pub(crate) const CANONICAL_ENVIRONMENT_INVENTORY_NAMESPACE_DIR: &str =
    HANDBOOK_PRODUCT_CANONICAL_LAYOUT
        .artifact(CanonicalArtifactKind::EnvironmentInventory)
        .namespace_dir();
pub(crate) const CANONICAL_FEATURE_SPEC_NAMESPACE_DIR: &str = HANDBOOK_PRODUCT_CANONICAL_LAYOUT
    .artifact(CanonicalArtifactKind::FeatureSpec)
    .namespace_dir();

pub(crate) fn handbook_product_canonical_layout_contract() -> &'static CanonicalLayoutContract {
    &HANDBOOK_PRODUCT_CANONICAL_LAYOUT
}

fn validate_canonical_layout_contract(contract: CanonicalLayoutContract) -> Result<(), String> {
    let _ = NormalizedRepoRelativePath::parse(contract.system_root_relative())?;

    for kind in [
        CanonicalArtifactKind::Charter,
        CanonicalArtifactKind::ProjectContext,
        CanonicalArtifactKind::EnvironmentInventory,
        CanonicalArtifactKind::FeatureSpec,
    ] {
        let artifact = contract.artifact(kind);
        let _ = NormalizedRepoRelativePath::parse(artifact.namespace_dir())?;
        let _ = NormalizedRepoRelativePath::parse(artifact.relative_path())?;
    }

    Ok(())
}

pub(crate) fn canonical_artifact_relative_path(kind: CanonicalArtifactKind) -> &'static str {
    handbook_product_canonical_layout_contract()
        .artifact(kind)
        .relative_path()
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CanonicalLayout<'repo> {
    workspace: CanonicalWorkspace<'repo>,
    contract: CanonicalLayoutContract,
}

impl<'repo> CanonicalLayout<'repo> {
    pub(crate) fn new(repo_root: &'repo Path) -> Self {
        Self::with_contract(repo_root, *handbook_product_canonical_layout_contract())
    }

    pub(crate) fn with_contract(repo_root: &'repo Path, contract: CanonicalLayoutContract) -> Self {
        validate_canonical_layout_contract(contract)
            .expect("canonical layout contract should stay repo-relative");
        Self {
            workspace: CanonicalWorkspace::new(repo_root),
            contract,
        }
    }

    pub(crate) fn workspace(self) -> CanonicalWorkspace<'repo> {
        self.workspace
    }

    pub(crate) fn contract(self) -> CanonicalLayoutContract {
        self.contract
    }

    pub(crate) fn system_root_relative(self) -> &'static str {
        self.contract().system_root_relative()
    }

    pub(crate) fn system_root(self) -> NormalizedRepoRelativePath {
        self.workspace()
            .normalize_repo_relative(self.system_root_relative())
            .expect("canonical .handbook root should stay repo-relative")
    }

    pub(crate) fn artifact_relative_path(self, kind: CanonicalArtifactKind) -> &'static str {
        self.contract().artifact(kind).relative_path()
    }

    pub(crate) fn namespace_dir(self, kind: CanonicalArtifactKind) -> &'static str {
        self.contract().artifact(kind).namespace_dir()
    }

    pub(crate) fn artifact_path(self, kind: CanonicalArtifactKind) -> NormalizedRepoRelativePath {
        self.workspace()
            .normalize_repo_relative(self.artifact_relative_path(kind))
            .expect("canonical artifact path should stay repo-relative")
    }

    pub(crate) fn namespace_dir_path(
        self,
        kind: CanonicalArtifactKind,
    ) -> NormalizedRepoRelativePath {
        self.workspace()
            .normalize_repo_relative(self.namespace_dir(kind))
            .expect("canonical namespace path should stay repo-relative")
    }
}

#[cfg(test)]
mod tests {
    use super::{CanonicalArtifactPathContract, CanonicalLayout, CanonicalLayoutContract};
    use crate::canonical_artifacts::CanonicalArtifactKind;
    use std::path::Path;

    #[test]
    fn canonical_layout_contract_can_drive_non_default_paths() {
        let contract = CanonicalLayoutContract::new(
            ".custom_handbook",
            CanonicalArtifactPathContract::new(
                ".custom_handbook/charter",
                ".custom_handbook/charter/CHARTER.md",
            ),
            CanonicalArtifactPathContract::new(
                ".custom_handbook/project_context",
                ".custom_handbook/project_context/PROJECT_CONTEXT.md",
            ),
            CanonicalArtifactPathContract::new(
                ".custom_handbook/environment_inventory",
                ".custom_handbook/environment_inventory/ENVIRONMENT_INVENTORY.md",
            ),
            CanonicalArtifactPathContract::new(
                ".custom_handbook/feature_spec",
                ".custom_handbook/feature_spec/FEATURE_SPEC.md",
            ),
        );

        let layout = CanonicalLayout::with_contract(Path::new("."), contract);

        assert_eq!(layout.system_root_relative(), ".custom_handbook");
        assert_eq!(
            layout.artifact_relative_path(CanonicalArtifactKind::Charter),
            ".custom_handbook/charter/CHARTER.md"
        );
        assert_eq!(
            layout.namespace_dir(CanonicalArtifactKind::FeatureSpec),
            ".custom_handbook/feature_spec"
        );
    }
}
