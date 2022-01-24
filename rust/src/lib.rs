pub mod error;
pub mod eval;
pub mod parse;

use error::{Error, Result};

pub fn query_value(query_str: String, data: serde_json::Value) -> Result<serde_json::Value> {
    match parse::parse_query(&query_str) {
        Ok(ast) => ast.evaluate(&data),
        Err(err) => Err(Error::query(err.to_string())),
    }
}

pub fn query(query_str: String, data_str: String) -> Result<serde_json::Value> {
    match serde_json::from_str(&data_str) {
        Ok(data) => query_value(query_str, data),
        Err(err) => Err(Error::json(err)),
    }
}