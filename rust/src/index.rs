use crate::{expr, terminal, Error, Result, Rule, Value};
use pest::iterators::Pair;

pub fn eval(pair: Pair<Rule>, data: &Value) -> Result<Value> {
    // indexed_value must contain a target and an index
    let mut itr = pair.into_inner();
    let target = expr::eval(itr.next().unwrap(), data, None)?;
    let index_val = itr.next().unwrap();
    match index_val.as_rule() {
        Rule::number => item_index(&terminal::eval(index_val)?, &target),
        Rule::low_range => range_index(
            &terminal::eval(index_val.into_inner().next().unwrap())?,
            &Value::Null,
            &target,
        ),
        Rule::high_range => range_index(
            &Value::Null,
            &terminal::eval(index_val.into_inner().next().unwrap())?,
            &target,
        ),
        Rule::range => {
            let mut index_itr = index_val.into_inner();
            range_index(
                &terminal::eval(index_itr.next().unwrap())?,
                &terminal::eval(index_itr.next().unwrap())?,
                &target,
            )
        }
        _ => unreachable!(),
    }
}

pub fn item_index(idx: &Value, target: &Value) -> Result<Value> {
    match target {
        Value::Null => index_null(idx),
        Value::String(s) => index_string(s, idx),
        Value::Array(a) => index_array(a, idx),
        Value::Object(o) => index_object(o, idx),
        _ => Err(Error::query(format!(
            "index on unindexable type {:?}",
            target
        ))),
    }
}

pub fn range_index(low: &Value, high: &Value, target: &Value) -> Result<Value> {
    match target {
        Value::String(s) => range_index_string(s, low, high),
        Value::Array(a) => range_index_array(a, low, high),
        _ => Err(Error::query(format!(
            "range index on unindexable type {:?}",
            target
        ))),
    }
}

fn index_null(idx: &Value) -> Result<Value> {
    match idx {
        Value::Int(_) | Value::Float(_) | Value::String(_) => Ok(Value::Null),
        _ => Err(Error::query(
            "index must be a string or a number".to_string(),
        )),
    }
}

fn index_string(val: &str, idx_raw: &Value) -> Result<Value> {
    if let Value::Int(idx) = idx_raw {
        if *idx >= 0 {
            match val.chars().nth(*idx as usize) {
                Some(c) => Ok(Value::String(c.to_string())),
                None => Ok(Value::Null),
            }
        } else {
            match val.chars().rev().nth((-1 * idx - 1) as usize) {
                Some(c) => Ok(Value::String(c.to_string())),
                None => Ok(Value::Null),
            }
        }
    } else {
        Err(Error::query("string index must be an integer".to_string()))
    }
}

fn index_array(val: &Vec<Value>, idx_raw: &Value) -> Result<Value> {
    if let Value::Int(idx) = idx_raw {
        if *idx >= 0 {
            match val.iter().nth(*idx as usize) {
                Some(elt) => Ok(elt.clone()),
                None => Ok(Value::Null),
            }
        } else {
            match val.iter().rev().nth((-1 * *idx - 1) as usize) {
                Some(elt) => Ok(elt.clone()),
                None => Ok(Value::Null),
            }
        }
    } else {
        Err(Error::query("array index must be an integer".to_string()))
    }
}

fn normalize_range(raw_low: &Value, raw_high: &Value, length: i64) -> Result<(usize, usize)> {
    match (raw_low, raw_high) {
        (Value::Int(low_num), Value::Int(high_num)) => {
            let low = if *low_num < 0 {
                length + low_num
            } else {
                *low_num
            };

            let high = if *high_num < 0 {
                length + high_num
            } else {
                *high_num
            };

            if high < 0 || low < 0 || high < low {
                Err(Error::query(
                    "indexes must be integers that specify a range".to_string(),
                ))
            } else {
                Ok((low as usize, high as usize))
            }
        }
        (Value::Null, Value::Int(high_num)) => {
            let high = if *high_num < 0 {
                length + high_num
            } else {
                *high_num
            };

            if high < 0 {
                Err(Error::query(
                    "indexes must be integers that specify a range".to_string(),
                ))
            } else {
                Ok((0, high as usize))
            }
        }
        (Value::Int(low_num), Value::Null) => {
            let low = if *low_num < 0 {
                length + low_num
            } else {
                *low_num
            };

            if low < 0 {
                Err(Error::query(
                    "indexes must be integers that specify a range".to_string(),
                ))
            } else {
                Ok((low as usize, length as usize))
            }
        }
        _ => Err(Error::query(
            "indexes must be integers that specify a range".to_string(),
        )),
    }
}

fn range_index_string(val: &str, idx_low_raw: &Value, idx_high_raw: &Value) -> Result<Value> {
    let (low, high) = normalize_range(idx_low_raw, idx_high_raw, val.len() as i64)?;

    if low >= high || low > val.len() {
        Ok(Value::Null)
    } else {
        Ok(Value::String(
            val.chars().skip(low).take(high - low).collect(),
        ))
    }
}

fn range_index_array(val: &Vec<Value>, idx_low_raw: &Value, idx_high_raw: &Value) -> Result<Value> {
    let (low, high) = normalize_range(idx_low_raw, idx_high_raw, val.len() as i64)?;

    if low >= high || low > val.len() {
        Ok(Value::Null)
    } else {
        Ok(Value::Array(
            val.iter()
                .skip(low)
                .take(high - low)
                .map(|elt| elt.clone())
                .collect(),
        ))
    }
}

fn index_object(val: &std::collections::BTreeMap<String, Value>, idx_raw: &Value) -> Result<Value> {
    if let Value::String(idx) = idx_raw {
        match val.get(idx) {
            Some(elt) => Ok(elt.clone()),
            None => Ok(Value::Null),
        }
    } else {
        Err(Error::query("object index must be a string".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::{item_index, range_index};
    use crate::{MistQLParser, Rule, Value};

    #[test]
    fn cant_index_bool_or_number() {
        assert!(item_index(&Value::Int(1), &Value::Int(1)).is_err());
        assert!(item_index(&Value::Boolean(true), &Value::Int(1)).is_err());
    }

    #[test]
    fn indexes_on_null() {
        assert_eq!(
            item_index(&Value::Int(0), &Value::Null).unwrap(),
            Value::Null
        );

        assert_eq!(
            item_index(&Value::String("a".to_string()), &Value::Null).unwrap(),
            Value::Null
        );

        assert!(item_index(&Value::Null, &Value::Boolean(true)).is_err())
    }

    #[test]
    fn index_must_be_an_int() {
        assert!(item_index(&Value::Float(4.5), &Value::String("abc".to_string())).is_err());

        assert!(range_index(
            &Value::Int(4),
            &Value::Float(7.5),
            &Value::String("abc".to_string())
        )
        .is_err())
    }

    #[test]
    fn indexes_on_postive_ints_on_strings() {
        assert_eq!(
            item_index(&Value::Int(0), &Value::String("abc".to_string())).unwrap(),
            Value::String("a".to_string())
        );

        assert_eq!(
            item_index(&Value::Int(1), &Value::String("abc".to_string())).unwrap(),
            Value::String("b".to_string())
        );

        assert_eq!(
            item_index(&Value::Int(4), &Value::String("abc".to_string())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn indexes_on_negative_ints_on_strings() {
        assert_eq!(
            item_index(&Value::Int(-1), &Value::String("abc".to_string())).unwrap(),
            Value::String("c".to_string())
        );

        assert_eq!(
            item_index(&Value::Int(-2), &Value::String("abc".to_string())).unwrap(),
            Value::String("b".to_string())
        );

        assert_eq!(
            item_index(&Value::Int(-4), &Value::String("abc".to_string())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn range_indexes_on_strings() {
        assert_eq!(
            range_index(
                &Value::Int(0),
                &Value::Int(2),
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::String("ab".to_string())
        );

        assert_eq!(
            range_index(
                &Value::Int(2),
                &Value::Int(4),
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::String("cd".to_string())
        );

        assert_eq!(
            range_index(
                &Value::Int(4),
                &Value::Int(7),
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::String("ef".to_string())
        );

        assert_eq!(
            range_index(
                &Value::Int(7),
                &Value::Int(12),
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::Null
        );

        assert_eq!(
            range_index(
                &Value::Int(2),
                &Value::Int(-2),
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::String("cd".to_string())
        );

        // can't specify a range where the low is greater than the high
        assert!(range_index(
            &Value::Int(4),
            &Value::Int(2),
            &Value::String("abcdef".to_string()),
        )
        .is_err());

        // can't specify negative ranges greater than the length of the string
        assert!(range_index(
            &Value::Int(4),
            &Value::Int(-10),
            &Value::String("abcdef".to_string()),
        )
        .is_err());

        assert!(range_index(
            &Value::Int(-10),
            &Value::Int(4),
            &Value::String("abcdef".to_string()),
        )
        .is_err());
    }

    #[test]
    fn range_indexes_with_nulls_on_strings() {
        assert_eq!(
            range_index(
                &Value::Int(2),
                &Value::Null,
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::String("cdef".to_string())
        );

        assert_eq!(
            range_index(
                &Value::Int(-2),
                &Value::Null,
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::String("ef".to_string())
        );

        assert_eq!(
            range_index(
                &Value::Int(7),
                &Value::Null,
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::Null
        );

        assert_eq!(
            range_index(
                &Value::Null,
                &Value::Int(4),
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::String("abcd".to_string())
        );

        assert_eq!(
            range_index(
                &Value::Null,
                &Value::Int(-2),
                &Value::String("abcdef".to_string()),
            )
            .unwrap(),
            Value::String("abcd".to_string())
        );

        assert!(range_index(
            &Value::Null,
            &Value::Null,
            &Value::String("abcdef".to_string()),
        )
        .is_err());
    }

    #[test]
    fn indexes_on_postive_ints_on_arrays() {
        let array = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(item_index(&Value::Int(0), &array).unwrap(), Value::Int(1),);

        assert_eq!(item_index(&Value::Int(1), &array).unwrap(), Value::Int(2));

        assert_eq!(item_index(&Value::Int(4), &array).unwrap(), Value::Null);
    }

    #[test]
    fn indexes_on_negative_ints_on_arrays() {
        let array = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);

        assert_eq!(item_index(&Value::Int(-1), &array).unwrap(), Value::Int(3));

        assert_eq!(item_index(&Value::Int(-2), &array).unwrap(), Value::Int(2));

        assert_eq!(item_index(&Value::Int(-4), &array).unwrap(), Value::Null);
    }

    #[test]
    fn range_indexes_on_arrays() {
        let array = Value::Array(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
            Value::Int(4),
            Value::Int(5),
            Value::Int(6),
        ]);

        assert_eq!(
            range_index(&Value::Int(0), &Value::Int(2), &array).unwrap(),
            Value::Array(vec![Value::Int(1), Value::Int(2),])
        );

        assert_eq!(
            range_index(&Value::Int(2), &Value::Int(4), &array).unwrap(),
            Value::Array(vec![Value::Int(3), Value::Int(4),])
        );

        assert_eq!(
            range_index(&Value::Int(4), &Value::Int(7), &array).unwrap(),
            Value::Array(vec![Value::Int(5), Value::Int(6),])
        );

        assert_eq!(
            range_index(&Value::Int(7), &Value::Int(12), &array).unwrap(),
            Value::Null
        );

        assert_eq!(
            range_index(&Value::Int(2), &Value::Int(-2), &array).unwrap(),
            Value::Array(vec![Value::Int(3), Value::Int(4),])
        );

        // can't specify a range where the low is greater than the high
        assert!(range_index(&Value::Int(4), &Value::Int(2), &array).is_err());

        // can't specify negative ranges greater than the length of the string
        assert!(range_index(&Value::Int(4), &Value::Int(-10), &array).is_err());

        assert!(range_index(&Value::Int(-10), &Value::Int(4), &array).is_err());
    }

    #[test]
    fn range_indexes_with_nulls_on_arrays() {
        let array = Value::Array(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
            Value::Int(4),
            Value::Int(5),
            Value::Int(6),
        ]);

        assert_eq!(
            range_index(&Value::Int(2), &Value::Null, &array).unwrap(),
            Value::Array(vec![
                Value::Int(3),
                Value::Int(4),
                Value::Int(5),
                Value::Int(6),
            ])
        );

        assert_eq!(
            range_index(&Value::Int(-2), &Value::Null, &array).unwrap(),
            Value::Array(vec![Value::Int(5), Value::Int(6)])
        );

        assert_eq!(
            range_index(&Value::Int(7), &Value::Null, &array).unwrap(),
            Value::Null
        );

        assert_eq!(
            range_index(&Value::Null, &Value::Int(4), &array).unwrap(),
            Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
            ])
        );

        assert_eq!(
            range_index(&Value::Null, &Value::Int(-2), &array).unwrap(),
            Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
            ])
        );

        assert!(range_index(&Value::Null, &Value::Null, &array).is_err());
    }

    #[test]
    fn indexes_strings_on_objects() {
        let mut map = std::collections::BTreeMap::new();
        map.insert("a".to_string(), Value::Int(1));
        map.insert("b".to_string(), Value::Int(2));
        map.insert("c".to_string(), Value::Int(3));
        let val = Value::Object(map);

        assert_eq!(
            item_index(&Value::String("a".to_string()), &val).unwrap(),
            Value::Int(1)
        );

        assert_eq!(
            item_index(&Value::String("b".to_string()), &val).unwrap(),
            Value::Int(2)
        );

        assert_eq!(
            item_index(&Value::String("m".to_string()), &val).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn indexed_value_behaves_like_index_function() {
        let result = crate::query(
            "(@[-1:]) == (index (-1) null @)".to_string(),
            "[1,2,3,4]".to_string(),
        )
        .unwrap();
        assert_eq!(result, serde_json::Value::Bool(true));

        let result = crate::query(
            "(@[:-2]) == (index null (-2) @)".to_string(),
            "[1,2,3,4]".to_string(),
        )
        .unwrap();
        assert_eq!(result, serde_json::Value::Bool(true));
    }

    #[test]
    fn parses_indexed_value() {
        let query = "@[1]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                indexed_value(0,4, [
                    at(0,1),
                    number(2,3)
                ])
            ]
        }
    }

    #[test]
    fn parses_negative_indexed_value() {
        parses_to! {
            parser: MistQLParser,
            input: "@[-1]",
            rule: Rule::query,
            tokens: [
                indexed_value(0,5, [
                    at(0,1),
                    number(2,4)
                ])
            ]
        }
    }

    #[test]
    fn parses_range_index() {
        parses_to! {
            parser: MistQLParser,
            input: "@[1:4]",
            rule: Rule::query,
            tokens: [
                indexed_value(0,6, [
                    at(0,1),
                    range(2,5, [
                        number(2,3),
                        number(4,5)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parses_range_index_with_no_start() {
        parses_to! {
            parser: MistQLParser,
            input: "@[:4]",
            rule: Rule::query,
            tokens: [
                indexed_value(0,5, [
                    at(0,1),
                    high_range(2,4, [
                        number(3,4)
                    ])
                ])
            ]
        }

        parses_to! {
            parser: MistQLParser,
            input: "@[:-4]",
            rule: Rule::query,
            tokens: [
                indexed_value(0,6, [
                    at(0,1),
                    high_range(2,5, [
                        number(3,5)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parses_range_index_with_no_end() {
        parses_to! {
            parser: MistQLParser,
            input: "@[4:]",
            rule: Rule::query,
            tokens: [
                indexed_value(0,5, [
                    at(0,1),
                    low_range(2,4, [
                        number(2,3)
                    ])
                ])
            ]
        }

        parses_to! {
            parser: MistQLParser,
            input: "@[-4:]",
            rule: Rule::query,
            tokens: [
                indexed_value(0,6, [
                    at(0,1),
                    low_range(2,5, [
                        number(2,4)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parses_selector() {
        parses_to! {
            parser: MistQLParser,
            input: "one.two",
            rule: Rule::query,
            tokens: [
                compound_reference(0,7, [
                    ident(0,3),
                    ident(4,7)
                ])
            ]
        }
    }

    // #[test]
    // fn parses_selector_with_lhs_expr() {
    //     parses_to! {
    //         parser: MistQLParser,
    //         input: "(@ | apply @[0]).hello",
    //         rule: Rule::query,
    //         tokens: [
    //             at(1,2),
    //             infix_op(3,4),
    //             function(5,15, [
    //                 ident(5,10),
    //                 subscript(11,15 ,[
    //                     at(11,12),
    //                     number(13,14)
    //                 ])
    //             ]),
    //             infix_op(16,17),
    //             ident(17,22)
    //         ]
    //     }
    // }
}
