use crate::error::{Error, Result};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

#[derive(Debug)]
pub enum Node {
    At,
}

pub fn parse_query(query: &str) -> Result<Node> {
    match MistQLParser::parse(Rule::query, query) {
        Err(err) => Err(Error::query(err.to_string())),
        Ok(mut pair) => {
            // must be a Rule::query or MistQLParser::parse would have failed
            let root = pair.next().unwrap();
            match root.as_rule() {
                Rule::at => Ok(Node::At),
                _ => Err(Error::query(format!(
                    "unimplemented rule {:?}",
                    root.as_rule()
                ))),
            }
        }
    }
}

#[cfg(test)]
mod test {}
