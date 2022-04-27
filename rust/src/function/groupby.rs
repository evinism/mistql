use super::args::ArgParser;
use crate::{expr, Error, Result, Value};
use std::collections::BTreeMap;

pub fn groupby(arg_parser: ArgParser) -> Result<Value> {
    let args = arg_parser.one_func_one_arg()?;
    match args {
        (func, Value::Array(entries)) => {
            let mut result: BTreeMap<String, Value> = BTreeMap::new();
            for entry in entries.into_iter() {
                let key = expr::eval(func.clone(), &entry, None)?.to_string();
                match result.get(&key) {
                    Some(Value::Array(vals)) => {
                        let mut new_vals = vals.clone();
                        new_vals.push(entry);
                        result.insert(key, Value::Array(new_vals));
                    }
                    None => {
                        result.insert(key, Value::Array(vec![entry]));
                    }
                    _ => unreachable!(),
                }
            }
            Ok(Value::Object(result))
        }
        _ => Err(Error::eval(format!("groupby target must be an array"))),
    }
}
