use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn reverse(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("reverse requires one argument".to_string())),
    };

    match arg {
        Value::Array(arr) => Ok(Value::Array(arr.into_iter().rev().collect())),
        _ => Err(Error::eval(format!(
            "argument to reverse must be an array (got {:?}",
            arg
        ))),
    }
}
