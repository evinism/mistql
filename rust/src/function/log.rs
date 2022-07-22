use super::args::ArgParser;
use crate::{Result, Value};

pub fn log(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?.to_value(arg_parser.data)?;
    dbg!(&arg);
    Ok(arg)
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn log_takes_one_arg() {
        assert!(query_value("log 1".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("log 1 2".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn log_uses_implicit_at() {
        assert_eq!(
            query_value("123 | log".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(123)
        );
    }

    #[test]
    fn log_uses_explicit_at() {
        assert_eq!(
            query_value("log @".to_string(), serde_json::Value::from(123)).unwrap(),
            serde_json::Value::from(123)
        );
    }
}
