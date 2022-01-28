use crate::error::{Error, Result};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

pub mod value;

pub use value::Value;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

#[derive(Clone)]
pub enum Expression {
    At,
    Value(Value),
    EOI,
}

pub fn parse_query(query: &str) -> Result<Expression> {
    let mut pairs = MistQLParser::parse(Rule::query, query)?;
    match pairs.next() {
        Some(pair) => parse_expression(pair),
        None => Err(Error::query(format!("no expressions found"))),
    }
}

pub fn parse_expression(pair: Pair<Rule>) -> Result<Expression> {
    match pair.into_inner().next() {
        None => Err(Error::query(format!("no expression found"))),
        Some(expr) => match expr.as_rule() {
            Rule::at => Ok(Expression::At),
            Rule::value => Ok(Expression::Value(value::parse_value(expr)?)),
            Rule::EOI => Ok(Expression::EOI),
            _ => Err(Error::query(format!("unknown expression type {:?}", expr))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::parses_to;

    #[test]
    fn parse_at() {
        parses_to! {
            parser: MistQLParser,
            input: "@",
            rule: Rule::at,
            tokens: [
                at(0,1)
            ]
        }
    }
}
