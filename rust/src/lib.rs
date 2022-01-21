pub mod error;
pub mod eval;
pub mod parse;

use error::MistQLError;

pub fn query_value(
    query_str: String,
    data: serde_json::Value,
) -> Result<serde_json::Value, MistQLError> {
    match parse::query(&query_str) {
        Ok((_, ast)) => ast.evaluate(&data),
        // Err(err) => Err(err.to_string()),
        Err(err) => Err(MistQLError::QueryParseError(err.to_string())),
    }
}

pub fn query(query_str: String, data_str: String) -> Result<serde_json::Value, MistQLError> {
    match serde_json::from_str(&data_str) {
        Ok(data) => query_value(query_str, data),
        Err(err) => Err(MistQLError::JSONParseError(err.to_string())),
    }
}
