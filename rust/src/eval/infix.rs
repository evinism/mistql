use pest::iterators::Pair;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

use crate::eval::expr;
use crate::{Error, Result, Rule};

lazy_static! {
    static ref INFIX_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::plus_op, Assoc::Left) | Operator::new(Rule::minus_op, Assoc::Left),
        Operator::new(Rule::mult_op, Assoc::Left)
    ]);
}

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    let pairs = pair.into_inner();
    INFIX_CLIMBER.climb(
        pairs,
        |current_pair: Pair<Rule>| expr::eval(current_pair, context),
        |lhs: Result<serde_json::Value>, op: Pair<Rule>, rhs: Result<serde_json::Value>| match (
            lhs, rhs, op,
        ) {
            (Err(err), _, _) => Err(err),
            (_, Err(err), _) => Err(err),
            (Ok(left), Ok(right), op) => apply_operator(left, op, right),
        },
    )
}

fn apply_operator(
    left: serde_json::Value,
    op: Pair<Rule>,
    right: serde_json::Value,
) -> Result<serde_json::Value> {
    match op.as_rule() {
        Rule::plus_op => add(left, right),
        Rule::mult_op => multiply(left, right),
        _ => Err(Error::unimplemented(format!(
            "unimplemented operator {:?}",
            op.as_str()
        ))),
    }
}

fn add(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_i64(), right.as_i64()) {
        Ok((l + r).into())
    } else if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l + r).into())
    } else {
        Err(Error::eval("can't add non-numbers".to_string()))
    }
}

fn multiply(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_i64(), right.as_i64()) {
        Ok((l * r).into())
    } else if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l * r).into())
    } else {
        Err(Error::eval("can't add non-numbers".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{MistQLParser, Rule};
    #[test]
    fn parses_infix_operators() {
        let query = "1 + 3";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                infix_expr(0,5, [
                    number(0,1),
                    plus_op(2,3),
                    number(4,5)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(4))
    }

    #[test]
    fn parses_nested_infix_operators() {
        let query = "1 + 2 * 3";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                infix_expr(0,9, [
                    number(0,1),
                    plus_op(2,3),
                    number(4,5),
                    mult_op(6,7),
                    number(8,9)
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::from(7))
    }

    #[test]
    #[ignore]
    fn parses_infix_operators_as_function_args() {
        let query = "map @ + 1 [1, 2, 3]";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,19, [
                    ident(0,3),
                    infix_expr(4,10, [
                        at(4,5),
                        plus_op(6,7),
                        number(8,9)
                    ]),
                    array(10,19, [
                        number(11,12),
                        number(14,15),
                        number(17,18)
                    ])
                ])
            ]
        }

        let result = crate::query(query.to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                (2 as u32).into(),
                (3 as u32).into(),
                (4 as u32).into()
            ])
        )
    }

    #[test]
    fn throws_on_addition_of_booleans() {
        let query = "true + 3";
        let result = crate::query(query.to_string(), "null".to_string());
        assert!(result.is_err())
    }
}
