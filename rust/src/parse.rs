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
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Null,
}

pub fn parse_query(query: &str) -> Result<Expression> {
    let mut pairs = MistQLParser::parse(Rule::query, query)?;
    let mut exprs: Vec<Expression> = vec![];
    for p in pairs.next().unwrap().into_inner() {
        match p.as_rule() {
            Rule::at => exprs.push(Expression::At),
            Rule::value => exprs.push(Expression::Value(parse_value(p)?)),
            Rule::EOI => (),
            _ => return Err(Error::query(format!("unknown rule \"{:?}\"", p.as_rule()))),
        }
    }
    match exprs.get(0) {
        Some(expr) => Ok(expr.clone()),
        None => Err(Error::query(format!("no expressions found"))),
    }
}

pub fn parse_value(pair: Pair<Rule>) -> Result<Value> {
    // todo don't initialize this
    let mut val: Value = Value::Null;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::number => val = Value::Number(p.as_str().parse().unwrap()),
            _ => return Err(Error::query(format!("unknown value type {:?}", p))),
        }
    }
    Ok(val)
}

#[cfg(test)]
mod tests {}
