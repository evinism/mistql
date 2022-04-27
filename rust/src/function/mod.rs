use pest::iterators::Pair;

use crate::{Result, Rule, Value};
use args::ArgParser;

mod apply;
mod args;
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
mod stringjoin;
mod sum;
mod summarize;
mod values;

pub fn eval(pair: Pair<Rule>, data: &Value, context: Option<Value>) -> Result<Value> {
    dbg!(pair.as_str());
    let arg_parser = ArgParser::new(pair, data, context)?;

    match arg_parser.function.clone().as_str() {
        "apply" => apply::apply(arg_parser),
        "count" => count::count(arg_parser),
        "entries" => entries::entries(arg_parser),
        "filter" => filter::filter(arg_parser),
        "filterkeys" => filter::filterkeys(arg_parser),
        "filtervalues" => filter::filtervalues(arg_parser),
        "find" => find::find(arg_parser),
        "flatten" => flatten::flatten(arg_parser),
        "float" => float::float(arg_parser),
        "fromentries" => fromentries::fromentries(arg_parser),
        "groupby" => groupby::groupby(arg_parser),
        "if" => if_fn::if_fn(arg_parser),
        "index" => index::index(arg_parser),
        "keys" => keys::keys(arg_parser),
        "log" => log::log(arg_parser),
        "map" => map::map(arg_parser),
        "mapkeys" => map::mapkeys(arg_parser),
        "mapvalues" => map::mapvalues(arg_parser),
        "match" => regex::match_fn(arg_parser),
        "reduce" => reduce::reduce(arg_parser),
        "regex" => regex::regex(arg_parser),
        "replace" => regex::replace(arg_parser),
        "reverse" => reverse::reverse(arg_parser),
        // "sequence" => Err(Error::unimplemented(format!("function {}", function))),
        "sort" => sort::sort(arg_parser),
        "sortby" => sort::sortby(arg_parser),
        "split" => regex::split(arg_parser),
        "string" => string::string(arg_parser),
        "stringjoin" => stringjoin::stringjoin(arg_parser),
        "sum" => sum::sum(arg_parser),
        "summarize" => summarize::summarize(arg_parser),
        "values" => values::values(arg_parser),
        // // if we can't find a function, treat it as a dot index
        function => index::dot_index(function, arg_parser),
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
    #[ignore]
    fn functions_are_first_class_citizens() {
        let query = "(if toggle keys values) {one: \"two\"}";
        parses_to! {
            parser: MistQLParser,
            input: query,
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

        assert_eq!(
            crate::query(query.to_string(), "{\"toggle\": true}".to_string()).unwrap(),
            serde_json::Value::Array(vec!["one".into()])
        );

        assert_eq!(
            crate::query(query.to_string(), "{\"toggle\": false}".to_string()).unwrap(),
            serde_json::Value::Array(vec!["two".into()])
        );
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
