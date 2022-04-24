use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn stringjoin(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let (join_val, target_val) = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next())
    {
        (Some(ctx), Some(arg1), None, None) => (expr::eval(arg1, data, None)?, ctx),
        (None, Some(arg1), Some(arg2), None) => {
            (expr::eval(arg1, data, None)?, expr::eval(arg2, data, None)?)
        }
        _ => return Err(Error::eval("stringjoin requires arguments".to_string())),
    };

    match (join_val, target_val) {
        (Value::String(join), Value::Array(target)) => {
            let joined = match target.first() {
                None => Ok(String::new()),
                Some(Value::String(init)) => {
                    target
                        .iter()
                        .skip(1)
                        .try_fold(init.clone(), |acc, elt| match elt {
                            Value::String(s) => Ok(format!("{}{}{}", acc, join, s).to_string()),
                            _ => Err(Error::eval("stringjoin target is not a string".to_string())),
                        })
                }
                _ => Err(Error::eval("stringjoin target is not a string".to_string())),
            }?;

            Ok(Value::String(joined))
        }
        _ => Err(Error::eval(
            "stringjoin args must be a string an array of strings".to_string(),
        )),
    }
}
