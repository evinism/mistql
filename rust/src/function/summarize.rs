use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;
// use std::collections::BTreeMap;

pub fn summarize(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("summarize requires one argument".to_string())),
    };

    match arg {
        Value::Array(_arr) => {
            // let mut result: BTreeMap<String, Value> = BTreeMap::new();

            Err(Error::unimplemented("summarize function".to_string()))
        }
        _ => Err(Error::eval(format!(
            "argument to summarize must be an array (got {:?}",
            arg
        ))),
    }
}
