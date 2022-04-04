use crate::{Error, Result, Value};

pub fn keys(args: Vec<Value>) -> Result<Value> {
    match (args.len(), args.get(0)) {
        (1, Some(Value::Object(val))) => Ok(Value::Array(
            val.keys()
                .map(|k| Value::String(k.to_string()))
                .collect::<Vec<Value>>(),
        )),
        (1, Some(val)) => Err(Error::eval(format!(
            "argument to keys must be an object (got {:?}",
            val
        ))),
        (n, _) => Err(Error::eval(format!("keys expected 1 argument, got {}", n))),
    }
}

#[cfg(test)]
mod tests {
    use super::keys;
    use crate::Value;
    use crate::{MistQLParser, Rule};
    use std::collections::BTreeMap;

    #[test]
    fn keys_takes_one_arg() {
        assert!(keys(vec![]).is_err());
        assert!(keys(vec![Value::Object(BTreeMap::new())]).is_ok());
        assert!(keys(vec![
            Value::Object(BTreeMap::new()),
            Value::Object(BTreeMap::new())
        ])
        .is_err());
    }

    #[test]
    fn keys_arg_must_be_an_object() {
        assert!(keys(vec![Value::Int(1)]).is_err());
        assert!(keys(vec![Value::String("abc".to_string())]).is_err());
    }

    #[test]
    fn keys_returns_keys() {
        let query = "keys {a: 1, b: 2}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,17, [
                    ident(0,4),
                    object(5,17, [
                        keyval(6,10, [
                            ident(6,7),
                            number(9,10)
                        ]),
                        keyval(12,16, [
                            ident(12,13),
                            number(15,16)
                        ])
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec!["a".into(), "b".into(),])
        )
    }

    #[test]
    fn keys_returns_keys_when_piped() {
        let query = "{a: 1, b: 2} | keys";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                piped_expr(0,19, [
                    object(0,12, [
                        keyval(1,5, [
                            ident(1,2),
                            number(4,5)
                        ]),
                        keyval(7,11, [
                            ident(7,8),
                            number(10,11)
                        ])
                    ]),
                    ident(15,19)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec!["a".into(), "b".into(),])
        )
    }
}
