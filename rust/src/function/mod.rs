use pest::iterators::Pair;

use crate::{Error, Result, Rule, Value};

mod count;
mod entries;
mod flatten;
mod float;
mod index;
mod keys;
mod log;
mod map;
pub mod regex;
mod string;
mod sum;
mod values;

pub fn eval(pair: Pair<Rule>, data: &Value, context: Option<Value>) -> Result<Value> {
    let mut function_iter = pair.clone().into_inner();
    let function = match pair.as_rule() {
        Rule::ident => pair.as_str(),
        Rule::function => function_iter.next().unwrap().as_str(),
        _ => unreachable!(),
    };

    match function {
        "count" => count::count(function_iter, data, context),
        "entries" => entries::entries(function_iter, data, context),
        "flatten" => flatten::flatten(function_iter, data, context),
        "float" => float::float(function_iter, data, context),
        "index" => index::index(function_iter, data, context),
        "keys" => keys::keys(function_iter, data, context),
        "log" => log::log(function_iter, data, context),
        "map" => map::map(function_iter, data, context),
        "mapkeys" => map::mapkeys(function_iter, data, context),
        "mapvalues" => map::mapvalues(function_iter, data, context),
        "match" => regex::match_fn(function_iter, data, context),
        "regex" => regex::regex(function_iter, data, context),
        "split" => regex::split(function_iter, data, context),
        "string" => string::string(function_iter, data, context),
        "sum" => sum::sum(function_iter, data, context),
        "values" => values::values(function_iter, data, context),
        _ => Err(Error::unimplemented(format!("function {}", function))),
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
