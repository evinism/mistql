use pest::iterators::Pair;

use crate::eval::expr;
use crate::{Result, Rule};

enum PrefixOperator {
    Not,
}

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    let mut prefix_iter = pair.into_inner();
    let operator = match prefix_iter.next().unwrap().as_rule() {
        Rule::not_op => PrefixOperator::Not,
        _ => unreachable!("unrecognized prefix operator"),
    };
    let operand = expr::eval(prefix_iter.next().unwrap(), context)?;

    match operator {
        PrefixOperator::Not => {
            let falsiness = !(truthiness(&operand));
            Ok(falsiness.into())
        }
    }
}

pub fn truthiness(val: &serde_json::Value) -> bool {
    match val {
        serde_json::Value::Null => false,
        serde_json::Value::Bool(bool) => bool.clone(),
        serde_json::Value::Number(n) => n.as_f64() != Some(0.0),
        serde_json::Value::String(s) => s.len() != 0,
        serde_json::Value::Array(arr) => arr.len() != 0,
        serde_json::Value::Object(obj) => obj.len() != 0,
    }
}

#[cfg(test)]
mod tests {
    use super::truthiness;
    use crate::{MistQLParser, Rule};

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
        assert_eq!(truthiness(&serde_json::Value::Null), false)
    }

    #[test]
    fn truthiness_booleans_are_themselves() {
        assert_eq!(truthiness(&serde_json::Value::Bool(false)), false);
        assert_eq!(truthiness(&serde_json::Value::Bool(true)), true)
    }

    #[test]
    fn truthiness_zero_is_false() {
        assert_eq!(truthiness(&serde_json::Value::from(0)), false);
        assert_eq!(truthiness(&serde_json::Value::from(0.0)), false);
        assert_eq!(truthiness(&serde_json::Value::from(42)), true);
        assert_eq!(truthiness(&serde_json::Value::from(37.0)), true);
        assert_eq!(truthiness(&serde_json::Value::from(-1)), true);
        assert_eq!(truthiness(&serde_json::Value::from(-3.45)), true);
    }

    #[test]
    fn truthiness_empty_string_is_false() {
        assert_eq!(truthiness(&serde_json::Value::String(String::new())), false);
        assert_eq!(
            truthiness(&serde_json::Value::String("abc".to_string())),
            true
        );
    }

    #[test]
    fn truthiness_empty_array_is_false() {
        assert_eq!(truthiness(&serde_json::Value::Array(vec![])), false);
        assert_eq!(
            truthiness(&serde_json::Value::Array(vec![
                (1 as u32).into(),
                (2 as u32).into(),
                (3 as u32).into()
            ])),
            true
        );
    }

    #[test]
    fn truthiness_empty_object_is_false() {
        assert_eq!(
            truthiness(&serde_json::Value::Object(serde_json::Map::new())),
            false
        );

        let mut map = serde_json::Map::new();
        map.insert("a".to_string(), (1 as u32).into());
        assert_eq!(truthiness(&serde_json::Value::Object(map)), true);
    }
}
