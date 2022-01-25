use crate::error::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

#[derive(Clone)]
pub enum Expression {
    At,
}

pub fn parse_query(query: &str) -> Result<Expression> {
    let mut pairs = MistQLParser::parse(Rule::query, query)?;
    let mut exprs: Vec<Expression> = vec![];
    for p in pairs.next().unwrap().into_inner() {
        match p.as_rule() {
            Rule::at => exprs.push(Expression::At),
            Rule::EOI => (),
            _ => return Err(Error::query(format!("unknown rule \"{:?}\"", p.as_rule()))),
        }
    }
    match exprs.get(0) {
        Some(expr) => Ok(expr.clone()),
        None => Err(Error::query(format!("no expressions found"))),
    }
}

#[cfg(test)]
mod tests {}
