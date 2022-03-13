use pest::iterators::Pair;

use crate::eval::expr;
use crate::{Result, Rule};

pub fn eval(pair: Pair<Rule>, data: &serde_json::Value) -> Result<serde_json::Value> {
    Ok(pair
        .into_inner()
        .map(|elt| expr::eval(elt, data, None))
        .collect::<Result<Vec<serde_json::Value>>>()?
        .into())
}

#[cfg(test)]
mod tests {
    use crate::{MistQLParser, Rule};
    #[test]
    fn parses_empty_array() {
        let query = "[]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                array(0,2)
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Array(vec![]))
    }
    #[test]
    fn parses_array_of_integers() {
        let query = "[1,2,3]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                array(0,7, [
                    number(1,2),
                    number(3,4),
                    number(5,6)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            // the joys of type-safe JSON
            serde_json::Value::Array(vec![
                (1 as u32).into(),
                (2 as u32).into(),
                (3 as u32).into()
            ])
        )
    }
    #[test]
    fn parses_array_of_strings() {
        let query = "[\"a\",\"b\",\"c\"]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                array(0,13, [
                    string(1,4, [inner(2,3)]),
                    string(5,8, [inner(6,7)]),
                    string(9,12, [inner(10,11)])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec!["a".into(), "b".into(), "c".into()])
        )
    }
    #[test]
    fn parses_array_of_booleans() {
        let query = "[true, false, true]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                array(0,19, [
                    bool(1,5),
                    bool(7,12),
                    bool(14,18)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![true.into(), false.into(), true.into()])
        )
    }
    #[test]
    fn parses_array_of_nulls() {
        let query = "[null, null]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                array(0,12, [
                    null(1,5),
                    null(7,11),
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![serde_json::Value::Null, serde_json::Value::Null])
        )
    }
    #[test]
    fn parses_array_with_mixed_types() {
        let query = "[null, true, 3, \"d\"]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                array(0,20, [
                    null(1,5),
                    bool(7,11),
                    number(13,14),
                    string(16,19, [inner(17,18)])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                serde_json::Value::Null,
                serde_json::Value::Bool(true),
                (3 as u32).into(),
                "d".into()
            ])
        )
    }
    #[test]
    fn parses_array_of_arrays() {
        let query = "[[1,2],3]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                array(0,9, [
                    array(1,6, [
                        number(2,3),
                        number(4,5),
                    ]),
                    number(7,8)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                serde_json::Value::Array(vec![(1 as u32).into(), (2 as u32).into()]),
                (3 as u32).into()
            ])
        )
    }

    #[test]
    fn parses_array_with_expression_elements() {
        let query = "[3 + 2, 4 + 3]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                array(0,14, [
                    infix_expr(1,6, [
                        number(1,2),
                        plus_op(3,4),
                        number(5,6)
                    ]),
                    infix_expr(8,13, [
                        number(8,9),
                        plus_op(10,11),
                        number(12,13)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![(5 as u32).into(), (7 as u32).into()])
        )
    }

    #[test]
    fn fails_to_parse_unterminated_array() {
        fails_with! {
            parser: MistQLParser,
            input: "[",
            rule: Rule::query,
            positives: vec![
                Rule::piped_expr, Rule::function, Rule::indexed_value, Rule::not_op,
                Rule::infix_expr, Rule::compound_reference, Rule::object, Rule::array,
                Rule::ident, Rule::string, Rule::number, Rule::bool, Rule::null, Rule::at,
                Rule::dollar
            ],
            negatives: vec![],
            pos: 1
        }
    }
    #[test]
    fn fails_to_parse_unterminated_array_with_contents() {
        fails_with! {
            parser: MistQLParser,
            input: "[1,2,3",
            rule: Rule::query,
            positives: vec![
                Rule::plus_op, Rule::minus_op, Rule::mult_op, Rule::div_op, Rule::mod_op,
                Rule::eq_op, Rule::ne_op, Rule::gte_op, Rule::gt_op, Rule::lte_op, Rule::lt_op,
                Rule::and_op, Rule::or_op, Rule::match_op
            ],
            negatives: vec![],
            pos: 6
        }
    }
}
