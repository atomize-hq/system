use crate::ManifestError;

#[derive(Debug)]
pub enum CompilerError {
    Manifest(ManifestError),
}
