use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;
use std::collections::BTreeMap;

pub fn groupby(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let args = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "groupby requires one function and one target".to_string(),
            ))
        }
    };
    match args {
        (Value::Array(entries), func) => {
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
