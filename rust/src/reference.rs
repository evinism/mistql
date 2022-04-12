use crate::{expr, index, Result, Rule, Value};
use pest::iterators::Pair;

pub fn eval(pair: Pair<Rule>, data: &Value) -> Result<Value> {
    let mut current_data = data.clone();
    for subpair in pair.into_inner() {
        match subpair.as_rule() {
            Rule::ident => {
                current_data =
                    index::item_index(&Value::String(subpair.as_str().to_string()), &current_data)?
            }
            _ => current_data = expr::eval(subpair, &current_data, None)?,
        }
    }

    Ok(current_data)
}

#[cfg(test)]
mod tests {
    use crate::{MistQLParser, Rule};

    #[test]
    fn test_index_as_function() {
        let query = "hello".to_string();
        let data = "{\"hello\": \"world\"}".to_string();

        let result = crate::query(query, data).unwrap();
        assert_eq!(result, serde_json::Value::String("world".to_string()))
    }

    #[test]
    fn test_deep_dot_access() {
        let query = "hello.over.there";
        let data = "{\"hello\": {\"over\": {\"there\": \"world\"}}}".to_string();

        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                compound_reference(0,16, [
                    ident(0,5),
                    compound_reference(6,16, [
                        ident(6,10),
                        ident(11,16),
                    ]),
                ])
            ]
        }

        let result = crate::query(query.to_string(), data).unwrap();
        assert_eq!(result, serde_json::Value::String("world".to_string()))
    }
}
