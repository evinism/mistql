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
    Array(Vec<Node>),
}

pub fn parse_query(query: &str) -> Result<Node> {
    match MistQLParser::parse(Rule::query, query) {
        Err(err) => Err(Error::query(err.to_string())),
        Ok(mut pair) => {
            // must be a Rule::query or MistQLParser::parse would have failed
            let root = pair.next().unwrap();
            parse_expr(root)
        }
    }
}

fn parse_expr(expr: Pair<Rule>) -> Result<Node> {
    match expr.as_rule() {
        Rule::at => Ok(Node::At),
        Rule::bool | Rule::number | Rule::string | Rule::null => parse_value(expr),
        Rule::array => parse_array(expr),
        _ => Err(Error::query(format!(
            "unimplemented rule {:?}",
            expr.as_rule()
        ))),
    }
}

fn parse_value(value: Pair<Rule>) -> Result<Node> {
    match serde_json::from_str(value.as_str()) {
        Ok(val) => Ok(Node::Value(val)),
        Err(_) => Err(Error::query(format!("unparseable value {:?}", value))),
    }
}

fn parse_array(array: Pair<Rule>) -> Result<Node> {
    let elts: Vec<Node> = array
        .into_inner()
        .map(|elt| parse_expr(elt))
        .collect::<Result<Vec<Node>>>()?;
    Ok(Node::Array(elts))
}

#[cfg(test)]
mod test {}
