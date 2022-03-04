#[macro_use]
extern crate pest;

mod error;

pub use error::{Error, Result};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

pub fn query_value(query_str: String, data: serde_json::Value) -> Result<serde_json::Value> {
    match MistQLParser::parse(Rule::query, &query_str) {
        Err(err) => Err(Error::query(err.to_string())),
        Ok(mut pair) => {
            // must be a Rule::query or MistQLParser::parse would have failed
            let root = pair.next().unwrap();
            Err(Error::query(format!(
                "unimplemented rule {:?}",
                root.as_rule()
            )))
        }
    }
}

pub fn query(query_str: String, data_str: String) -> Result<serde_json::Value> {
    match serde_json::from_str(&data_str) {
        Ok(data) => query_value(query_str, data),
        Err(err) => Err(Error::json(err)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
