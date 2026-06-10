#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateLibraryRequest {
    CharterAuthoring,
    EnvironmentInventoryAuthoring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateLibraryAsset {
    CharterAuthoringMethod,
    CharterSynthesizeDirective,
    CharterTemplate,
    EnvironmentInventorySynthesizeDirective,
    EnvironmentInventoryTemplate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemplateLibraryDocument {
    asset: TemplateLibraryAsset,
    repo_relative_path: &'static str,
    contents: &'static str,
}

impl TemplateLibraryDocument {
    const fn new(
        asset: TemplateLibraryAsset,
        repo_relative_path: &'static str,
        contents: &'static str,
    ) -> Self {
        Self {
            asset,
            repo_relative_path,
            contents,
        }
    }

    pub const fn asset(self) -> TemplateLibraryAsset {
        self.asset
    }

    pub const fn repo_relative_path(self) -> &'static str {
        self.repo_relative_path
    }

    pub const fn contents(self) -> &'static str {
        self.contents
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharterTemplateLibrarySelection {
    authoring_method: TemplateLibraryDocument,
    synthesize_directive: TemplateLibraryDocument,
    template: TemplateLibraryDocument,
}

impl CharterTemplateLibrarySelection {
    pub const fn authoring_method(self) -> TemplateLibraryDocument {
        self.authoring_method
    }

    pub const fn synthesize_directive(self) -> TemplateLibraryDocument {
        self.synthesize_directive
    }

    pub const fn template(self) -> TemplateLibraryDocument {
        self.template
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvironmentInventoryTemplateLibrarySelection {
    synthesize_directive: TemplateLibraryDocument,
    template: TemplateLibraryDocument,
}

impl EnvironmentInventoryTemplateLibrarySelection {
    pub const fn synthesize_directive(self) -> TemplateLibraryDocument {
        self.synthesize_directive
    }

    pub const fn template(self) -> TemplateLibraryDocument {
        self.template
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateLibrarySelection {
    Charter(CharterTemplateLibrarySelection),
    EnvironmentInventory(EnvironmentInventoryTemplateLibrarySelection),
}

pub fn resolve_shipped_template_library(
    request: TemplateLibraryRequest,
) -> TemplateLibrarySelection {
    match request {
        TemplateLibraryRequest::CharterAuthoring => {
            TemplateLibrarySelection::Charter(CharterTemplateLibrarySelection {
                authoring_method: CHARTER_AUTHORING_METHOD,
                synthesize_directive: CHARTER_SYNTHESIZE_DIRECTIVE,
                template: CHARTER_TEMPLATE,
            })
        }
        TemplateLibraryRequest::EnvironmentInventoryAuthoring => {
            TemplateLibrarySelection::EnvironmentInventory(
                EnvironmentInventoryTemplateLibrarySelection {
                    synthesize_directive: ENVIRONMENT_INVENTORY_SYNTHESIZE_DIRECTIVE,
                    template: ENVIRONMENT_INVENTORY_TEMPLATE,
                },
            )
        }
    }
}

const CHARTER_AUTHORING_METHOD: TemplateLibraryDocument = TemplateLibraryDocument::new(
    TemplateLibraryAsset::CharterAuthoringMethod,
    "core/library/authoring/charter_authoring_method.md",
    include_str!("../../../core/library/authoring/charter_authoring_method.md"),
);

const CHARTER_SYNTHESIZE_DIRECTIVE: TemplateLibraryDocument = TemplateLibraryDocument::new(
    TemplateLibraryAsset::CharterSynthesizeDirective,
    "core/library/charter/charter_synthesize_directive.md",
    include_str!("../../../core/library/charter/charter_synthesize_directive.md"),
);

const CHARTER_TEMPLATE: TemplateLibraryDocument = TemplateLibraryDocument::new(
    TemplateLibraryAsset::CharterTemplate,
    "core/library/charter/charter.md.tmpl",
    include_str!("../../../core/library/charter/charter.md.tmpl"),
);

const ENVIRONMENT_INVENTORY_SYNTHESIZE_DIRECTIVE: TemplateLibraryDocument =
    TemplateLibraryDocument::new(
        TemplateLibraryAsset::EnvironmentInventorySynthesizeDirective,
        "core/library/environment_inventory/environment_inventory_directive.md",
        include_str!(
            "../../../core/library/environment_inventory/environment_inventory_directive.md"
        ),
    );

const ENVIRONMENT_INVENTORY_TEMPLATE: TemplateLibraryDocument = TemplateLibraryDocument::new(
    TemplateLibraryAsset::EnvironmentInventoryTemplate,
    "core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl",
    include_str!("../../../core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl"),
);
