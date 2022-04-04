use crate::{expr, Value};
use crate::{Error, Result, Rule};
use pest::iterators::Pair;

pub fn map(func: Pair<Rule>, args: Vec<Value>) -> Result<Value> {
    match (args.len(), args.get(0)) {
        (1, Some(Value::Array(val))) => Ok(Value::Array(
            val.iter()
                .map(|elt| expr::eval(func.clone(), elt, None))
                .collect::<Result<Vec<Value>>>()?,
        )),
        (1, Some(val)) => Err(Error::eval(format!(
            "argument to keys must be an object (got {:?}",
            val
        ))),
        (n, _) => Err(Error::eval(format!("keys expected 1 argument, got {}", n))),
    }
}

// pub fn mapkeys(func: Pair<Rule>, args: Vec<Value>) -> Result<Value> {
//     match (args.len(), args.get(0)) {
//         (1, Some(Value::Object(val))) => Ok(Value::Array(
//             val.keys()
//                 .map(|elt| expr::eval(func.clone(), elt, None))
//                 .collect::<Result<Vec<Value>>>()?,
//         )),
//         (1, Some(val)) => Err(Error::eval(format!(
//             "argument to keys must be an object (got {:?}",
//             val
//         ))),
//         (n, _) => Err(Error::eval(format!("keys expected 1 argument, got {}", n))),
//     }
// }

pub fn mapvalues(func: Pair<Rule>, args: Vec<Value>) -> Result<Value> {
    match (args.len(), args.get(0)) {
        (1, Some(Value::Object(val))) => Ok(Value::Array(
            val.values()
                .map(|elt| expr::eval(func.clone(), elt, None))
                .collect::<Result<Vec<Value>>>()?,
        )),
        (1, Some(val)) => Err(Error::eval(format!(
            "argument to keys must be an object (got {:?}",
            val
        ))),
        (n, _) => Err(Error::eval(format!("keys expected 1 argument, got {}", n))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;
    use crate::{MistQLParser, Rule};
    use pest::Parser;

    #[test]
    fn map_parses() {
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
                (2 as i64).into(),
                (3 as i64).into(),
                (4 as i64).into()
            ])
        )
    }

    #[test]
    fn map_takes_one_arg() {
        let pair = MistQLParser::parse(Rule::query, "@ + 1")
            .unwrap()
            .next()
            .unwrap();
        assert!(map(pair.clone(), vec![]).is_err());
        assert!(map(pair.clone(), vec![Value::Array(vec![])]).is_ok());
        assert!(map(pair, vec![Value::Array(vec![]), Value::Array(vec![])]).is_err());
    }

    #[test]
    fn map_arg_must_be_an_array() {
        let pair = MistQLParser::parse(Rule::query, "@ + 1")
            .unwrap()
            .next()
            .unwrap();

        assert!(map(pair.clone(), vec![Value::Int(1)]).is_err());
        assert!(map(pair, vec![Value::String("abc".to_string())]).is_err());
    }

    // #[test]
    // fn mapvalues_takes_one_arg() {
    //     let pair = MistQLParser::parse(Rule::query, "@ + 1")
    //         .unwrap()
    //         .next()
    //         .unwrap();
    //     assert!(mapvalues(pair.clone(), vec![]).is_err());
    //     assert!(mapvalues(pair.clone(), vec![Value::Array(vec![])]).is_ok());
    //     assert!(mapvalues(pair, vec![Value::Array(vec![]), Value::Array(vec![])]).is_err());
    // }
}
