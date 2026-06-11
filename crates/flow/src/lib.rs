#![forbid(unsafe_code)]

//! Scaffold crate for future handbook flow extraction work.

pub fn flow_contract_version() -> &'static str {
    handbook_compiler::workspace_contract_version()
}
