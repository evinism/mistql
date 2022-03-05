use pest::iterators::Pair;

use crate::eval::{array, object, value};
use crate::{Error, Result, Rule};

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    match pair.as_rule() {
        Rule::at => Ok(context.clone()),
        Rule::bool | Rule::number | Rule::string | Rule::null => value::eval(pair, &context),
        Rule::array => array::eval(pair, &context),
        Rule::object => object::eval(pair, &context),
        _ => Err(Error::unimplemented(format!(
            "unimplemented rule {:?}",
            pair.as_rule()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{MistQLParser, Rule};

    #[test]
    fn test_at() {
        parses_to! {
            parser: MistQLParser,
            input: "@",
            rule: Rule::query,
            tokens: [
                at(0,1)
            ]
        }

        let result = crate::query("@".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::Null)
    }
}
