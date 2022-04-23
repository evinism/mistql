use pest::iterators::Pair;
use regex::Regex;
use snailquote::unescape;

use crate::{Error, Result, Rule, Value};

pub fn eval(pair: Pair<Rule>) -> Result<Value> {
    match pair.as_rule() {
        Rule::null => Ok(Value::Null),
        Rule::bool => match pair.as_str() {
            "true" => Ok(Value::Boolean(true)),
            "false" => Ok(Value::Boolean(false)),
            _ => unreachable!("boolean terminal"),
        },
        Rule::number => Ok(Value::Number(crate::value::Number::try_from(
            pair.as_str(),
        )?)),
        Rule::string => Ok(Value::String(unescape_string(pair)?)),
        _ => Err(Error::unimplemented(format!("terminal {:?}", pair))),
    }
}

fn unescape_string(pair: Pair<Rule>) -> Result<String> {
    let re = Regex::new(r"\\u([[:xdigit:]]{2,4})").unwrap();
    let str = re.replace_all(pair.as_str(), r"\u{$1}");

    Ok(String::from(unescape(&str)?))
}

#[cfg(test)]
mod tests {
    use crate::{MistQLParser, Rule};
    #[test]
    fn parses_null() {
        parses_to! {
            parser: MistQLParser,
            input: "null",
            rule: Rule::query,
            tokens: [
                null(0,4)
            ]
        }

        let result = crate::query("null".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Null)
    }

    #[test]
    fn parses_true() {
        parses_to! {
            parser: MistQLParser,
            input: "true",
            rule: Rule::query,
            tokens: [
                bool(0,4)
            ]
        }

        let result = crate::query("true".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(true))
    }

    #[test]
    fn parses_false() {
        parses_to! {
            parser: MistQLParser,
            input: "false",
            rule: Rule::query,
            tokens: [
                bool(0,5)
            ]
        }

        let result = crate::query("false".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(false))
    }

    #[test]
    fn ident_doesnt_begin_with_integer() {
        fails_with! {
            parser: MistQLParser,
            input: "12float",
            rule: Rule::ident,
            positives: vec![Rule::ident],
            negatives: vec![],
            pos: 0
        }
    }

    #[test]
    fn parses_positive_integer() {
        let query = "100000";
        parses_to! {
            parser: MistQLParser,
            input: query.clone(),
            rule: Rule::query,
            tokens: [
                number(0,6)
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(100000))
    }

    #[test]
    fn parses_negative_integer() {
        let query = "-100000";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                number(0,7)
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(-100000))
    }

    #[test]
    fn parses_zero() {
        let query = "0";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                number(0,1)
            ]
        }
        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(0))
    }

    #[test]
    fn parses_float() {
        let query = "30.5";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                number(0,4)
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(30.5))
    }

    #[test]
    fn parses_float_with_leading_zero() {
        let query = "0.9";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                number(0,3)
            ]
        }
        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(0.9))
    }

    #[test]
    fn parses_negative_float() {
        let query = "-30.5";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                number(0,5)
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(-30.5))
    }

    #[test]
    fn parses_float_with_exponent() {
        let query = "4.9E50";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                number(0,6)
            ]
        }
    }

    #[test]
    fn parses_negative_float_with_exponent() {
        parses_to! {
            parser: MistQLParser,
            input: "-30.5e-2",
            rule: Rule::query,
            tokens: [
                number(0,8)
            ]
        }
    }

    #[test]
    fn parses_a_string() {
        let query = "\"hello\"";

        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                string(0,7, [
                    inner(1,6)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("hello".to_string()))
    }

    #[test]
    fn parse_escaped_quotes() {
        let query = "\"\"";

        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                string(0,2, [
                    inner(1,1)
                ])
            ]
        }

        // not sure this is correct
        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String(String::new()))
    }

    #[test]
    fn parse_escaped_escapes() {
        let query = "\"\\\"\"";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                string(0,4, [
                    inner(1,3)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("\"".to_string()))
    }

    #[test]
    fn parse_unicodes() {
        let query = "\"\\u0022\\\\\\\"\"";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                string(0,12, [
                    inner(1,11)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("\"\\\"".to_string()))
    }

    #[test]
    fn parse_all_the_escapes() {
        let query = "\"\\u0022\\\\\\\"\\b\\r\\n\"";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                string(0,18, [
                    inner(1,17)
                ])
            ]
        }
    }

    #[test]
    fn parse_double_escapes() {
        let query = "\"\\\\s\"";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                string(0,5, [
                    inner(1,4)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("\\s".to_string()))
    }
}
