use pest::iterators::Pair;

use crate::{Error, Result, Rule, Value};

mod apply;
mod count;
mod entries;
mod filter;
mod find;
mod flatten;
mod float;
mod fromentries;
mod groupby;
mod if_fn;
mod index;
mod keys;
mod log;
mod map;
mod reduce;
pub mod regex;
mod reverse;
mod sort;
mod string;
mod sum;
mod summarize;
mod values;

pub fn eval(pair: Pair<Rule>, data: &Value, context: Option<Value>) -> Result<Value> {
    let mut function_iter = pair.clone().into_inner();
    let function = match pair.as_rule() {
        Rule::ident => pair.as_str(),
        Rule::function => function_iter.next().unwrap().as_str(),
        _ => unreachable!(),
    };

    match function {
        "apply" => apply::apply(function_iter, data, context),
        "count" => count::count(function_iter, data, context),
        "entries" => entries::entries(function_iter, data, context),
        "filter" => filter::filter(function_iter, data, context),
        "filterkeys" => filter::filterkeys(function_iter, data, context),
        "filtervalues" => filter::filtervalues(function_iter, data, context),
        "find" => find::find(function_iter, data, context),
        "flatten" => flatten::flatten(function_iter, data, context),
        "float" => float::float(function_iter, data, context),
        "fromentries" => fromentries::fromentries(function_iter, data, context),
        "groupby" => groupby::groupby(function_iter, data, context),
        "if" => if_fn::if_fn(function_iter, data, context),
        "index" => index::index(function_iter, data, context),
        "keys" => keys::keys(function_iter, data, context),
        "log" => log::log(function_iter, data, context),
        "map" => map::map(function_iter, data, context),
        "mapkeys" => map::mapkeys(function_iter, data, context),
        "mapvalues" => map::mapvalues(function_iter, data, context),
        "match" => regex::match_fn(function_iter, data, context),
        "reduce" => reduce::reduce(function_iter, data, context),
        "regex" => regex::regex(function_iter, data, context),
        "replace" => Err(Error::unimplemented(format!("function {}", function))),
        "reverse" => reverse::reverse(function_iter, data, context),
        "sequence" => Err(Error::unimplemented(format!("function {}", function))),
        "sort" => sort::sort(function_iter, data, context),
        "sortby" => sort::sortby(function_iter, data, context),
        "split" => regex::split(function_iter, data, context),
        "string" => string::string(function_iter, data, context),
        "stringjoin" => Err(Error::unimplemented(format!("function {}", function))),
        "sum" => sum::sum(function_iter, data, context),
        "summarize" => summarize::summarize(function_iter, data, context),
        "values" => values::values(function_iter, data, context),
        // if we can't find a function, treat it as a dot index
        _ => index::dot_index(&function, function_iter, data, context),
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
