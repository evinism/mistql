use crate::{expr, Value};
use crate::{Error, Result, Rule};
use pest::iterators::Pairs;

pub fn reduce(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let args = match (
        context_opt,
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
    ) {
        (Some(target), Some(func), Some(init), None, None) => {
            (func, expr::eval(init, data, None)?, target)
        }
        (None, Some(func), Some(init), Some(target), None) => (
            func,
            expr::eval(init, data, None)?,
            expr::eval(target, data, None)?,
        ),
        _ => {
            return Err(Error::eval(
                "reduce requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (func, init, Value::Array(val)) => {
            let itr = val.into_iter();
            let mut result = init.clone();
            for elt in itr {
                let func_param = Value::Array(vec![result.clone(), elt]);
                result = expr::eval(func.clone(), &func_param, None)?;
            }

            Ok(result)
        }
        (val, _, _) => Err(Error::eval(format!(
            "argument to reduce must be an array (got {:?}",
            val
        ))),
    }
}
