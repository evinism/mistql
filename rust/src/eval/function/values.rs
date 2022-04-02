use crate::{eval::Value, Error, Result};

pub fn values(args: Vec<Value>) -> Result<Value> {
    match (args.len(), args.get(0)) {
        (1, Some(Value::Object(val))) => {
            Ok(Value::Array(val.values().cloned().collect::<Vec<Value>>()))
        }
        (1, Some(val)) => Err(Error::eval(format!(
            "argument to values must be an object (got {:?}",
            val
        ))),
        (n, _) => Err(Error::eval(format!(
            "values expected 1 argument, got {}",
            n
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::values;
    use crate::eval::Value;
    use crate::{MistQLParser, Rule};
    use std::collections::BTreeMap;

    #[test]
    fn values_takes_one_arg() {
        assert!(values(vec![]).is_err());
        assert!(values(vec![Value::Object(BTreeMap::new())]).is_ok());
        assert!(values(vec![
            Value::Object(BTreeMap::new()),
            Value::Object(BTreeMap::new())
        ])
        .is_err());
    }

    #[test]
    fn values_arg_must_be_an_object() {
        assert!(values(vec![Value::Int(1)]).is_err());
        assert!(values(vec![Value::String("abc".to_string())]).is_err());
    }

    #[test]
    fn values_returns_values() {
        let query = "values {a: 1, b: 2}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,19, [
                    ident(0,6),
                    object(7,19, [
                        keyval(8,12, [
                            ident(8,9),
                            number(11,12)
                        ]),
                        keyval(14,18, [
                            ident(14,15),
                            number(17,18)
                        ])
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Array(vec![1.into(), 2.into(),]))
    }

    #[test]
    fn keys_returns_keys_when_piped() {
        let query = "{a: 1, b: 2} | values";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                piped_expr(0,21, [
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
                    ident(15,21)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Array(vec![1.into(), 2.into(),]))
    }
}
