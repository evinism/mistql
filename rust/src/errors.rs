//! Error handling for MistQL

// TODO: Implement custom error types and user-friendly error messages
// TODO: Type errors, runtime errors, and reference errors

#[derive(Debug, thiserror::Error)]
pub enum MistQLError {
    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Type error: {0}")]
    Type(String),

    #[error("Reference error: {0}")]
    Reference(String),
}
