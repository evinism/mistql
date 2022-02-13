use crate::error::{Error, Result};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

#[derive(Debug)]
pub enum Node {
    At,
    Bool(bool),
    Int(i32),
    Float(f64),
}

pub fn parse_query(query: &str) -> Result<Node> {
    match MistQLParser::parse(Rule::query, query) {
        Err(err) => Err(Error::query(err.to_string())),
        Ok(mut pair) => {
            // must be a Rule::query or MistQLParser::parse would have failed
            let root = pair.next().unwrap();
            match root.as_rule() {
                Rule::at => Ok(Node::At),
                Rule::bool => parse_bool(root.as_str()),
                Rule::number => parse_number(root.as_str()),
                _ => Err(Error::query(format!(
                    "unimplemented rule {:?}",
                    root.as_rule()
                ))),
            }
        }
    }
}

fn parse_bool(query: &str) -> Result<Node> {
    match query.parse::<bool>() {
        Ok(val) => Ok(Node::Bool(val)),
        Err(_) => unreachable!("{} doesn't parse to bool", query),
    }
}

fn parse_number(query: &str) -> Result<Node> {
    match query.parse::<f64>() {
        Ok(val) => {
            if val.fract() == 0.0 {
                Ok(Node::Int(val as i32))
            } else {
                Ok(Node::Float(val))
            }
        }
        Err(_) => unreachable!("{} doesn't parse to number", query),
    }
}

#[cfg(test)]
mod test {}
