use crate::{Error, Result, Rule, Value};
use args::ArgParser;
use pest::iterators::Pair;
use std::convert::TryFrom;

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

#[derive(Debug)]
enum Function {
    Apply,
    Count,
    Entries,
    Filter,
    FilterKeys,
    FilterValues,
    Find,
    Flatten,
    Float,
    FromEntries,
    GroupBy,
    If,
    Index,
    Keys,
    Log,
    Map,
    MapKeys,
    MapValues,
    Match,
    Reduce,
    Regex,
    Replace,
    Reverse,
    Sequence,
    Sort,
    SortBy,
    Split,
    String,
    StringJoin,
    Sum,
    Summarize,
    Values,
}

impl TryFrom<&str> for Function {
    type Error = Error;

    fn try_from(name: &str) -> Result<Function> {
        match name {
            "apply" => Ok(Function::Apply),
            "count" => Ok(Function::Count),
            "entries" => Ok(Function::Entries),
            "filter" => Ok(Function::Filter),
            "filterkeys" => Ok(Function::FilterKeys),
            "filtervalues" => Ok(Function::FilterValues),
            "find" => Ok(Function::Find),
            "flatten" => Ok(Function::Flatten),
            "float" => Ok(Function::Float),
            "fromentries" => Ok(Function::FromEntries),
            "groupby" => Ok(Function::GroupBy),
            "if" => Ok(Function::If),
            "index" => Ok(Function::Index),
            "keys" => Ok(Function::Keys),
            "log" => Ok(Function::Log),
            "map" => Ok(Function::Map),
            "mapkeys" => Ok(Function::MapKeys),
            "mapvalues" => Ok(Function::MapValues),
            "match" => Ok(Function::Match),
            "reduce" => Ok(Function::Reduce),
            "regex" => Ok(Function::Regex),
            "replace" => Ok(Function::Replace),
            "reverse" => Ok(Function::Reverse),
            "sequence" => Ok(Function::Sequence),
            "sort" => Ok(Function::Sort),
            "sortby" => Ok(Function::SortBy),
            "split" => Ok(Function::Split),
            "string" => Ok(Function::String),
            "stringjoin" => Ok(Function::StringJoin),
            "sum" => Ok(Function::Sum),
            "summarize" => Ok(Function::Summarize),
            "values" => Ok(Function::Values),
            function => Err(Error::eval(format!("unknown function {}", function))),
        }
    }
}

impl TryFrom<String> for Function {
    type Error = Error;

    fn try_from(name: String) -> Result<Function> {
        Function::try_from(name.as_str())
    }
}

pub fn eval(pair: Pair<Rule>, data: &Value, context: Option<Value>) -> Result<Value> {
    let arg_parser = ArgParser::from_pair(pair, data, context)?;

    match arg_parser.clone().function.try_into() {
        Ok(Function::Apply) => apply::apply(arg_parser),
        Ok(Function::Count) => count::count(arg_parser),
        Ok(Function::Entries) => entries::entries(arg_parser),
        Ok(Function::Filter) => filter::filter(arg_parser),
        Ok(Function::FilterKeys) => filter::filterkeys(arg_parser),
        Ok(Function::FilterValues) => filter::filtervalues(arg_parser),
        Ok(Function::Find) => find::find(arg_parser),
        Ok(Function::Flatten) => flatten::flatten(arg_parser),
        Ok(Function::Float) => float::float(arg_parser),
        Ok(Function::FromEntries) => fromentries::fromentries(arg_parser),
        Ok(Function::GroupBy) => groupby::groupby(arg_parser),
        Ok(Function::If) => if_fn::if_fn(arg_parser),
        Ok(Function::Index) => index::index(arg_parser),
        Ok(Function::Keys) => keys::keys(arg_parser),
        Ok(Function::Log) => log::log(arg_parser),
        Ok(Function::Map) => map::map(arg_parser),
        Ok(Function::MapKeys) => map::mapkeys(arg_parser),
        Ok(Function::MapValues) => map::mapvalues(arg_parser),
        Ok(Function::Match) => regex::match_fn(arg_parser),
        Ok(Function::Reduce) => reduce::reduce(arg_parser),
        Ok(Function::Regex) => regex::regex(arg_parser),
        Ok(Function::Replace) => regex::replace(arg_parser),
        Ok(Function::Reverse) => reverse::reverse(arg_parser),
        Ok(Function::Sequence) => Err(Error::unimplemented("function sequence".to_string())),
        Ok(Function::Sort) => sort::sort(arg_parser),
        Ok(Function::SortBy) => sort::sortby(arg_parser),
        Ok(Function::Split) => regex::split(arg_parser),
        Ok(Function::String) => string::string(arg_parser),
        Ok(Function::StringJoin) => stringjoin::stringjoin(arg_parser),
        Ok(Function::Sum) => sum::sum(arg_parser),
        Ok(Function::Summarize) => summarize::summarize(arg_parser),
        Ok(Function::Values) => values::values(arg_parser),
        Err(err) => Err(err),
    }
}

pub fn ident_eval(pair: Pair<Rule>, data: &Value, context: Option<Value>) -> Result<Value> {
    let arg_parser = ArgParser::from_ident(&pair, data, context)?;

    match pair.as_str().try_into() {
        // only single-argument functions work in this manner
        Ok(Function::Count) => count::count(arg_parser),
        Ok(Function::Entries) => entries::entries(arg_parser),
        Ok(Function::Flatten) => flatten::flatten(arg_parser),
        Ok(Function::Float) => float::float(arg_parser),
        Ok(Function::FromEntries) => fromentries::fromentries(arg_parser),
        Ok(Function::Keys) => keys::keys(arg_parser),
        Ok(Function::Log) => log::log(arg_parser),
        Ok(Function::Regex) => regex::regex(arg_parser),
        Ok(Function::Reverse) => reverse::reverse(arg_parser),
        Ok(Function::Sort) => sort::sort(arg_parser),
        Ok(Function::String) => string::string(arg_parser),
        Ok(Function::Sum) => sum::sum(arg_parser),
        Ok(Function::Summarize) => summarize::summarize(arg_parser),
        Ok(Function::Values) => values::values(arg_parser),
        // the only error here is unknown function, so treat it as a reference
        Err(_) => index::dot_index(pair.as_str(), arg_parser),
        // unsupported functions are also references - if you need to override
        // a function this way, use $
        Ok(function) => Err(Error::eval(format!(
            "can't treat {:?} as a bare identifier",
            function
        ))),
    }
}

pub fn fn_ident_eval(pair: Pair<Rule>, data: &Value, context: Option<Value>) -> Result<Value> {
    let arg_parser = ArgParser::from_pair(pair, data, context)?;

    match arg_parser.clone().function.as_str() {
        "if" => if_fn::if_fn_ident(arg_parser),
        function => Err(Error::unimplemented(format!(
            "unsupported fn_ident function {:?}",
            function
        ))),
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
                    fn_ident(0,5, [
                        ident(0,5)
                    ]),
                    fn_args(6,13, [
                        array(6,13, [
                            number(7,8),
                            number(9,10),
                            number(11,12)
                        ])
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
                    fn_ident(0,2, [
                        ident(0,2)
                    ]),
                    fn_args(3,12, [
                        bool(3,8),
                        number(9,10),
                        number(11,12)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn functions_are_first_class_citizens() {
        let query = "(if toggle keys values) {one: \"two\"}";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::query,
            tokens: [
                function(0,36, [
                    fn_ident(0,23, [
                        function(1,22, [
                            fn_ident(1,3, [
                                ident(1,3)
                            ]),
                            fn_args(4,22, [
                                ident(4,10),
                                ident(11,15),
                                ident(16,22)
                            ])
                        ]),
                    ]),
                    fn_args(24,36, [
                        object(24,36, [
                            keyval(25,35, [
                                ident(25,28),
                                string(30,35, [
                                    inner(31,34)
                                ])
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
                    fn_ident(0,6, [
                        ident(0,6)
                    ]),
                    fn_args(7,22, [
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
                ])
            ]
        }
    }
}
