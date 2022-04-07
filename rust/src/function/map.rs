use crate::{expr, Value};
use crate::{Error, Result, Rule};
use pest::iterators::Pairs;
use std::collections::BTreeMap;

pub fn map(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let args = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "map requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (Value::Array(val), func) => Ok(Value::Array(
            val.iter()
                .map(|elt| expr::eval(func.clone(), elt, None))
                .collect::<Result<Vec<Value>>>()?,
        )),
        (val, _) => Err(Error::eval(format!(
            "argument to map must be an array (got {:?}",
            val
        ))),
    }
}

pub fn mapkeys(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let args = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "mapkeys requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (Value::Object(val), func) => {
            // it's rather alarming how much cleaner mutation is than iterator chains when dealing
            // with results inside of Maps
            let mut mapped: BTreeMap<String, Value> = BTreeMap::new();
            for (k, v) in val.iter() {
                let map_k = expr::eval(func.clone(), &Value::String(k.clone()), None)?.to_string();
                mapped.insert(map_k, v.clone());
            }
            Ok(Value::Object(mapped))
        }
        (val, _) => Err(Error::eval(format!(
            "argument to mapkeys must be an object (got {:?}",
            val
        ))),
    }
}

pub fn mapvalues(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let args = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "mapvalues requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (Value::Object(val), func) => {
            let mut mapped: BTreeMap<String, Value> = BTreeMap::new();
            for (k, v) in val.iter() {
                let map_v = expr::eval(func.clone(), &v, None)?;
                mapped.insert(k.clone(), map_v);
            }
            Ok(Value::Object(mapped))
        }
        (val, _) => Err(Error::eval(format!(
            "argument to mapvalues must be an object (got {:?}",
            val
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{query_value, MistQLParser, Rule, Value};

    #[test]
    fn map_parses() {
        let query = "map @ + 1 [1, 2, 3]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,19, [
                    ident(0,3),
                    infix_expr(4,10, [
                        at(4,5),
                        plus_op(6,7),
                        number(8,9)
                    ]),
                    array(10,19, [
                        number(11,12),
                        number(14,15),
                        number(17,18)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                (2 as i64).into(),
                (3 as i64).into(),
                (4 as i64).into()
            ])
        )
    }

    #[test]
    fn map_parses_with_a_function() {
        let query = "map (string @) [1, 2, 3]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,24,[
                    ident(0,3),
                    function(5,13,[
                        ident(5,11),
                        at(12,13)
                    ]),
                    array(15,24, [
                        number(16,17),
                        number(19,20),
                        number(22,23)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec!["1".into(), "2".into(), "3".into()])
        )
    }

    #[test]
    fn map_takes_one_arg() {
        assert!(query_value("map @ [1,2,3]".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("map @".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value(
            "map @ [1,2,3], [4,5,6]".to_string(),
            serde_json::Value::Null
        )
        .is_err());
    }

    #[test]
    fn map_arg_must_be_an_array() {
        assert!(query_value("map @ 123".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("map @ \"abc\"".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn mapvalues_parses() {
        let query = "mapvalues @ + 1 {a: 1, b: 2, c: 3}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,34,[
                    ident(0,9),
                    infix_expr(10,16,[
                        at(10,11),
                        plus_op(12,13),
                        number(14,15)
                    ]),
                    object(16,34,[
                        keyval(17,21,[
                            ident(17,18),
                            number(20,21)
                        ]),
                        keyval(23,27,[
                            ident(23,24),
                            number(26,27)
                        ]),
                        keyval(29,33,[
                            ident(29,30),
                            number(32,33)
                        ])

                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        let mut map = std::collections::BTreeMap::new();
        map.insert("a".to_string(), Value::Int(2));
        map.insert("b".to_string(), Value::Int(3));
        map.insert("c".to_string(), Value::Int(4));
        let expected: serde_json::Value = Value::Object(map).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn mapvalues_takes_one_arg() {
        assert!(query_value(
            "mapvalues @ {a: 1, b: 2, c: 3}".to_string(),
            serde_json::Value::Null
        )
        .is_ok());
        assert!(query_value("mapvalues @".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value(
            "mapvalues @ {a: 1, b: 2, c: 3}, {a: 4, b: 5, c: 6}".to_string(),
            serde_json::Value::Null
        )
        .is_err());
    }

    #[test]
    fn mapvalues_arg_must_be_an_object() {
        assert!(query_value("mapvalues @ 123".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("mapvalues @ \"abc\"".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn mapkeys_parses() {
        let query = "mapkeys (string @) {a: 1, b: 2, c: 3}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,37,[
                    ident(0,7),
                    function(9,17,[
                        ident(9,15),
                        at(16,17)
                    ]),
                    object(19,37,[
                        keyval(20,24,[
                            ident(20,21),
                            number(23,24)
                        ]),
                        keyval(26,30,[
                            ident(26,27),
                            number(29,30)
                        ]),
                        keyval(32,36,[
                            ident(32,33),
                            number(35,36)
                        ])

                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        let mut map = std::collections::BTreeMap::new();
        map.insert("a".to_string(), Value::Int(1));
        map.insert("b".to_string(), Value::Int(2));
        map.insert("c".to_string(), Value::Int(3));
        let expected: serde_json::Value = Value::Object(map).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn mapkeys_takes_one_arg() {
        assert!(query_value(
            "mapkeys @ {a: 1, b: 2, c: 3}".to_string(),
            serde_json::Value::Null
        )
        .is_ok());
        assert!(query_value("mapkeys @".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value(
            "mapkeys @ {a: 1, b: 2, c: 3}, {a: 4, b: 5, c: 6}".to_string(),
            serde_json::Value::Null
        )
        .is_err());
    }

    #[test]
    fn mapkeys_arg_must_be_an_object() {
        assert!(query_value("mapkeys @ 123".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("mapkeys @ \"abc\"".to_string(), serde_json::Value::Null).is_err());
    }
}
