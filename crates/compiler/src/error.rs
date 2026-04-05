use crate::RefusalPlaceholder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilerError {
    Refusal(RefusalPlaceholder),
}
