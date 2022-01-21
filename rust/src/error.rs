use thiserror::Error;

#[derive(Error, Debug)]
pub enum MistQLError {
    #[error("JSON parse error {0}")]
    JSONParseError(String),
    #[error("Parse error {0}")]
    QueryParseError(String),
    #[error("Argument error {0}")]
    ArgumentError(String),
    #[error("Unimplemented function {0}")]
    UnimplementedFunction(String),
    #[error("Unimplemented evaluation {0}")]
    UnimplementedEvaluation(String),
}
