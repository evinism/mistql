use super::args::ArgParser;
use crate::{Error, Number, Result, Value};

pub fn count(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?;

    match arg {
        Value::Array(arr) => Ok(Value::Number(Number::Int(arr.len() as i64))),
        _ => Err(Error::eval(format!(
            "argument to count must be an array (got {:?}",
            arg
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn count_takes_one_arg() {
        assert!(query_value("count [1,2,3]".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("count [1,2,3] [4,5,6]".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn count_arg_must_be_an_array() {
        assert!(query_value("count 1".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("count \"abc\"".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn count_counts() {
        assert_eq!(
            query_value("count []".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(0)
        );
        assert_eq!(
            query_value("count [1,2,3]".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(3)
        );
    }
}
