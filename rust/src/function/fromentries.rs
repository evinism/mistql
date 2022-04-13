use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;
use std::collections::BTreeMap;

pub fn fromentries(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("fromentries requires one argument".to_string())),
    };
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
