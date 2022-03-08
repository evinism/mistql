use crate::eval::prefix::truthiness;
use crate::Result;

pub fn and(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    Ok((truthiness(left) && truthiness(right)).into())
}

pub fn or(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    Ok((truthiness(left) || truthiness(right)).into())
}
