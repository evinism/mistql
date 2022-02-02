use crate::error::{Error, Result};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

pub mod literal;

pub use literal::Literal;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'a> {
    Value(Value<'a>),
    Monad {
        op: Operator,
        target: Box<Expression<'a>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Not,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    At,
    Literal(Literal<'a>),
    EOI,
}

pub fn parse_query(query: &str) -> Result<Expression> {
    let mut pairs = MistQLParser::parse(Rule::query, query)?;
    match pairs.next() {
        Some(pair) => parse_expression(pair.into_inner().next().unwrap()),
        None => Err(Error::query(format!("no expressions found"))),
    }
}

pub fn parse_expression(pair: Pair<Rule>) -> Result<Expression> {
    match pair.into_inner().next() {
        None => Err(Error::query(format!("no expression found"))),
        Some(expr) => match expr.as_rule() {
            Rule::value => Ok(Expression::Value(parse_value(expr)?)),
            Rule::monad => {
                let mut inner = expr.into_inner();
                let operator = inner.next().unwrap();
                let target = inner.next().unwrap();

                Ok(Expression::Monad {
                    op: parse_operator(operator)?,
                    target: Box::new(parse_expression(target)?),
                })
            }
            _ => Err(Error::query(format!("unknown expression type {:?}", expr))),
        },
    }
}

pub fn parse_value(pair: Pair<Rule>) -> Result<Value> {
    match pair.into_inner().next() {
        None => Err(Error::query(format!("no value found"))),
        Some(value) => match value.as_rule() {
            Rule::at => Ok(Value::At),
            Rule::literal => Ok(Value::Literal(literal::parse_literal(value)?)),
            Rule::EOI => Ok(Value::EOI),
            _ => Err(Error::query(format!("unknown value type {:?}", value))),
        },
    }
}

pub fn parse_operator(pair: Pair<Rule>) -> Result<Operator> {
    match pair.as_str() {
        "!" => Ok(Operator::Not),
        _ => Err(Error::query(format!("unknown operator type {:?}", pair))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::parses_to;

    #[test]
    fn parse_at() {
        let query = "@";

        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::at,
            tokens: [
                at(0,1)
            ]
        }

        let pair = MistQLParser::parse(Rule::value, query)
            .unwrap()
            .next()
            .unwrap();
        let parsed = parse_value(pair).unwrap();
        assert_eq!(parsed, Value::At);
    }
}
