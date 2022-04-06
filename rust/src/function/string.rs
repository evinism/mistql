use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn string(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("string requires one argument".to_string())),
    };

    Ok(Value::String(arg.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn string_takes_one_arg() {
        assert!(query_value("string [1,2,3]".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("string [1,2,3] 4".to_string(), serde_json::Value::Null).is_err());
    }

    // since this function delegates to the Display trait on value, unit
    // tests appear there
}
