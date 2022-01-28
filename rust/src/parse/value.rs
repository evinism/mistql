use super::Rule;
use crate::{Error, Result};
use pest::iterators::Pair;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Number(f64),
    Null,
}

pub fn parse_value(pair: Pair<Rule>) -> Result<Value> {
    match pair.into_inner().next() {
        None => Err(Error::query(format!("no value found"))),
        Some(value) => match value.as_rule() {
            Rule::number => Ok(parse_number(value.as_str())),
            Rule::null => Ok(Value::Null),
            _ => Err(Error::query(format!("unknown value type {:?}", value))),
        },
    }
}

fn parse_number(text: &str) -> Value {
    Value::Number(text.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{MistQLParser, Rule};
    use pest::Parser;

    #[test]
    fn parses_positive_integer() {
        let query = "100000";
        parses_to! {
            parser: MistQLParser,
            input: query.clone(),
            rule: Rule::number,
            tokens: [
                number(0,6)
            ]
        }

        let pair = MistQLParser::parse(Rule::number, query)
            .unwrap()
            .next()
            .unwrap();
        let parsed = parse_number(pair.as_str());
        // TODO should probably differentiate between ints and floats
        assert_eq!(parsed, Value::Number(100000.0))
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
