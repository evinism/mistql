use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn flatten(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("flatten requires one argument".to_string())),
    };
    match arg {
        Value::Array(val) => {
            let mut flattened: Vec<Value> = vec![];
            for elt in val.into_iter() {
                if let Value::Array(a) = elt {
                    a.into_iter().for_each(|e| flattened.push(e));
                } else {
                    return Err(Error::eval(format!("can't flatten non-array {}", elt)));
                }
            }
            Ok(Value::Array(flattened))
        }
        _ => Err(Error::eval(format!("flatten expected array, got {}", arg))),
    }
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn flatten_takes_one_arg() {
        assert!(query_value(
            "flatten [[1, 2], [3, 4]]".to_string(),
            serde_json::Value::Null
        )
        .is_ok());
        assert!(query_value(
            "flatten [[1, 2], [3, 4]] [[5,6], [7,8".to_string(),
            serde_json::Value::Null
        )
        .is_err());
    }

    #[test]
    fn flatten_only_flattens_arrays() {
        assert!(query_value("flatten 1".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("flatten \"abc\"".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("flatten {a: {b: 2}}".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("flatten [[1,2], 3]".to_string(), serde_json::Value::Null).is_err());
    }
}
