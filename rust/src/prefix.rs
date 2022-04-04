use pest::iterators::Pair;

use crate::{expr, Result, Rule, Value};

enum PrefixOperator {
    Not,
}

pub fn eval(pair: Pair<Rule>, data: &Value) -> Result<Value> {
    let mut prefix_iter = pair.into_inner();
    let operator = match prefix_iter.next().unwrap().as_rule() {
        Rule::not_op => PrefixOperator::Not,
        _ => unreachable!("unrecognized prefix operator"),
    };
    let operand = expr::eval(prefix_iter.next().unwrap(), data, None)?;

    match operator {
        PrefixOperator::Not => {
            let falsiness = !(truthiness(&operand));
            Ok(Value::Boolean(falsiness))
        }
    }
}

pub fn truthiness(val: &Value) -> bool {
    match val {
        Value::Null => false,
        Value::Boolean(bool) => bool.clone(),
        Value::Int(i) => *i != 0,
        Value::Float(f) => *f != 0.0,
        Value::String(s) | Value::Ident(s) => s.len() != 0,
        Value::Array(arr) => arr.len() != 0,
        Value::Object(obj) => obj.len() != 0,
    }
}

#[cfg(test)]
mod tests {
    use super::truthiness;
    use crate::{MistQLParser, Rule, Value};

    #[test]
    fn parses_prefix_operators() {
        let query = "!true";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                prefixed_value(0,5, [
                    not_op(0,1),
                    bool(1,5)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(false))
    }

    #[test]
    fn parses_prefix_operators_with_space() {
        let query = "! true";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                prefixed_value(0,6, [
                    not_op(0,1),
                    bool(2,6)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(false))
    }

    #[test]
    fn parses_doubled_prefix_operators() {
        let query = "!!true";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                prefixed_value(0,6, [
                    not_op(0,1),
                    prefixed_value(1,6, [
                        not_op(1,2),
                        bool(2,6)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(true))
    }

    #[test]
    #[ignore]
    fn parses_prefix_operator_on_expression() {
        let query = "!!(regex \"hi\")";

        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                prefixed_value(0,14, [
                    not_op(0,1),
                    prefixed_value(1,14, [
                        not_op(1,2),
                        function(3,13, [
                            ident(3,8),
                            string(9,13, [
                                inner(10,12)
                            ])
                        ])
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(true))
    }

    #[test]
    fn parses_prefix_operator_on_ident() {
        // it's weird that this is legal
        let query = "!!float";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                prefixed_value(0,7, [
                    not_op(0,1),
                    prefixed_value(1,7, [
                        not_op(1,2),
                        ident(2,7)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(true))
    }

    #[test]
    fn truthiness_null_is_false() {
        assert_eq!(truthiness(&Value::Null), false)
    }

    #[test]
    fn truthiness_booleans_are_themselves() {
        assert_eq!(truthiness(&Value::Boolean(false)), false);
        assert_eq!(truthiness(&Value::Boolean(true)), true)
    }

    #[test]
    fn truthiness_zero_is_false() {
        assert_eq!(truthiness(&Value::Int(0)), false);
        assert_eq!(truthiness(&Value::Float(0.0)), false);
        assert_eq!(truthiness(&Value::Int(42)), true);
        assert_eq!(truthiness(&Value::Float(37.0)), true);
        assert_eq!(truthiness(&Value::Int(-1)), true);
        assert_eq!(truthiness(&Value::Float(-3.45)), true);
    }

    #[test]
    fn truthiness_empty_string_is_false() {
        assert_eq!(truthiness(&Value::String(String::new())), false);
        assert_eq!(truthiness(&Value::String("abc".to_string())), true);
    }

    #[test]
    fn truthiness_empty_array_is_false() {
        assert_eq!(truthiness(&Value::Array(vec![])), false);
        assert_eq!(
            truthiness(&Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3)
            ])),
            true
        );
    }

    #[test]
    fn truthiness_empty_object_is_false() {
        assert_eq!(
            truthiness(&Value::Object(std::collections::BTreeMap::new())),
            false
        );

        let mut map = std::collections::BTreeMap::new();
        map.insert("a".to_string(), Value::Int(1));
        assert_eq!(truthiness(&Value::Object(map)), true);
    }
}
