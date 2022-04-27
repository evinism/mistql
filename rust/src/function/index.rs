use super::args::ArgParser;
use crate::index::{item_index, range_index};
use crate::{Error, Result, Value};

pub fn index(arg_parser: ArgParser) -> Result<Value> {
    if let Ok((idx, target)) = arg_parser.two_args() {
        item_index(&idx, &target)
    } else if let Ok((low, high, target)) = arg_parser.three_args() {
        range_index(&low, &high, &target)
    } else {
        Err(Error::eval(
            "index requires two or three argument".to_string(),
        ))
    }
}

pub fn dot_index(raw_idx: &str, arg_parser: ArgParser) -> Result<Value> {
    let idx = Value::String(raw_idx.to_string());
    if let Ok(target) = arg_parser.one_arg() {
        item_index(&idx, &target)
    } else {
        item_index(&idx, arg_parser.data)
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
