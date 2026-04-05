use crate::{ManifestError, RefusalPlaceholder};

#[derive(Debug)]
pub enum CompilerError {
    Manifest(ManifestError),
    Refusal(RefusalPlaceholder),
}
