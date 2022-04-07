use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn entries(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("entries requires one argument".to_string())),
    };
    match arg {
        Value::Object(val) => Ok(Value::Array(
            val.iter()
                .map(|(k, v)| Value::Array(vec![Value::String(k.clone()), v.clone()]))
                .collect::<Vec<Value>>(),
        )),
        _ => Err(Error::eval(format!("entries expected object, got {}", arg))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{query_value, MistQLParser, Rule};

    #[test]
    fn entries_takes_one_arg() {
        assert!(query_value("entries {}".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("entries {} 4".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn entries_arg_must_be_an_object() {
        assert!(query_value("entries null".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("entries [1,2,3]".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn entries_returns_entries() {
        let query = "entries {a: 1, b: 2}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,20, [
                    ident(0,7),
                    object(8,20, [
                        keyval(9,13, [
                            ident(9,10),
                            number(12,13)
                        ]),
                        keyval(15,19, [
                            ident(15,16),
                            number(18,19)
                        ])
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                serde_json::Value::Array(vec!["a".into(), serde_json::Value::from(1)]),
                serde_json::Value::Array(vec!["b".into(), serde_json::Value::from(2)])
            ])
        )
    }

    #[test]
    fn entries_returns_entries_when_piped() {
        let query = "{a: 1, b: 2} | entries";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                piped_expr(0,22, [
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
                    ident(15,22)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                serde_json::Value::Array(vec!["a".into(), serde_json::Value::from(1)]),
                serde_json::Value::Array(vec!["b".into(), serde_json::Value::from(2)])
            ])
        )
    }
}
