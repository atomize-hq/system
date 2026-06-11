#![forbid(unsafe_code)]

//! Scaffold crate for future handbook pipeline extraction work.

pub fn pipeline_contract_version() -> &'static str {
    handbook_compiler::workspace_contract_version()
}
