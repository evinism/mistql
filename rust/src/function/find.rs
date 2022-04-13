use crate::prefix::truthiness;
use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn find(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let args = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "find requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (Value::Array(val), func) => {
            for elt in val.into_iter() {
                let predicate = expr::eval(func.clone(), &elt, None)?;
                if truthiness(&predicate) {
                    return Ok(elt);
                }
            }
            return Ok(Value::Null);
        }
        (val, _) => Err(Error::eval(format!(
            "argument to find must be an array (got {:?}",
            val
        ))),
    }
}
