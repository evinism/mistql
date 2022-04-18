use crate::{expr, Error, Number, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn float(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("float requires one argument".to_string())),
    };

    match arg {
        Value::Number(Number::Float(_)) => Ok(arg.clone()),
        Value::Number(Number::Int(num)) => Ok(Value::Number(Number::Float(num as f64))),
        Value::String(string) => match string.trim().parse::<f64>() {
            Ok(val) => Ok(Value::Number(Number::Float(val))),
            Err(err) => Err(Error::eval(err.to_string())),
        },
        Value::Boolean(true) => Ok(Value::Number(Number::Float(1.0))),
        Value::Boolean(false) => Ok(Value::Number(Number::Float(0.0))),
        Value::Null => Ok(Value::Number(Number::Float(0.0))),
        _ => Err(Error::eval(format!(
            "argument {} does not cast to float",
            arg
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn float_takes_one_arg() {
        assert!(query_value("float 1".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("float 2 3".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn float_arg_must_be_a_scalar() {
        assert!(query_value("float [1,2,3]".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn float_casts_to_float() {
        assert_eq!(
            query_value("float 1".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(1.0)
        );
        assert_eq!(
            query_value("float null".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(0.0)
        );
        assert_eq!(
            query_value("float \"2.3\"".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(2.3)
        );
    }
}
