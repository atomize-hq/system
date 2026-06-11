#![forbid(unsafe_code)]

//! Scaffold crate for future handbook engine extraction work.

pub fn engine_contract_version() -> &'static str {
    handbook_compiler::workspace_contract_version()
}
