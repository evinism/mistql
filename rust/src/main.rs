fn main() {
    let query = "count @".to_string();
    let data = "[]".to_string();

    let result = mistql::query(query, data);
    dbg!(result);
}
