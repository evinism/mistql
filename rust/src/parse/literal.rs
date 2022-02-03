use super::{parse_expression, Expression, Rule};
use crate::{Error, Result};
use pest::iterators::Pair;

#[derive(Clone, PartialEq, Debug)]
pub enum Literal<'a> {
    Object(Vec<(String, Expression<'a>)>),
    Array(Vec<Expression<'a>>),
    String(&'a str),
    Number(serde_json::Number),
    Boolean(bool),
    Null,
}

pub fn parse_literal(pair: Pair<Rule>) -> Result<Literal> {
    match pair.into_inner().next() {
        None => Err(Error::query(format!("no value found"))),
        Some(value) => match value.as_rule() {
            Rule::object => parse_object(value),
            Rule::array => parse_array(value),
            Rule::string => Ok(Literal::String(value.as_str())),
            Rule::number => Ok(Literal::Number(value.as_str().parse().unwrap())),
            Rule::boolean => Ok(Literal::Boolean(value.as_str().parse().unwrap())),
            Rule::null => Ok(Literal::Null),
            _ => Err(Error::query(format!("unknown value type {:?}", value))),
        },
    }
}

pub fn parse_object(pair: Pair<Rule>) -> Result<Literal> {
    Ok(Literal::Object(
        pair.into_inner()
            .map(|inner_pair| match inner_pair.as_rule() {
                Rule::keyval => {
                    let key_iter = inner_pair.clone().into_inner().step_by(2);
                    let val_iter = inner_pair.into_inner().skip(1).step_by(2);
                    key_iter.zip(val_iter).map(|(key, val)| {
                        let key = parse_obj_key(key);
                        let val = parse_expression(val);
                        (key, val.unwrap())
                    })
                }
                _ => unreachable!("not a keyval"),
            })
            .flatten()
            .collect(),
    ))
}

fn parse_obj_key(pair: Pair<Rule>) -> String {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::string => inner.into_inner().next().unwrap().as_str().to_string(),
        Rule::ident => inner.as_str().to_string(),
        _ => unreachable!("not a key"),
    }
}

pub fn parse_array(pair: Pair<Rule>) -> Result<Literal> {
    let contents: Result<Vec<Expression>> = pair.into_inner().map(parse_expression).collect();
    match contents {
        Ok(arr) => Ok(Literal::Array(arr)),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{MistQLParser, Rule};

    #[test]
    fn parses_positive_integer() {
        let query = "100000";
        parses_to! {
            parser: MistQLParser,
            input: query.clone(),
            rule: Rule::number,
            tokens: [
                number(0,6)
            ]
        }
    }

    #[test]
    fn parses_negative_integer() {
        let query = "-100000";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::number,
            tokens: [
                number(0,7)
            ]
        }
    }

    #[test]
    fn parses_zero() {
        let query = "0";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::number,
            tokens: [
                number(0,1)
            ]
        }
    }

    #[test]
    fn parses_float() {
        let query = "30.5";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::number,
            tokens: [
                number(0,4)
            ]
        }
    }

    #[test]
    fn parses_float_with_leading_zero() {
        let query = "0.9";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::number,
            tokens: [
                number(0,3)
            ]
        }
    }

    #[test]
    fn parses_negative_float() {
        let query = "-30.5";
        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::number,
            tokens: [
                number(0,5)
            ]
        }
    }

    #[test]
    fn parses_float_with_exponent() {
        parses_to! {
            parser: MistQLParser,
            input: "4.9E50",
            rule: Rule::number,
            tokens: [
                number(0,6)
            ]
        }
    }

    #[test]
    fn parses_negative_float_with_exponent() {
        parses_to! {
            parser: MistQLParser,
            input: "-30.5e-2",
            rule: Rule::number,
            tokens: [
                number(0,8)
            ]
        }
    }

    #[test]
    fn fails_to_parse_semver_as_number() {
        fails_with! {
            parser: MistQLParser,
            input: "0.9.5",
            rule: Rule::query,
            positives: vec![Rule::EOI],
            negatives: vec![],
            pos: 3
        }
    }

    #[test]
    fn fails_to_parse_two_zero_semver_as_number() {
        fails_with! {
            parser: MistQLParser,
            input: "0.0.5",
            rule: Rule::query,
            positives: vec![Rule::EOI],
            negatives: vec![],
            pos: 3
        }
    }

    #[test]
    fn parsing_objects() {
        let query = "{\"a\": 1, \"b\": 2, \"c\": 3}";

        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::object,
            tokens: [
                object(0,24, [
                    keyval(1,7, [
                        key(1,4, [
                            string(1,4, [
                                inner(2,3)
                            ])
                        ]),
                        expression(6,7, [
                            value(6,7, [
                                literal(6,7, [
                                    number(6,7)
                                ])
                            ])
                        ])
                    ]),
                    keyval(9,15, [
                        key(9,12, [
                            string(9,12, [
                                inner(10,11)
                            ])
                        ]),
                        expression(14,15, [
                            value(14,15, [
                                literal(14,15, [
                                    number(14,15)
                                ])
                            ])
                        ])
                    ]),
                    keyval(17,23, [
                        key(17,20, [
                            string(17,20, [
                                inner(18,19)
                            ])
                        ]),
                        expression(22,23, [
                            value(22,23, [
                                literal(22,23, [
                                    number(22,23)
                                ])
                            ])
                        ])
                    ]),
                ])
            ]
        }
    }

    #[test]
    fn parsing_objects_with_unqouted_keys() {
        let query = "{a: 1, b: 2, c: 3}";

        parses_to! {
            parser: MistQLParser,
            input: query,
            rule: Rule::object,
            tokens: [
                object(0,18, [
                    keyval(1,5, [
                        key(1,2, [
                            ident(1,2)
                        ]),
                        expression(4,5, [
                            value(4,5, [
                                literal(4,5, [
                                    number(4,5)
                                ])
                            ])
                        ])
                    ]),
                    keyval(7,11, [
                        key(7,8, [
                            ident(7,8)
                        ]),
                        expression(10,11, [
                            value(10,11, [
                                literal(10,11, [
                                    number(10,11)
                                ])
                            ])
                        ])
                    ]),
                    keyval(13,17, [
                        key(13,14, [
                            ident(13,14)
                        ]),
                        expression(16,17, [
                            value(16,17, [
                                literal(16,17, [
                                    number(16,17)
                                ])
                            ])
                        ])
                    ]),
                ])
            ]
        }
    }
}
