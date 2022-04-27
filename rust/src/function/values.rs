use super::args::ArgParser;
use crate::{Error, Result, Value};

pub fn values(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?;

    match arg {
        Value::Object(val) => Ok(Value::Array(val.values().cloned().collect::<Vec<Value>>())),
        _ => Err(Error::eval(format!("values expected object, got {}", arg))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{query_value, MistQLParser, Rule};

    #[test]
    fn values_takes_one_arg() {
        assert!(query_value("values {}".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("values {} 4".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn values_arg_must_be_an_object() {
        assert!(query_value("values null".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("values [1,2,3]".to_string(), serde_json::Value::Null).is_err());
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
