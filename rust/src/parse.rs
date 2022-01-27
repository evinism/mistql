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

    #[test]
    fn parses_positive_integer() {
        parses_to! {
            parser: MistQLParser,
            input: "100000",
            rule: Rule::number,
            tokens: [
                number(0,6)
            ]
        }
    }

    #[test]
    fn parses_negative_integer() {
        parses_to! {
            parser: MistQLParser,
            input: "-100000",
            rule: Rule::number,
            tokens: [
                number(0,7)
            ]
        }
    }

    #[test]
    fn parses_zero() {
        parses_to! {
            parser: MistQLParser,
            input: "0",
            rule: Rule::number,
            tokens: [
                number(0,1)
            ]
        }
    }

    #[test]
    fn parses_float() {
        parses_to! {
            parser: MistQLParser,
            input: "30.5",
            rule: Rule::number,
            tokens: [
                number(0,4)
            ]
        }
    }

    #[test]
    fn parses_float_with_leading_zero() {
        parses_to! {
            parser: MistQLParser,
            input: "0.9",
            rule: Rule::number,
            tokens: [
                number(0,3)
            ]
        }
    }

    #[test]
    fn parses_negative_float() {
        parses_to! {
            parser: MistQLParser,
            input: "-30.5",
            rule: Rule::number,
            tokens: [
                number(0,5)
            ]
        }
    }

    #[test]
    fn parses_float_with_exponent() {
        parses_to! {
            parser: MistQLParser,
            input: "4.9E50",
            rule: Rule::number,
            tokens: [
                number(0,5)
            ]
        }
    }

    #[test]
    fn parses_negative_float_with_exponent() {
        parses_to! {
            parser: MistQLParser,
            input: "-30.5e-2",
            rule: Rule::number,
            tokens: [
                number(0,7)
            ]
        }
    }

    #[test]
    fn fails_to_parse_semver_as_number() {
        parses_to! {
            parser: MistQLParser,
            input: "0.9.5",
            rule: Rule::number,
            tokens: [
                number(0,4)
            ]
        }
    }

    #[test]
    fn fails_to_parse_two_zero_semver_as_number() {
        parses_to! {
            parser: MistQLParser,
            input: "0.0.5",
            rule: Rule::number,
            tokens: [
                number(0,4)
            ]
        }
    }
}
