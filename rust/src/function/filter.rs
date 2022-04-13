use crate::prefix::truthiness;
use crate::{expr, Value};
use crate::{Error, Result, Rule};
use pest::iterators::Pairs;
use std::collections::BTreeMap;

pub fn filter(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let args = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "filter requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (Value::Array(val), func) => {
            let mut filtered = vec![];
            for elt in val {
                let predicate = expr::eval(func.clone(), &elt, None)?;
                if truthiness(&predicate) {
                    filtered.push(elt);
                }
            }
            Ok(Value::Array(filtered))
        }
        (val, _) => Err(Error::eval(format!(
            "argument to filter must be an array (got {:?}",
            val
        ))),
    }
}

pub fn filterkeys(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let args = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "filterkeys requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (Value::Object(val), func) => {
            let mut mapped: BTreeMap<String, Value> = BTreeMap::new();
            for (k, v) in val.iter() {
                let predicate = expr::eval(func.clone(), &Value::String(k.clone()), None)?;
                if truthiness(&predicate) {
                    mapped.insert(k.clone(), v.clone());
                }
            }
            Ok(Value::Object(mapped))
        }
        (val, _) => Err(Error::eval(format!(
            "argument to filterkeys must be an object (got {:?}",
            val
        ))),
    }
}

pub fn filtervalues(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let args = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "filterkeys requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (Value::Object(val), func) => {
            let mut mapped: BTreeMap<String, Value> = BTreeMap::new();
            for (k, v) in val.iter() {
                let predicate = expr::eval(func.clone(), v, None)?;
                if truthiness(&predicate) {
                    mapped.insert(k.clone(), v.clone());
                }
            }
            Ok(Value::Object(mapped))
        }
        (val, _) => Err(Error::eval(format!(
            "argument to filterkeys must be an object (got {:?}",
            val
        ))),
    }
}
