use crate::error::{Error, Result};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

#[derive(Debug)]
pub enum Node {
    At,
    Value(serde_json::Value),
}

pub fn parse_query(query: &str) -> Result<Node> {
    match MistQLParser::parse(Rule::query, query) {
        Err(err) => Err(Error::query(err.to_string())),
        Ok(mut pair) => {
            // must be a Rule::query or MistQLParser::parse would have failed
            let root = pair.next().unwrap();
            match root.as_rule() {
                Rule::at => Ok(Node::At),
                Rule::bool | Rule::number | Rule::string | Rule::null => parse_value(root),
                _ => Err(Error::query(format!(
                    "unimplemented rule {:?}",
                    root.as_rule()
                ))),
            }
        }
    }
}

fn parse_value(query: Pair<Rule>) -> Result<Node> {
    match serde_json::from_str(query.as_str()) {
        Ok(val) => Ok(Node::Value(val)),
        Err(_) => Err(Error::query(format!("unparseable value {:?}", query))),
    }
}

#[cfg(test)]
mod test {}
