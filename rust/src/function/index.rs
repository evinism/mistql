use crate::index::{item_index, range_index};
use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn index(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    match (
        context_opt,
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
    ) {
        (Some(target), Some(idx), None, None, None) => {
            item_index(&expr::eval(idx, data, None)?, &target)
        }
        (Some(target), Some(low), Some(high), None, None) => range_index(
            &expr::eval(low, data, None)?,
            &expr::eval(high, data, None)?,
            &target,
        ),
        (None, Some(idx), Some(target), None, None) => item_index(
            &expr::eval(idx, data, None)?,
            &expr::eval(target, data, None)?,
        ),
        (None, Some(low), Some(high), Some(target), None) => range_index(
            &expr::eval(low, data, None)?,
            &expr::eval(high, data, None)?,
            &expr::eval(target, data, None)?,
        ),
        _ => {
            return Err(Error::eval(
                "index requires two or three argument".to_string(),
            ))
        }
    }
}

pub fn dot_index(
    raw_idx: &str,
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let idx = Value::String(raw_idx.to_string());
    dbg!(idx.clone());
    match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(target), None, None) => item_index(&idx, &target),
        (None, Some(target), None) => item_index(&idx, &expr::eval(target, data, None)?),
        (None, None, None) => item_index(&idx, data),
        _ => return Err(Error::eval("dot index requires two arguments".to_string())),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_index_as_function() {
        let query = "hello".to_string();
        let data = "{\"hello\": \"world\"}".to_string();

        let result = crate::query(query, data).unwrap();
        assert_eq!(result, serde_json::Value::String("world".to_string()))
    }
}
