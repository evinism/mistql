fn main() {
    let query = "count @".to_string();
    let data = r#"
{ "foo": "bar" }
"#
    .to_string();

    let result = mistql::query(query, data);
    dbg!(result);
}
