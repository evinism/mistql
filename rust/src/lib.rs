pub mod parse;

pub fn query(
    _query_str: String,
    _data: serde_json::Value,
) -> Result<serde_json::Value, &'static str> {
    Err("unimplemented")
}
