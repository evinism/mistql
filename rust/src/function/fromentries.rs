use super::args::ArgParser;
use crate::{Error, Result, Value};
use std::collections::BTreeMap;

pub fn fromentries(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?;
    match arg {
        Value::Array(entries) => {
            let mut result = BTreeMap::new();
            for entry in entries.iter() {
                if let Value::Array(entry_pair) = entry {
                    match (entry_pair.get(0), entry_pair.get(1)) {
                        (Some(s), Some(val)) => {
                            result.insert(s.to_string(), val.clone());
                        }
                        (Some(s), None) => {
                            result.insert(s.to_string(), Value::Null);
                        }
                        (None, None) => {
                            result.insert(Value::Null.to_string(), Value::Null);
                        }
                        (None, Some(_)) => unreachable!(),
                    }
                } else {
                    return Err(Error::eval(
                        "each fromentries entry must be a two-item array".to_string(),
                    ));
                }
            }
            Ok(Value::Object(result))
        }
        _ => Err(Error::eval(format!(
            "fromentries expected object, got {}",
            arg
        ))),
    }
}
