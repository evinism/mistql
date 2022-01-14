pub mod parse;

pub fn query_value(
    query_str: String,
    _data: serde_json::Value,
) -> Result<serde_json::Value, &'static str> {
    let ast = parse::expr(&query_str);
    dbg!(ast);
    Err("unimplemented")
}

pub fn query(query_str: String, data_str: String) -> Result<serde_json::Value, &'static str> {
    match serde_json::from_str(&data_str) {
        Ok(data) => query_value(query_str, data),
        Err(_) => Err("Invalid JSON data"),
    }
}
