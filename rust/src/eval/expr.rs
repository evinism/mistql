use pest::iterators::Pair;

use crate::eval::{array, function, infix, object, prefix, value};
use crate::{Error, Result, Rule};

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    match pair.as_rule() {
        Rule::at => Ok(context.clone()),
        Rule::ident => Ok(pair.as_str().into()),
        Rule::bool | Rule::number | Rule::string | Rule::null => value::eval(pair, &context),
        Rule::array => array::eval(pair, &context),
        Rule::object => object::eval(pair, &context),
        Rule::infix_expr => infix::eval(pair, &context),
        Rule::function => function::eval(pair, &context),
        Rule::prefixed_value => prefix::eval(pair, &context),
        Rule::piped_expr => eval_piped(pair, &context),
        _ => Err(Error::unimplemented(format!(
            "unimplemented rule {:?}",
            pair.as_rule()
        ))),
    }
}

fn eval_piped(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    pair.into_inner()
        .try_fold(context.clone(), |ctx, expr| eval(expr, &ctx))
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

    #[test]
    fn parses_piped_expression() {
        let query = "[1,2,3] | count @";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                piped_expr(0,17, [
                    array(0,7, [
                        number(1,2),
                        number(3,4),
                        number(5,6)
                    ]),
                    function(10,17, [
                        ident(10,15),
                        at(16,17)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(3))
    }
}
