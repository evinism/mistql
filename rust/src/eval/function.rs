use crate::parse::{Expression, Function};
use crate::{Error, Result};

pub fn call_fn(
    func: Function,
    raw_args: Vec<Expression>,
    context: &serde_json::Value,
) -> Result<serde_json::Value> {
    let args = resolve_args(raw_args, context)?;

    match func {
        Function::Regex => Ok("regex".into()),
        Function::Count => match args.get(0) {
            Some(arg) => count(arg),
            None => Err(Error::evaluation("count requires one argument".to_string())),
        },
        Function::Sum => match args.get(0) {
            Some(arg) => sum(arg),
            None => Err(Error::evaluation("sum requires one argument".to_string())),
        },
    }
}

fn resolve_args(
    raw_args: Vec<Expression>,
    context: &serde_json::Value,
) -> Result<Vec<serde_json::Value>> {
    let eval_args: Result<Vec<serde_json::Value>> =
        raw_args.iter().map(|arg| arg.evaluate(context)).collect();
    match eval_args {
        Err(err) => Err(err),
        Ok(args) => Ok(args),
    }
}

fn count(list: &serde_json::Value) -> Result<serde_json::Value> {
    match list {
        serde_json::Value::Array(arr) => Ok(arr.len().into()),
        _ => Err(Error::evaluation("uncountable argument".to_string())),
    }
}

fn sum(list: &serde_json::Value) -> Result<serde_json::Value> {
    match list {
        serde_json::Value::Array(arr) => {
            if arr.iter().all(|elt| elt.is_i64()) {
                Ok(arr
                    .iter()
                    .map(|elt| elt.as_i64().unwrap())
                    .sum::<i64>()
                    .into())
            } else if arr.iter().all(|elt| elt.is_f64()) {
                Ok(arr
                    .iter()
                    .map(|elt| elt.as_f64().unwrap())
                    .sum::<f64>()
                    .into())
            } else {
                Err(Error::evaluation(
                    "one or more non-numerical elements in sum".to_string(),
                ))
            }
        }
        _ => Err(Error::evaluation("unsummable argument".to_string())),
    }
}
