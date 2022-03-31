use pest::iterators::Pair;
use std::str::FromStr;

use crate::eval::{expr, Value};
use crate::{Error, Result, Rule};

// mod float;
mod string;

enum Function {
    Count,
    // Float,
    Index,
    Log,
    String,
}

impl FromStr for Function {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "count" => Ok(Function::Count),
            // "float" => Ok(Function::Float),
            "index" => Ok(Function::Index),
            "log" => Ok(Function::Log),
            "string" => Ok(Function::String),
            _ => Err(Error::query(format!("unknown function {}", s))),
        }
    }
}

pub fn eval(pair: Pair<Rule>, data: &Value, context: Option<Value>) -> Result<Value> {
    let mut function_iter = pair.into_inner();
    let function: Function = function_iter.next().unwrap().as_str().parse()?;
    let mut use_implicit_context = true;
    let mut args = vec![];

    // lot of mutation required here but we need to capture use_implicit_context
    // as a side effect so an iterator chain is even more gross
    for arg in function_iter {
        match arg.as_rule() {
            Rule::at => {
                args.push(expr::eval(arg, data, context.clone())?);
                use_implicit_context = false
            }
            _ => args.push(expr::eval(arg, data, None)?),
        }
    }

    // if context is present, it becomes the last arg
    if use_implicit_context {
        if let Some(ctx) = context {
            args.push(ctx);
        }
    }

    match function {
        Function::Count => count(args),
        // Function::Float => float::float(args),
        Function::Index => super::index::index(args),
        Function::Log => log(args),
        Function::String => string::string(args),
    }
}

fn count(args: Vec<Value>) -> Result<Value> {
    if let Some(Value::Array(vals)) = args.get(0) {
        Ok(Value::Int(vals.len() as i64))
    } else {
        Err(Error::eval(format!(
            "argument to count must be an array (got {:?}",
            args
        )))
    }
}

fn log(args: Vec<Value>) -> Result<Value> {
    if let Some(val) = args.get(0) {
        Ok(val.clone())
    } else {
        Err(Error::eval("log requires one argument".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{MistQLParser, Rule};

    #[test]
    fn parses_basic_function_call() {
        parses_to! {
            parser: MistQLParser,
            input: "count [1,2,3]",
            rule: Rule::query,
            tokens: [
                function(0,13, [
                    ident(0,5),
                    array(6,13, [
                        number(7,8),
                        number(9,10),
                        number(11,12)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parses_function_with_three_arguments() {
        parses_to! {
            parser: MistQLParser,
            input: "if false 1 2",
            rule: Rule::query,
            tokens: [
                function(0,12, [
                    ident(0,2),
                    bool(3,8),
                    number(9,10),
                    number(11,12)
                ])
            ]
        }
    }

    #[test]
    fn functions_are_first_class_citizens() {
        parses_to! {
            parser: MistQLParser,
            input: "(if toggle keys values) {one: \"two\"}",
            rule: Rule::query,
            tokens: [
                function(0,36, [
                    function(1,22, [
                        ident(1,3),
                        ident(4,10),
                        ident(11,15),
                        ident(16,22)
                    ]),
                    object(24,36, [
                        keyval(25,35, [
                            ident(25,28),
                            string(30,35, [
                                inner(31,34)
                            ])
                        ])
                    ])
                ])
            ]
        }
    }

    #[test]
    fn function_with_function_as_parameter() {
        parses_to! {
            parser: MistQLParser,
            input: "reduce @[0] + @[1] 0 @",
            rule: Rule::query,
            tokens: [
                function(0,22, [
                    ident(0,6),
                    infix_expr(7,19, [
                        indexed_value(7,11, [
                            at(7,8),
                            number(9,10)
                        ]),
                        plus_op(12,13),
                        indexed_value(14,18, [
                            at(14,15),
                            number(16,17)
                        ])
                    ]),
                    number(19,20),
                    at(21,22)
                ])
            ]
        }
    }
}
