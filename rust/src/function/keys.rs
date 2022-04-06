use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn keys(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("keys requires one argument".to_string())),
    };
    match arg {
        Value::Object(val) => Ok(Value::Array(
            val.keys()
                .map(|k| Value::String(k.to_string()))
                .collect::<Vec<Value>>(),
        )),
        _ => Err(Error::eval(format!("keys expected object, got {}", arg))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{query_value, MistQLParser, Rule};

    #[test]
    fn keys_takes_one_arg() {
        assert!(query_value("keys {}".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("keys {} 4".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn keys_arg_must_be_an_object() {
        assert!(query_value("keys null".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("keys [1,2,3]".to_string(), serde_json::Value::Null).is_err());
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
