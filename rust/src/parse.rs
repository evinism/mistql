use crate::error::{Error, Result};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

#[derive(Clone)]
pub enum Expression {
    At,
    Value(Value),
    EOI,
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Null,
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
            Rule::value => Ok(Expression::Value(parse_value(expr)?)),
            Rule::EOI => Ok(Expression::EOI),
            _ => Err(Error::query(format!("unknown expression type {:?}", expr))),
        },
    }
}

pub fn parse_value(pair: Pair<Rule>) -> Result<Value> {
    match pair.into_inner().next() {
        None => Err(Error::query(format!("no value found"))),
        Some(value) => match value.as_rule() {
            Rule::number => Ok(Value::Number(value.as_str().parse().unwrap())),
            Rule::null => Ok(Value::Null),
            _ => Err(Error::query(format!("unknown value type {:?}", value))),
        },
    }
}

#[cfg(test)]
mod tests {}
