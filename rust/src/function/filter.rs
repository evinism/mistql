use super::args::ArgParser;
use crate::prefix::truthiness;
use crate::{expr, Value};
use crate::{Error, Result};
use std::collections::BTreeMap;

pub fn filter(arg_parser: ArgParser) -> Result<Value> {
    let (func_arg, target_arg) = arg_parser.two_args()?;
    match (func_arg.to_pair()?, target_arg.to_value(arg_parser.data)?) {
        (func, Value::Array(val)) => {
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

pub fn filterkeys(arg_parser: ArgParser) -> Result<Value> {
    let (func_arg, target_arg) = arg_parser.two_args()?;
    match (func_arg.to_pair()?, target_arg.to_value(arg_parser.data)?) {
        (func, Value::Object(val)) => {
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

pub fn filtervalues(arg_parser: ArgParser) -> Result<Value> {
    let (func_arg, target_arg) = arg_parser.two_args()?;
    match (func_arg.to_pair()?, target_arg.to_value(arg_parser.data)?) {
        (func, Value::Object(val)) => {
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
