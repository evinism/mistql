use crate::prefix::truthiness;
use crate::Result;
use crate::Value;

pub fn and(left: Value, right: Value) -> Result<Value> {
    match (truthiness(&left), truthiness(&right)) {
        (false, _) => Ok(left),
        (true, _) => Ok(right),
    }
}

pub fn or(left: Value, right: Value) -> Result<Value> {
    match (truthiness(&left), truthiness(&right)) {
        (true, _) => Ok(left),
        (false, _) => Ok(right),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn and_on_booleans() {
        let result = crate::query("true && true".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(true));

        let result = crate::query("true && false".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(false));

        let result = crate::query("false && true".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(false));

        let result = crate::query("false && false".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(false));
    }

    #[test]
    fn and_on_truthy_values() {
        let result = crate::query("1 && 1".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(1));

        let result = crate::query("1 && 0".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(0));

        let result = crate::query("\"a\" && \"\"".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(""));

        let result = crate::query("\"\" && \"a\"".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(""));

        let result = crate::query("\"a\" && \"b\"".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from("b"));

        let result = crate::query("[1] && []".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Array(vec![]));

        let result = crate::query("[1] && [2]".to_string(), "null".to_string()).unwrap();
        let expected = serde_json::Value::Array(vec![(2 as u32).into()]);
        assert_eq!(result, expected);

        let result = crate::query("[] && [2]".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Array(vec![]));
    }

    #[test]
    fn and_on_mixed_types() {
        let result = crate::query("null && 0".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Null);

        let result = crate::query("null && 1".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Null);

        let result = crate::query("0 && null".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(0));

        let result = crate::query("1 && null".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Null);
    }

    #[test]
    fn or_on_booleans() {
        let result = crate::query("true || true".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(true));

        let result = crate::query("true || false".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(true));

        let result = crate::query("false || true".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(true));

        let result = crate::query("false || false".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Bool(false));
    }

    #[test]
    fn or_on_truthy_values() {
        let result = crate::query("1 || 1".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(1));

        let result = crate::query("1 || 0".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(1));

        let result = crate::query("\"a\" || \"\"".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from("a"));

        let result = crate::query("\"\" || \"a\"".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from("a"));

        let result = crate::query("\"a\" || \"b\"".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from("a"));

        let result = crate::query("[1] || []".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Array(vec![(1 as u32).into()]));

        let result = crate::query("[1] || [2]".to_string(), "null".to_string()).unwrap();
        let expected = serde_json::Value::Array(vec![(1 as u32).into()]);
        assert_eq!(result, expected);

        let result = crate::query("[] || [2]".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Array(vec![(2 as u32).into()]));
    }

    #[test]
    fn or_on_mixed_types() {
        let result = crate::query("null || 0".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(0));

        let result = crate::query("null || 1".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(1));

        let result = crate::query("0 || null".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Null);

        let result = crate::query("1 || null".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(1));
    }
}
