use pest::iterators::Pair;
use std::collections::BTreeMap;

use crate::eval::{expr, Value};
use crate::{Result, Rule};

pub fn eval(pair: Pair<Rule>, data: &Value) -> Result<Value> {
    let elts = pair
        .into_inner()
        .map(|elt| {
            let mut keyval_iter = elt.into_inner();
            let key = keyval_iter.next().unwrap();
            let val = keyval_iter.next().unwrap();

            let key_str = match key.as_rule() {
                Rule::ident => key.as_str(),
                Rule::string => key.into_inner().next().unwrap().as_str(),
                _ => unreachable!("unrecognized string as object key {:?}", key),
            };

            Ok((key_str.into(), expr::eval(val, data, None)?))
        })
        .collect::<Result<BTreeMap<String, Value>>>()?;
    Ok(Value::Object(elts))
}

#[cfg(test)]
mod tests {
    use crate::{MistQLParser, Rule};

    #[test]
    fn parses_empty_object() {
        let query = "{}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                object(0,2)
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Object(serde_json::Map::new()))
    }

    #[test]
    fn parses_object_with_string_keys() {
        let query = "{\"a\": 1, \"b\": 2, \"c\": 3}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                object(0,24, [
                    keyval(1,7, [
                        string(1,4, [inner(2,3)]),
                        number(6,7)
                    ]),
                    keyval(9,15, [
                        string(9,12, [inner(10,11)]),
                        number(14,15)
                    ]),
                    keyval(17,23, [
                        string(17,20, [inner(18,19)]),
                        number(22,23)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        let mut expected = serde_json::Map::new();
        expected.insert("a".to_string(), (1 as u32).into());
        expected.insert("b".to_string(), (2 as u32).into());
        expected.insert("c".to_string(), (3 as u32).into());
        assert_eq!(result, serde_json::Value::Object(expected))
    }

    #[test]
    fn parses_object_with_mixed_type_values() {
        let query = "{\"a\": 1, \"b\": false, \"c\": \"three\"}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                object(0,34, [
                    keyval(1,7, [
                        string(1,4, [inner(2,3)]),
                        number(6,7)
                    ]),
                    keyval(9,19, [
                        string(9,12, [inner(10,11)]),
                        bool(14,19)
                    ]),
                    keyval(21,33, [
                        string(21,24, [inner(22,23)]),
                        string(26,33, [inner(27,32)])
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        let mut expected = serde_json::Map::new();
        expected.insert("a".to_string(), (1 as u32).into());
        expected.insert("b".to_string(), false.into());
        expected.insert("c".to_string(), "three".into());
        assert_eq!(result, serde_json::Value::Object(expected))
    }

    #[test]
    fn parses_object_with_unqouted_keys() {
        let query = "{a: 1, b: 2, c: 3}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                object(0,18, [
                    keyval(1,5, [
                        ident(1,2),
                        number(4,5)
                    ]),
                    keyval(7,11, [
                        ident(7,8),
                        number(10,11)
                    ]),
                    keyval(13,17, [
                        ident(13,14),
                        number(16,17)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        let mut expected = serde_json::Map::new();
        expected.insert("a".to_string(), (1 as u32).into());
        expected.insert("b".to_string(), (2 as u32).into());
        expected.insert("c".to_string(), (3 as u32).into());
        assert_eq!(result, serde_json::Value::Object(expected))
    }

    #[test]
    fn parses_object_with_stringified_int_values() {
        let query = "{a: 1, b: 2, c: \"3\"}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                object(0,20, [
                    keyval(1,5, [
                        ident(1,2),
                        number(4,5)
                    ]),
                    keyval(7,11, [
                        ident(7,8),
                        number(10,11)
                    ]),
                    keyval(13,19, [
                        ident(13,14),
                        string(16,19, [
                            inner(17,18)
                        ])
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        let mut expected = serde_json::Map::new();
        expected.insert("a".to_string(), (1 as u32).into());
        expected.insert("b".to_string(), (2 as u32).into());
        expected.insert("c".to_string(), "3".into());
        assert_eq!(result, serde_json::Value::Object(expected))
    }

    #[test]
    fn parses_object_with_expression_values() {
        let query = "{a: 1 + 2, b: 3 * 4}";

        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                object(0,20, [
                    keyval(1,9, [
                        ident(1,2),
                        infix_expr(4,9, [
                            number(4,5),
                            plus_op(6,7),
                            number(8,9)
                        ])
                    ]),
                    keyval(11,19, [
                        ident(11,12),
                        infix_expr(14,19, [
                            number(14,15),
                            mult_op(16,17),
                            number(18,19)
                        ])
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        let mut expected = serde_json::Map::new();
        expected.insert("a".to_string(), (3 as u32).into());
        expected.insert("b".to_string(), (12 as u32).into());
        assert_eq!(result, serde_json::Value::Object(expected))
    }

    #[test]
    fn fails_to_parse_unterminated_object() {
        fails_with! {
            parser: MistQLParser,
            input: "{",
            rule: Rule::query,
            positives: vec![Rule::keyval],
            negatives: vec![],
            pos: 1
        }
    }

    #[test]
    fn fails_to_parse_unterminated_object_with_contents() {
        fails_with! {
            parser: MistQLParser,
            input: "{a: 1, b: 2",
            rule: Rule::query,
            positives: vec![
                Rule::plus_op, Rule::minus_op, Rule::mult_op, Rule::div_op, Rule::mod_op,
                Rule::eq_op, Rule::ne_op, Rule::gte_op, Rule::gt_op, Rule::lte_op, Rule::lt_op,
                Rule::and_op, Rule::or_op, Rule::match_op
            ],
            negatives: vec![],
            pos: 11
        }
    }
}
