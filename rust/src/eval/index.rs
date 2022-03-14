use crate::{Error, Result};

pub fn index(args: Vec<serde_json::Value>) -> Result<serde_json::Value> {
    match (args.get(0), args.get(1), args.get(2), args.get(3)) {
        (None, _, _, _) | (_, None, _, _) | (_, _, _, Some(_)) => Err(Error::eval(
            "index must have two or three arguments".to_string(),
        )),
        (Some(val), Some(idx), None, None) => match val {
            serde_json::Value::Null => index_null(idx),
            serde_json::Value::String(s) => index_string(s, idx),
            serde_json::Value::Array(a) => index_array(a, idx),
            serde_json::Value::Object(o) => index_object(o, idx),
            _ => Err(Error::query("index on unindexable type".to_string())),
        },
        (Some(val), Some(idx_low), Some(idx_high), None) => match val {
            serde_json::Value::String(s) => range_index_string(s, idx_low, idx_high),
            serde_json::Value::Array(a) => range_index_array(a, idx_low, idx_high),
            _ => Err(Error::query("range index on unindexable type".to_string())),
        },
    }
}

fn index_null(idx: &serde_json::Value) -> Result<serde_json::Value> {
    match idx {
        serde_json::Value::Number(_) | serde_json::Value::String(_) => Ok(serde_json::Value::Null),
        _ => Err(Error::query(
            "index must be a string or a number".to_string(),
        )),
    }
}

fn index_string(val: &str, idx_raw: &serde_json::Value) -> Result<serde_json::Value> {
    match idx_raw.as_i64() {
        Some(idx) => {
            if idx >= 0 {
                match val.chars().nth(idx as usize) {
                    Some(c) => Ok(c.to_string().into()),
                    None => Ok(serde_json::Value::Null),
                }
            } else {
                match val.chars().rev().nth((-1 * idx - 1) as usize) {
                    Some(c) => Ok(c.to_string().into()),
                    None => Ok(serde_json::Value::Null),
                }
            }
        }
        None => Err(Error::query("string index must be an integer".to_string())),
    }
}

fn index_array(
    val: &Vec<serde_json::Value>,
    idx_raw: &serde_json::Value,
) -> Result<serde_json::Value> {
    match idx_raw.as_i64() {
        Some(idx) => {
            if idx >= 0 {
                match val.iter().nth(idx as usize) {
                    Some(elt) => Ok(elt.clone()),
                    None => Ok(serde_json::Value::Null),
                }
            } else {
                match val.iter().rev().nth((-1 * idx - 1) as usize) {
                    Some(elt) => Ok(elt.clone()),
                    None => Ok(serde_json::Value::Null),
                }
            }
        }
        None => Err(Error::query("array index must be an integer".to_string())),
    }
}

fn normalize_range(
    raw_low: &serde_json::Value,
    raw_high: &serde_json::Value,
    length: i64,
) -> Result<(usize, usize)> {
    match (
        raw_low.as_null(),
        raw_low.as_i64(),
        raw_high.as_null(),
        raw_high.as_i64(),
    ) {
        (_, Some(low_num), _, Some(high_num)) => {
            let low = if low_num < 0 {
                length + low_num
            } else {
                low_num
            };

            let high = if high_num < 0 {
                length + high_num
            } else {
                high_num
            };

            if high < 0 || low < 0 || high < low {
                Err(Error::query(
                    "indexes must be integers that specify a range".to_string(),
                ))
            } else {
                Ok((low as usize, high as usize))
            }
        }
        (Some(_), _, _, Some(high_num)) => {
            let high = if high_num < 0 {
                length + high_num
            } else {
                high_num
            };

            if high < 0 {
                Err(Error::query(
                    "indexes must be integers that specify a range".to_string(),
                ))
            } else {
                Ok((0, high as usize))
            }
        }
        (_, Some(low_num), Some(_), _) => {
            let low = if low_num < 0 {
                length + low_num
            } else {
                low_num
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

fn range_index_string(
    val: &str,
    idx_low_raw: &serde_json::Value,
    idx_high_raw: &serde_json::Value,
) -> Result<serde_json::Value> {
    let (low, high) = normalize_range(idx_low_raw, idx_high_raw, val.len() as i64)?;

    if low >= high || low > val.len() {
        Ok(serde_json::Value::Null)
    } else {
        Ok(val
            .chars()
            .skip(low)
            .take(high - low)
            .collect::<String>()
            .into())
    }
}

fn range_index_array(
    val: &Vec<serde_json::Value>,
    idx_low_raw: &serde_json::Value,
    idx_high_raw: &serde_json::Value,
) -> Result<serde_json::Value> {
    let (low, high) = normalize_range(idx_low_raw, idx_high_raw, val.len() as i64)?;

    if low >= high || low > val.len() {
        Ok(serde_json::Value::Null)
    } else {
        Ok(val
            .iter()
            .skip(low)
            .take(high - low)
            .map(|elt| elt.clone())
            .collect::<serde_json::Value>()
            .into())
    }
}

fn index_object(
    val: &serde_json::Map<String, serde_json::Value>,
    idx_raw: &serde_json::Value,
) -> Result<serde_json::Value> {
    match idx_raw.as_str() {
        Some(idx) => match val.get(idx) {
            Some(elt) => Ok(elt.clone()),
            None => Ok(serde_json::Value::Null),
        },
        None => Err(Error::query("object index must be a string".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::index;
    use crate::{MistQLParser, Rule};

    #[test]
    fn cant_index_bool_or_number() {
        assert!(index(vec![serde_json::Value::from(1), serde_json::Value::from(1)]).is_err());
        assert!(index(vec![
            serde_json::Value::Bool(true),
            serde_json::Value::from(1)
        ])
        .is_err());
    }

    #[test]
    fn indexes_on_null() {
        assert_eq!(
            index(vec![
                serde_json::Value::Null,
                serde_json::Value::from(0 as i32)
            ])
            .unwrap(),
            serde_json::Value::Null
        );

        assert_eq!(
            index(vec![
                serde_json::Value::Null,
                serde_json::Value::String("a".to_string())
            ])
            .unwrap(),
            serde_json::Value::Null
        );

        assert!(index(vec![serde_json::Value::Null, serde_json::Value::Bool(true)]).is_err())
    }

    #[test]
    fn index_must_be_an_int() {
        assert!(index(vec![
            serde_json::Value::String("abc".to_string()),
            serde_json::Value::from(4.5)
        ])
        .is_err());

        assert!(index(vec![
            serde_json::Value::String("abc".to_string()),
            serde_json::Value::from(4),
            serde_json::Value::from(7.5)
        ])
        .is_err())
    }

    #[test]
    fn indexes_on_postive_ints_on_strings() {
        assert_eq!(
            index(vec![
                serde_json::Value::String("abc".to_string()),
                serde_json::Value::from(0 as i32)
            ])
            .unwrap(),
            serde_json::Value::String("a".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abc".to_string()),
                serde_json::Value::from(1 as i32)
            ])
            .unwrap(),
            serde_json::Value::String("b".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abc".to_string()),
                serde_json::Value::from(4 as i32)
            ])
            .unwrap(),
            serde_json::Value::Null
        );
    }

    #[test]
    fn indexes_on_negative_ints_on_strings() {
        assert_eq!(
            index(vec![
                serde_json::Value::String("abc".to_string()),
                serde_json::Value::from(-1 as i32)
            ])
            .unwrap(),
            serde_json::Value::String("c".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abc".to_string()),
                serde_json::Value::from(-2 as i32)
            ])
            .unwrap(),
            serde_json::Value::String("b".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abc".to_string()),
                serde_json::Value::from(-4 as i32)
            ])
            .unwrap(),
            serde_json::Value::Null
        );
    }

    #[test]
    fn range_indexes_on_strings() {
        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::from(0 as i32),
                serde_json::Value::from(2 as i32),
            ])
            .unwrap(),
            serde_json::Value::String("ab".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::from(2 as i32),
                serde_json::Value::from(4 as i32),
            ])
            .unwrap(),
            serde_json::Value::String("cd".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::from(4 as i32),
                serde_json::Value::from(7 as i32),
            ])
            .unwrap(),
            serde_json::Value::String("ef".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::from(7 as i32),
                serde_json::Value::from(12 as i32),
            ])
            .unwrap(),
            serde_json::Value::Null
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::from(2 as i32),
                serde_json::Value::from(-2 as i32),
            ])
            .unwrap(),
            serde_json::Value::String("cd".to_string())
        );

        // can't specify a range where the low is greater than the high
        assert!(index(vec![
            serde_json::Value::String("abcdef".to_string()),
            serde_json::Value::from(4 as i32),
            serde_json::Value::from(2 as i32),
        ])
        .is_err());

        // can't specify negative ranges greater than the length of the string
        assert!(index(vec![
            serde_json::Value::String("abcdef".to_string()),
            serde_json::Value::from(4 as i32),
            serde_json::Value::from(-10 as i32),
        ])
        .is_err());

        assert!(index(vec![
            serde_json::Value::String("abcdef".to_string()),
            serde_json::Value::from(-10 as i32),
            serde_json::Value::from(4 as i32),
        ])
        .is_err());
    }

    #[test]
    fn range_indexes_with_nulls_on_strings() {
        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::from(2 as i32),
                serde_json::Value::Null,
            ])
            .unwrap(),
            serde_json::Value::String("cdef".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::from(-2 as i32),
                serde_json::Value::Null,
            ])
            .unwrap(),
            serde_json::Value::String("ef".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::from(7 as i32),
                serde_json::Value::Null,
            ])
            .unwrap(),
            serde_json::Value::Null
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::Null,
                serde_json::Value::from(4 as i32),
            ])
            .unwrap(),
            serde_json::Value::String("abcd".to_string())
        );

        assert_eq!(
            index(vec![
                serde_json::Value::String("abcdef".to_string()),
                serde_json::Value::Null,
                serde_json::Value::from(-2 as i32),
            ])
            .unwrap(),
            serde_json::Value::String("abcd".to_string())
        );

        assert!(index(vec![
            serde_json::Value::String("abcdef".to_string()),
            serde_json::Value::Null,
            serde_json::Value::Null,
        ])
        .is_err());
    }

    #[test]
    fn indexes_on_postive_ints_on_arrays() {
        assert_eq!(
            index(vec![
                serde_json::Value::Array(vec![
                    (1 as u32).into(),
                    (2 as u32).into(),
                    (3 as u32).into()
                ]),
                serde_json::Value::from(0 as i32)
            ])
            .unwrap(),
            serde_json::Value::from(1)
        );

        assert_eq!(
            index(vec![
                serde_json::Value::Array(vec![
                    (1 as u32).into(),
                    (2 as u32).into(),
                    (3 as u32).into()
                ]),
                serde_json::Value::from(1 as i32)
            ])
            .unwrap(),
            serde_json::Value::from(2)
        );

        assert_eq!(
            index(vec![
                serde_json::Value::Array(vec![
                    (1 as u32).into(),
                    (2 as u32).into(),
                    (3 as u32).into()
                ]),
                serde_json::Value::from(4 as i32)
            ])
            .unwrap(),
            serde_json::Value::Null
        );
    }

    #[test]
    fn indexes_on_negative_ints_on_arrays() {
        assert_eq!(
            index(vec![
                serde_json::Value::Array(vec![
                    (1 as u32).into(),
                    (2 as u32).into(),
                    (3 as u32).into()
                ]),
                serde_json::Value::from(-1 as i32)
            ])
            .unwrap(),
            serde_json::Value::from(3)
        );

        assert_eq!(
            index(vec![
                serde_json::Value::Array(vec![
                    (1 as u32).into(),
                    (2 as u32).into(),
                    (3 as u32).into()
                ]),
                serde_json::Value::from(-2 as i32)
            ])
            .unwrap(),
            serde_json::Value::from(2)
        );

        assert_eq!(
            index(vec![
                serde_json::Value::Array(vec![
                    (1 as u32).into(),
                    (2 as u32).into(),
                    (3 as u32).into()
                ]),
                serde_json::Value::from(-4 as i32)
            ])
            .unwrap(),
            serde_json::Value::Null
        );
    }

    #[test]
    fn range_indexes_on_arrays() {
        let array = serde_json::Value::Array(vec![
            (1 as u32).into(),
            (2 as u32).into(),
            (3 as u32).into(),
            (4 as u32).into(),
            (5 as u32).into(),
            (6 as u32).into(),
        ]);

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::from(0 as i32),
                serde_json::Value::from(2 as i32)
            ])
            .unwrap(),
            serde_json::Value::Array(vec![(1 as u32).into(), (2 as u32).into(),])
        );

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::from(2 as i32),
                serde_json::Value::from(4 as i32),
            ])
            .unwrap(),
            serde_json::Value::Array(vec![(3 as u32).into(), (4 as u32).into(),])
        );

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::from(4 as i32),
                serde_json::Value::from(7 as i32),
            ])
            .unwrap(),
            serde_json::Value::Array(vec![(5 as u32).into(), (6 as u32).into()])
        );

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::from(7 as i32),
                serde_json::Value::from(12 as i32),
            ])
            .unwrap(),
            serde_json::Value::Null
        );

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::from(2 as i32),
                serde_json::Value::from(-2 as i32),
            ])
            .unwrap(),
            serde_json::Value::Array(vec![(3 as u32).into(), (4 as u32).into(),])
        );

        // can't specify a range where the low is greater than the high
        assert!(index(vec![
            array.clone(),
            serde_json::Value::from(4 as i32),
            serde_json::Value::from(2 as i32),
        ])
        .is_err());

        // can't specify negative ranges greater than the length of the string
        assert!(index(vec![
            array.clone(),
            serde_json::Value::from(4 as i32),
            serde_json::Value::from(-10 as i32),
        ])
        .is_err());

        assert!(index(vec![
            array,
            serde_json::Value::from(-10 as i32),
            serde_json::Value::from(4 as i32),
        ])
        .is_err());
    }

    #[test]
    fn range_indexes_with_nulls_on_arrays() {
        let array = serde_json::Value::Array(vec![
            (1 as u32).into(),
            (2 as u32).into(),
            (3 as u32).into(),
            (4 as u32).into(),
            (5 as u32).into(),
            (6 as u32).into(),
        ]);

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::from(2 as i32),
                serde_json::Value::Null,
            ])
            .unwrap(),
            serde_json::Value::Array(vec![
                (3 as u32).into(),
                (4 as u32).into(),
                (5 as u32).into(),
                (6 as u32).into(),
            ])
        );

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::from(-2 as i32),
                serde_json::Value::Null,
            ])
            .unwrap(),
            serde_json::Value::Array(vec![(5 as u32).into(), (6 as u32).into(),])
        );

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::from(7 as i32),
                serde_json::Value::Null,
            ])
            .unwrap(),
            serde_json::Value::Null
        );

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::Null,
                serde_json::Value::from(4 as i32),
            ])
            .unwrap(),
            serde_json::Value::Array(vec![
                (1 as u32).into(),
                (2 as u32).into(),
                (3 as u32).into(),
                (4 as u32).into(),
            ])
        );

        assert_eq!(
            index(vec![
                array.clone(),
                serde_json::Value::Null,
                serde_json::Value::from(-2 as i32),
            ])
            .unwrap(),
            serde_json::Value::Array(vec![
                (1 as u32).into(),
                (2 as u32).into(),
                (3 as u32).into(),
                (4 as u32).into(),
            ])
        );

        assert!(index(vec![
            array,
            serde_json::Value::Null,
            serde_json::Value::Null,
        ])
        .is_err());
    }

    #[test]
    fn indexes_strings_on_objects() {
        let mut map = serde_json::Map::new();
        map.insert("a".to_string(), (1 as u32).into());
        map.insert("b".to_string(), (2 as u32).into());
        map.insert("c".to_string(), (3 as u32).into());
        let val = serde_json::Value::Object(map);

        assert_eq!(
            index(vec![
                val.clone(),
                serde_json::Value::String("a".to_string())
            ])
            .unwrap(),
            serde_json::Value::from(1)
        );

        assert_eq!(
            index(vec![
                val.clone(),
                serde_json::Value::String("b".to_string())
            ])
            .unwrap(),
            serde_json::Value::from(2)
        );

        assert_eq!(
            index(vec![val, serde_json::Value::String("m".to_string())]).unwrap(),
            serde_json::Value::Null
        );
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
                    low_range(2,4, [
                        number(3,4)
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
                    high_range(2,4, [
                        number(2,3)
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
    // fn parses_deep_selector() {
    //     parses_to! {
    //         parser: MistQLParser,
    //         input: "one.two.three.four",
    //         rule: Rule::query,
    //         tokens: [
    //             compound_reference(0,18, [
    //                 ident(0,3),
    //                 ident(4,7),
    //                 ident(8,13),
    //                 ident(14,18)
    //             ])
    //         ]
    //     }
    // }

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
