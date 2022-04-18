use crate::{expr, Error, Number, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn count(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("count requires one argument".to_string())),
    };

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
