use super::args::ArgParser;
use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pair;

pub fn sort(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?;

    match arg {
        Value::Array(arr) if arr.len() == 0 => Ok(Value::Array(vec![])),
        Value::Array(arr) => sorted_array(&arr),
        _ => Err(Error::eval(format!(
            "argument to sort must be an array (got {:?}",
            arg
        ))),
    }
}

pub fn sortby(arg_parser: ArgParser) -> Result<Value> {
    let args = arg_parser.one_func_one_arg()?;
    match args {
        (_, Value::Array(arr)) if arr.len() == 0 => Ok(Value::Array(vec![])),
        (func, Value::Array(arr)) => sorted_by_array(&arr, func),
        _ => Err(Error::eval(format!(
            "second argument to sortby must be an array (got {:?}",
            args
        ))),
    }
}

fn sorted_array(arr: &Vec<Value>) -> Result<Value> {
    if arr.iter().all(|elt| match elt {
        Value::String(_) => true,
        _ => false,
    }) {
        let mut strings = arr.clone();
        strings.sort_unstable_by(|l, r| match (l, r) {
            (Value::String(a), Value::String(b)) => a.cmp(b),
            _ => unreachable!(),
        });
        Ok(Value::Array(strings))
    } else if arr.iter().all(|elt| match elt {
        Value::Number(_) => true,
        _ => false,
    }) {
        let mut numbers = arr.clone();
        numbers.sort_unstable_by(|l, r| match (l, r) {
            (Value::Number(a), Value::Number(b)) => a.cmp(b),
            _ => unreachable!(),
        });
        Ok(Value::Array(numbers))
    } else if arr.iter().all(|elt| match elt {
        Value::Boolean(_) => true,
        _ => false,
    }) {
        let mut bools = arr.clone();
        bools.sort_unstable_by(|l, r| match (l, r) {
            (Value::Boolean(a), Value::Boolean(b)) => a.cmp(b),
            _ => unreachable!(),
        });
        Ok(Value::Array(bools))
    } else {
        Err(Error::eval(
            "can only sort arrays of booleans, strings, or numbers".to_string(),
        ))
    }
}

fn sorted_by_array(arr: &Vec<Value>, func: Pair<Rule>) -> Result<Value> {
    let sort_keys = arr
        .iter()
        .map(|elt| expr::eval(func.clone(), elt, None))
        .collect::<Result<Vec<Value>>>()?;
    if sort_keys.iter().all(|elt| match elt {
        Value::Boolean(_) => true,
        _ => false,
    }) || sort_keys.iter().all(|elt| match elt {
        Value::String(_) => true,
        _ => false,
    }) || sort_keys.iter().all(|elt| match elt {
        Value::Number(_) => true,
        _ => false,
    }) {
        let mut result = arr.clone();
        result.sort_unstable_by(|l, r| {
            // all of the unwrapping is safe because we verified that the
            // array of sort keys is sortable
            let left = expr::eval(func.clone(), l, None).unwrap();
            let right = expr::eval(func.clone(), r, None).unwrap();
            left.partial_cmp(&right).unwrap()
        });
        Ok(Value::Array(result))
    } else {
        Err(Error::eval(format!(
            "can only sortby if the key array is sortable (got {:?}",
            sort_keys
        )))
    }
}
