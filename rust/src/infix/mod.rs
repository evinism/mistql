use pest::iterators::Pair;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

use crate::{expr, function::regex, Error, Result, Rule, Value};

pub mod arithmetic;
mod boolean;
mod compare;

lazy_static! {
    static ref INFIX_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::plus_op, Assoc::Left) | Operator::new(Rule::minus_op, Assoc::Left),
        Operator::new(Rule::mult_op, Assoc::Left)
            | Operator::new(Rule::div_op, Assoc::Left)
            | Operator::new(Rule::mod_op, Assoc::Left),
        Operator::new(Rule::gte_op, Assoc::Left)
            | Operator::new(Rule::gt_op, Assoc::Left)
            | Operator::new(Rule::lte_op, Assoc::Left)
            | Operator::new(Rule::lt_op, Assoc::Left),
        Operator::new(Rule::eq_op, Assoc::Left)
            | Operator::new(Rule::ne_op, Assoc::Left)
            | Operator::new(Rule::match_op, Assoc::Left),
        Operator::new(Rule::and_op, Assoc::Left) | Operator::new(Rule::or_op, Assoc::Left)
    ]);
}

pub fn eval(pair: Pair<Rule>, data: &Value) -> Result<Value> {
    let pairs = pair.into_inner();
    INFIX_CLIMBER.climb(
        pairs,
        |current_pair: Pair<Rule>| expr::eval(current_pair, data, None),
        |lhs: Result<Value>, op: Pair<Rule>, rhs: Result<Value>| match (lhs, rhs, op) {
            (Err(err), _, _) => Err(err),
            (_, Err(err), _) => Err(err),
            (Ok(left), Ok(right), op) => apply_operator(left, op, right),
        },
    )
}

fn apply_operator(left: Value, op: Pair<Rule>, right: Value) -> Result<Value> {
    match op.as_rule() {
        Rule::plus_op => arithmetic::add(left, right),
        Rule::minus_op => arithmetic::subtract(left, right),
        Rule::mult_op => arithmetic::multiply(left, right),
        Rule::div_op => arithmetic::divide(left, right),
        Rule::mod_op => arithmetic::modulo(left, right),
        Rule::and_op => boolean::and(left, right),
        Rule::or_op => boolean::or(left, right),
        Rule::gte_op => compare::gte(left, right),
        Rule::gt_op => compare::gt(left, right),
        Rule::lte_op => compare::lte(left, right),
        Rule::lt_op => compare::lt(left, right),
        Rule::eq_op => compare::eq(left, right),
        Rule::ne_op => compare::ne(left, right),
        Rule::match_op => regex::match_op(left, right),
        _ => Err(Error::unimplemented(format!(
            "unimplemented operator {:?}",
            op.as_str()
        ))),
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
