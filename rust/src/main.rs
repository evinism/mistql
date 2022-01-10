use serde_json::json;

fn main() {
    let query = "count @".to_string();
    let data = json!([]);

    let result = mistql::query(query, data);
    dbg!(result);
}
