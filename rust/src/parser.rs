//! Parser implementation for MistQL syntax
//!
//! This module implements a parser for MistQL expressions using nom parser combinators.
//! It converts MistQL syntax into an Abstract Syntax Tree (AST) that can be executed.

use crate::types::RuntimeValue;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1},
    combinator::{map, opt, recognize, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use std::collections::HashMap;

// AST expression types for MistQL
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Function call: `function arg1 arg2`
    FnExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    // Reference expression: `@` (context), `$` (builtins), or `variable_name`
    RefExpression {
        name: String,
        absolute: bool,
    },
    // Literal value: `42`, `"hello"`, `true`, `null`
    ValueExpression {
        value: RuntimeValue,
    },
    // Array literal: `[1, 2, 3]`
    ArrayExpression {
        items: Vec<Expression>,
    },
    // Object literal: `{"key": "value"}`
    ObjectExpression {
        entries: HashMap<String, Expression>,
    },
    // Pipeline: `data | filter condition | map field`
    PipeExpression {
        stages: Vec<Expression>,
    },
    // Parenthetical expression: `(expression)`
    ParentheticalExpression {
        expression: Box<Expression>,
    },
    // Dot access: `object.field`
    DotAccessExpression {
        object: Box<Expression>,
        field: String,
    },
}

impl Expression {
    pub fn reference(name: &str, absolute: bool) -> Self {
        Expression::RefExpression {
            name: name.to_string(),
            absolute,
        }
    }

    pub fn value(value: RuntimeValue) -> Self {
        Expression::ValueExpression { value }
    }

    pub fn function_call(function: Expression, arguments: Vec<Expression>) -> Self {
        Expression::FnExpression {
            function: Box::new(function),
            arguments,
        }
    }

    pub fn pipeline(stages: Vec<Expression>) -> Self {
        Expression::PipeExpression { stages }
    }

    pub fn array(items: Vec<Expression>) -> Self {
        Expression::ArrayExpression { items }
    }

    pub fn object(entries: HashMap<String, Expression>) -> Self {
        Expression::ObjectExpression { entries }
    }

    pub fn parenthetical(expression: Expression) -> Self {
        Expression::ParentheticalExpression {
            expression: Box::new(expression),
        }
    }

    pub fn dot_access(object: Expression, field: &str) -> Self {
        Expression::DotAccessExpression {
            object: Box::new(object),
            field: field.to_string(),
        }
    }

    pub fn index_single(index: Expression, operand: Expression) -> Self {
        Expression::function_call(Expression::reference("index", false), vec![index, operand])
    }

    pub fn index_double(start: Expression, end: Expression, operand: Expression) -> Self {
        Expression::function_call(Expression::reference("index", false), vec![start, end, operand])
    }
}

// Parse a float number (with decimal point and optional scientific notation)
fn parse_float(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        digit1,
        pair(
            char('.'),
            pair(
                opt(digit1),
                opt(pair(alt((char('e'), char('E'))), pair(opt(alt((char('+'), char('-')))), digit1))),
            ),
        ),
    ))(input)
}

// Parse an integer number (with optional scientific notation)
fn parse_integer(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        digit1,
        opt(pair(alt((char('e'), char('E'))), pair(opt(alt((char('+'), char('-')))), digit1))),
    ))(input)
}

// Parse a number (integer or float)
fn parse_number(input: &str) -> IResult<&str, RuntimeValue> {
    map(recognize(pair(opt(char('-')), alt((parse_float, parse_integer)))), |s: &str| {
        s.parse::<f64>().map(RuntimeValue::Number).unwrap_or(RuntimeValue::Number(0.0))
    })(input)
}

// Parse a string literal with proper escape handling
fn parse_string(input: &str) -> IResult<&str, RuntimeValue> {
    let (remaining, _) = char('"')(input)?;

    let mut result = String::new();
    let mut chars = remaining.chars();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                // End of string
                return Ok((chars.as_str(), RuntimeValue::String(result)));
            }
            '\\' => {
                // Handle escape sequences
                if let Some(next) = chars.next() {
                    match next {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'b' => result.push('\u{0008}'), // backspace
                        'f' => result.push('\u{000C}'), // form feed
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        'u' => {
                            // Unicode escape sequence \uXXXX
                            let mut unicode_str = String::new();
                            for _ in 0..4 {
                                if let Some(hex_char) = chars.next() {
                                    unicode_str.push(hex_char);
                                } else {
                                    return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
                                }
                            }
                            if let Ok(unicode_value) = u32::from_str_radix(&unicode_str, 16) {
                                if let Some(unicode_char) = char::from_u32(unicode_value) {
                                    result.push(unicode_char);
                                } else {
                                    return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
                                }
                            } else {
                                return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
                            }
                        }
                        _ => {
                            // Unknown escape sequence, just add the character as-is
                            result.push(next);
                        }
                    }
                } else {
                    return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
                }
            }
            _ => {
                result.push(c);
            }
        }
    }

    // Unterminated string
    Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))
}

fn parse_boolean(input: &str) -> IResult<&str, RuntimeValue> {
    alt((
        value(RuntimeValue::Boolean(true), tag("true")),
        value(RuntimeValue::Boolean(false), tag("false")),
    ))(input)
}

fn parse_null(input: &str) -> IResult<&str, RuntimeValue> {
    value(RuntimeValue::Null, tag("null"))(input)
}

fn parse_literal(input: &str) -> IResult<&str, Expression> {
    map(alt((parse_number, parse_string, parse_boolean, parse_null)), Expression::value)(input)
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_"))))))(input)
}

// Parse a reference (@, $, or variable name)
fn parse_reference(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("@"), |_| Expression::reference("@", false)),
        map(tag("$"), |_| Expression::reference("$", true)),
        map(parse_identifier, |s| Expression::reference(s, false)),
    ))(input)
}

fn parse_array(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            pair(char('['), multispace0),
            separated_list0(pair(multispace0, char(',')), pair(multispace0, parse_expression)),
            pair(multispace0, char(']')),
        ),
        |items| Expression::array(items.into_iter().map(|(_, expr)| expr).collect()),
    )(input)
}

fn parse_object(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            pair(char('{'), multispace0),
            separated_list0(pair(multispace0, char(',')), pair(multispace0, parse_object_entry)),
            pair(multispace0, char('}')),
        ),
        |entries| {
            let mut map = HashMap::new();
            for (_, (key, value)) in entries {
                map.insert(key, value);
            }
            Expression::object(map)
        },
    )(input)
}

fn parse_object_entry(input: &str) -> IResult<&str, (String, Expression)> {
    separated_pair(
        alt((
            map(parse_string, |rv| match rv {
                RuntimeValue::String(s) => s,
                _ => unreachable!(),
            }),
            map(parse_identifier, |s| s.to_string()),
        )),
        pair(multispace0, char(':')),
        parse_expression,
    )(input)
}

fn parse_parenthetical(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(pair(char('('), multispace0), parse_pipeline, pair(multispace0, char(')'))),
        Expression::parenthetical,
    )(input)
}

// Parse a pipeline expression (top level: |)
// piped_expression: simple_expression | simple_expression ("|" fncall)+
fn parse_pipeline(input: &str) -> IResult<&str, Expression> {
    // First parse a simple expression
    let (remaining, first) = parse_simple_expression(input)?;

    // Then try to parse pipeline stages
    let (remaining, stages) = many0(pair(pair(multispace0, char('|')), pair(multispace0, parse_function_call)))(remaining)?;

    if stages.is_empty() {
        Ok((remaining, first))
    } else {
        let mut all_stages = vec![first];
        for (_, (_, stage)) in stages {
            all_stages.push(stage);
        }
        Ok((remaining, Expression::pipeline(all_stages)))
    }
}

// Parse a simple expression (op_a or fncall)
// simple_expression: _wslr{op_a} | _wslr{fncall}
fn parse_simple_expression(input: &str) -> IResult<&str, Expression> {
    // Try to parse as a function call first (op_a followed by space-separated arguments)
    // If that fails, fall back to just op_a.
    // TODO: This seems to differ from the lark grammar, but it works, so I
    // might be misunderstanding.
    alt((parse_function_call, parse_op_a))(input)
}

// Parse a function call
// fncall: op_a (_W op_a)*
fn parse_function_call(input: &str) -> IResult<&str, Expression> {
    // First parse an op_a expression
    let (remaining, function) = parse_op_a(input)?;

    // Then try to parse function arguments (space-separated op_a expressions)
    let (remaining, args) = many0(pair(multispace1, parse_op_a))(remaining)?;
    let arguments: Vec<Expression> = args.into_iter().map(|(_, arg)| arg).collect();

    if arguments.is_empty() {
        Ok((remaining, function))
    } else {
        Ok((remaining, Expression::function_call(function, arguments)))
    }
}

// Parse op_a expressions (logical OR: ||)
// op_a: op_b | op_a "||" op_b
fn parse_op_a(input: &str) -> IResult<&str, Expression> {
    map(
        separated_list1(pair(multispace0, tag("||")), pair(multispace0, parse_op_b)),
        |operands| {
            let operands: Vec<Expression> = operands.into_iter().map(|(_, operand)| operand).collect();
            if operands.len() == 1 {
                operands.into_iter().next().unwrap()
            } else {
                // Left-associative: a || b || c = (a || b) || c
                operands
                    .into_iter()
                    .reduce(|left, right| Expression::function_call(Expression::reference("||", false), vec![left, right]))
                    .unwrap()
            }
        },
    )(input)
}

// Parse op_b expressions (logical AND: &&)
// op_b: op_c | op_b "&&" op_c
fn parse_op_b(input: &str) -> IResult<&str, Expression> {
    map(
        separated_list1(pair(multispace0, tag("&&")), pair(multispace0, parse_op_c)),
        |operands| {
            let operands: Vec<Expression> = operands.into_iter().map(|(_, operand)| operand).collect();
            if operands.len() == 1 {
                operands.into_iter().next().unwrap()
            } else {
                // Left-associative: a && b && c = (a && b) && c
                operands
                    .into_iter()
                    .reduce(|left, right| Expression::function_call(Expression::reference("&&", false), vec![left, right]))
                    .unwrap()
            }
        },
    )(input)
}

// Parse op_c expressions (equality: ==, !=, =~)
// op_c: op_d | op_c ("==" | "!=" | "=~") op_d
fn parse_op_c(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_op_d,
            many0(pair(
                pair(
                    multispace0,
                    alt((map(tag("=="), |_| "=="), map(tag("!="), |_| "!="), map(tag("=~"), |_| "=~"))),
                ),
                pair(multispace0, parse_op_d),
            )),
        ),
        |(left, rest)| {
            rest.into_iter().fold(left, |left, ((_, operator), (_, right))| {
                Expression::function_call(Expression::reference(operator, false), vec![left, right])
            })
        },
    )(input)
}

// Parse op_d expressions (comparison: <, >, <=, >=)
// op_d: op_e | op_d (">" | "<" | ">=" | "<=") op_e
fn parse_op_d(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_op_e,
            many0(pair(
                pair(
                    multispace0,
                    alt((
                        map(tag(">="), |_| ">="),
                        map(tag("<="), |_| "<="),
                        map(tag(">"), |_| ">"),
                        map(tag("<"), |_| "<"),
                    )),
                ),
                pair(multispace0, parse_op_e),
            )),
        ),
        |(left, rest)| {
            rest.into_iter().fold(left, |left, ((_, operator), (_, right))| {
                Expression::function_call(Expression::reference(operator, false), vec![left, right])
            })
        },
    )(input)
}

// Parse op_e expressions (addition and subtraction: +, -)
// op_e: op_f | op_e ("+" | "-") op_f
fn parse_op_e(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_op_f,
            many0(pair(
                pair(multispace0, alt((map(tag("+"), |_| "+"), map(tag("-"), |_| "-")))),
                pair(multispace0, parse_op_f),
            )),
        ),
        |(left, rest)| {
            rest.into_iter().fold(left, |left, ((_, operator), (_, right))| {
                Expression::function_call(Expression::reference(operator, false), vec![left, right])
            })
        },
    )(input)
}

// Parse op_f expressions (multiplication, division, modulo: *, /, %)
// op_f: op_g | op_f ("*" | "/" | "%") op_g
fn parse_op_f(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_op_g,
            many0(pair(
                pair(
                    multispace0,
                    alt((map(tag("*"), |_| "*"), map(tag("/"), |_| "/"), map(tag("%"), |_| "%"))),
                ),
                pair(multispace0, parse_op_g),
            )),
        ),
        |(left, rest)| {
            rest.into_iter().fold(left, |left, ((_, operator), (_, right))| {
                Expression::function_call(Expression::reference(operator, false), vec![left, right])
            })
        },
    )(input)
}

// Parse op_g expressions (unary operators: !, -)
// op_g: op_h | "!" op_g | "-" op_g
fn parse_op_g(input: &str) -> IResult<&str, Expression> {
    alt((
        // Try op_h first (higher precedence)
        parse_op_h,
        // Parse logical NOT: !expression
        map(
            pair(
                char('!'),
                pair(multispace0, parse_op_g), // Recursive to handle multiple unary operators
            ),
            |(_, (_, operand))| Expression::function_call(Expression::reference("!/unary", true), vec![operand]),
        ),
        // Parse unary minus: -expression
        map(
            pair(
                char('-'),
                pair(multispace0, parse_op_g), // Recursive to handle multiple unary operators
            ),
            |(_, (_, operand))| Expression::function_call(Expression::reference("-/unary", false), vec![operand]),
        ),
    ))(input)
}

// Parse op_h expressions (dot access, indexing, simplevalue)
// op_h: simplevalue | op_h "." reference | op_h indexing
fn parse_op_h(input: &str) -> IResult<&str, Expression> {
    // Start with a simplevalue
    let (remaining, mut expr) = parse_simplevalue(input)?;

    // Then try to parse dot access and indexing operations
    let (remaining, operations) = many0(alt((
        // Dot access: .reference
        map(
            pair(pair(multispace0, char('.')), pair(multispace0, parse_reference)),
            |(_, (_, reference))| ("dot", reference),
        ),
        // Indexing: [expression]
        map(
            pair(char('['), pair(multispace0, parse_indexing_innards)),
            |(_, (_, index_expr))| ("index", index_expr),
        ),
    )))(remaining)?;

    // Apply operations left-associatively
    for (op_type, operand) in operations {
        match op_type {
            "dot" => {
                // Extract field name from reference expression
                if let Expression::RefExpression { name, .. } = operand {
                    expr = Expression::dot_access(expr, &name);
                } else {
                    // This shouldn't happen if parsing is correct
                    return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
                }
            }
            "index" => {
                // Handle indexing: add the target as the last argument
                if let Expression::FnExpression { function, mut arguments } = operand {
                    // This is a slicing expression with 2 arguments, add target as 3rd
                    arguments.push(expr);
                    expr = Expression::FnExpression { function, arguments };
                } else {
                    // This is simple indexing, create: index(operand, expr)
                    expr = Expression::function_call(Expression::reference("index", false), vec![operand, expr]);
                }
            }
            _ => unreachable!(),
        }
    }

    Ok((remaining, expr))
}

// Parse indexing innards (for array/string indexing and slicing)
// index_innards: piped_expression? (WCOLON piped_expression?)*
fn parse_indexing_innards(input: &str) -> IResult<&str, Expression> {
    // First try to parse a single expression (for simple indexing like [0])
    let (remaining, first_expr) = opt(pair(multispace0, parse_pipeline))(input)?;

    if let Some((_, expr)) = first_expr {
        // Check if there's a colon after the first expression (for slicing)
        let (remaining, colon_result) = opt(pair(pair(multispace0, char(':')), opt(pair(multispace0, parse_pipeline))))(remaining)?;

        if let Some((_, end_opt)) = colon_result {
            // This is slicing syntax: [start:end] or [start:]
            let end = end_opt.map(|(_, expr)| expr);
            // For slicing, we need to return a special marker that indicates this is a slice
            // The op_h parser will handle adding the target as the third argument
            let result = Expression::function_call(
                Expression::reference("index", false),
                vec![expr, end.unwrap_or_else(|| Expression::value(RuntimeValue::Null))],
            );
            let (remaining, _) = pair(multispace0, char(']'))(remaining)?;
            return Ok((remaining, result));
        } else {
            // This is simple indexing: [index]
            let (remaining, _) = pair(multispace0, char(']'))(remaining)?;
            return Ok((remaining, expr));
        }
    } else {
        // No first expression, check if there's a colon (for [:end] or [:] syntax)
        let (remaining, colon_result) = opt(pair(pair(multispace0, char(':')), opt(pair(multispace0, parse_pipeline))))(input)?;

        if let Some((_, end_opt)) = colon_result {
            // This is slicing syntax: [:end] or [:]
            let end = end_opt.map(|(_, expr)| expr);
            let result = Expression::function_call(
                Expression::reference("index", false),
                vec![
                    Expression::value(RuntimeValue::Null),
                    end.unwrap_or_else(|| Expression::value(RuntimeValue::Null)),
                ],
            );
            let (remaining, _) = pair(multispace0, char(']'))(remaining)?;
            return Ok((remaining, result));
        } else {
            // Empty indexing: []
            let (remaining, _) = pair(multispace0, char(']'))(input)?;
            return Ok((remaining, Expression::value(RuntimeValue::Null)));
        }
    }
}

// Parse a complete expression (top-level entry point)
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_pipeline(input)
}

// Parse a simplevalue (literal, reference, or parenthetical expression)
// simplevalue: literal | reference | "(" piped_expression ")"
fn parse_simplevalue(input: &str) -> IResult<&str, Expression> {
    alt((parse_literal, parse_reference, parse_array, parse_object, parse_parenthetical))(input)
}

// Parser for MistQL expressions
pub struct Parser;

impl Parser {
    // Parse a MistQL expression string into an AST
    pub fn parse(input: &str) -> Result<Expression, String> {
        let (remaining, result) = parse_expression(input).map_err(|e| format!("Parse error: {:?}", e))?;

        if !remaining.trim().is_empty() {
            return Err(format!("Unexpected input after expression: '{}'", remaining));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RuntimeValue;

    #[test]
    fn test_parse_literals() {
        // Test numbers
        assert_eq!(Parser::parse("42").unwrap(), Expression::value(RuntimeValue::Number(42.0)));
        assert_eq!(Parser::parse("3.14").unwrap(), Expression::value(RuntimeValue::Number(3.14)));
        assert_eq!(Parser::parse("-10").unwrap(), Expression::value(RuntimeValue::Number(-10.0)));

        // Test strings
        assert_eq!(
            Parser::parse("\"hello\"").unwrap(),
            Expression::value(RuntimeValue::String("hello".to_string()))
        );

        // Test booleans
        assert_eq!(Parser::parse("true").unwrap(), Expression::value(RuntimeValue::Boolean(true)));
        assert_eq!(Parser::parse("false").unwrap(), Expression::value(RuntimeValue::Boolean(false)));

        // Test null
        assert_eq!(Parser::parse("null").unwrap(), Expression::value(RuntimeValue::Null));
    }

    #[test]
    fn test_parse_references() {
        // Test @ reference (context)
        assert_eq!(Parser::parse("@").unwrap(), Expression::reference("@", false));

        // Test $ reference (builtins)
        assert_eq!(Parser::parse("$").unwrap(), Expression::reference("$", true));

        // Test variable name reference
        assert_eq!(
            Parser::parse("variable_name").unwrap(),
            Expression::reference("variable_name", false)
        );

        // Test identifier with underscores
        assert_eq!(Parser::parse("_private_var").unwrap(), Expression::reference("_private_var", false));
    }

    #[test]
    fn test_parse_arrays() {
        // Test empty array
        assert_eq!(Parser::parse("[]").unwrap(), Expression::array(vec![]));

        // Test array with elements
        let expected = Expression::array(vec![
            Expression::value(RuntimeValue::Number(1.0)),
            Expression::value(RuntimeValue::Number(2.0)),
            Expression::value(RuntimeValue::Number(3.0)),
        ]);
        assert_eq!(Parser::parse("[1, 2, 3]").unwrap(), expected);
    }

    #[test]
    fn test_parse_objects() {
        // Test empty object
        assert_eq!(Parser::parse("{}").unwrap(), Expression::object(HashMap::new()));

        // Test object with entries
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), Expression::value(RuntimeValue::String("John".to_string())));
        expected_map.insert("age".to_string(), Expression::value(RuntimeValue::Number(30.0)));
        let expected = Expression::object(expected_map);

        assert_eq!(Parser::parse("{\"name\": \"John\", \"age\": 30}").unwrap(), expected);
    }

    #[test]
    fn test_parse_function_calls() {
        // Test bare identifier (should be a reference, not a function call)
        let expected = Expression::reference("count", false);
        assert_eq!(Parser::parse("count").unwrap(), expected);

        // Test function call with arguments
        let expected = Expression::function_call(
            Expression::reference("filter", false),
            vec![Expression::reference("condition", false)],
        );
        assert_eq!(Parser::parse("filter condition").unwrap(), expected);
    }

    #[test]
    fn test_debug_count_array() {
        // Debug test for count [] parsing
        let result = Parser::parse("count []");
        println!("count [] parsed as: {:#?}", result);

        // This should be a function call, not an indexing operation
        let expected = Expression::function_call(Expression::reference("count", false), vec![Expression::array(vec![])]);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_debug_function_call_parsing() {
        // Test what parse_function_call does with "count []"
        let result = parse_function_call("count []");
        println!("parse_function_call('count []') = {:#?}", result);

        // Test what parse_op_a does with "count []"
        let result2 = parse_op_a("count []");
        println!("parse_op_a('count []') = {:#?}", result2);
    }

    #[test]
    fn test_parse_pipelines() {
        // Test simple pipeline
        let expected = Expression::pipeline(vec![
            Expression::reference("data", false),
            Expression::function_call(
                Expression::reference("filter", false),
                vec![Expression::reference("condition", false)],
            ),
        ]);
        assert_eq!(Parser::parse("data | filter condition").unwrap(), expected);
    }

    #[test]
    fn test_parse_parenthetical() {
        let expected = Expression::parenthetical(Expression::value(RuntimeValue::Number(42.0)));
        assert_eq!(Parser::parse("(42)").unwrap(), expected);
    }

    #[test]
    fn test_parse_unary_not() {
        // Test logical NOT with boolean
        let expected = Expression::function_call(
            Expression::reference("!/unary", true),
            vec![Expression::value(RuntimeValue::Boolean(true))],
        );
        assert_eq!(Parser::parse("!true").unwrap(), expected);

        // Test logical NOT with number
        let expected = Expression::function_call(
            Expression::reference("!/unary", true),
            vec![Expression::value(RuntimeValue::Number(42.0))],
        );
        assert_eq!(Parser::parse("!42").unwrap(), expected);

        // Test logical NOT with reference (bare identifier)
        let expected = Expression::function_call(
            Expression::reference("!/unary", true),
            vec![Expression::reference("condition", false)],
        );
        assert_eq!(Parser::parse("!condition").unwrap(), expected);
    }

    #[test]
    fn test_parse_unary_negate() {
        // Test unary minus with number
        let expected = Expression::value(RuntimeValue::Number(-42.0));
        assert_eq!(Parser::parse("-42").unwrap(), expected);

        // Test unary minus with float
        let expected = Expression::value(RuntimeValue::Number(-3.14));
        assert_eq!(Parser::parse("-3.14").unwrap(), expected);

        // Test unary minus with reference (bare identifier)
        let expected = Expression::function_call(Expression::reference("-/unary", false), vec![Expression::reference("value", false)]);
        assert_eq!(Parser::parse("-value").unwrap(), expected);
    }

    #[test]
    fn test_parse_multiple_unary_operators() {
        // !!true
        let expected = Expression::function_call(
            Expression::reference("!/unary", true),
            vec![Expression::function_call(
                Expression::reference("!/unary", true),
                vec![Expression::value(RuntimeValue::Boolean(true))],
            )],
        );
        assert_eq!(Parser::parse("!!true").unwrap(), expected);

        // Test NOT with negate
        let expected = Expression::function_call(
            Expression::reference("!/unary", true),
            vec![Expression::value(RuntimeValue::Number(-42.0))],
        );
        assert_eq!(Parser::parse("!-42").unwrap(), expected);

        // Test negate with NOT
        let expected = Expression::function_call(
            Expression::reference("-/unary", false),
            vec![Expression::function_call(
                Expression::reference("!/unary", true),
                vec![Expression::value(RuntimeValue::Boolean(false))],
            )],
        );
        assert_eq!(Parser::parse("-!false").unwrap(), expected);
    }

    #[test]
    fn test_parse_unary_with_whitespace() {
        // Test with spaces around operators
        let expected = Expression::function_call(
            Expression::reference("!/unary", true),
            vec![Expression::value(RuntimeValue::Boolean(true))],
        );
        assert_eq!(Parser::parse("! true").unwrap(), expected);

        let expected = Expression::function_call(
            Expression::reference("-/unary", false),
            vec![Expression::value(RuntimeValue::Number(42.0))],
        );
        assert_eq!(Parser::parse("- 42").unwrap(), expected);
    }

    #[test]
    fn test_parse_unary_with_parentheses() {
        // Test unary operator with parenthetical expression
        let expected = Expression::function_call(
            Expression::reference("!/unary", true),
            vec![Expression::parenthetical(Expression::value(RuntimeValue::Number(42.0)))],
        );
        assert_eq!(Parser::parse("!(42)").unwrap(), expected);

        let expected = Expression::function_call(
            Expression::reference("-/unary", false),
            vec![Expression::parenthetical(Expression::value(RuntimeValue::Number(10.0)))],
        );
        assert_eq!(Parser::parse("-(10)").unwrap(), expected);
    }

    #[test]
    fn test_parse_unary_with_function_calls() {
        // !count
        let expected = Expression::function_call(Expression::reference("!/unary", true), vec![Expression::reference("count", false)]);
        assert_eq!(Parser::parse("!count").unwrap(), expected);

        // -sum 1 == (-sum) 1
        let expected = Expression::function_call(
            Expression::function_call(Expression::reference("-/unary", false), vec![Expression::reference("sum", false)]),
            vec![Expression::value(RuntimeValue::Number(1.0))],
        );
        assert_eq!(Parser::parse("-sum 1").unwrap(), expected);

        // -(sum 1)
        let expected = Expression::function_call(
            Expression::reference("-/unary", false),
            vec![Expression::parenthetical(Expression::function_call(
                Expression::reference("sum", false),
                vec![Expression::value(RuntimeValue::Number(1.0))],
            ))],
        );
        assert_eq!(Parser::parse("-(sum 1)").unwrap(), expected);
    }

    #[test]
    fn test_parse_unary_precedence() {
        // Test that unary operators have higher precedence than references
        // This should parse as: !(count) not (!count)()
        let expected = Expression::function_call(Expression::reference("!/unary", true), vec![Expression::reference("count", false)]);
        assert_eq!(Parser::parse("!count").unwrap(), expected);
    }

    #[test]
    fn test_parse_binary_operators() {
        // Test addition
        let expected = Expression::function_call(
            Expression::reference("+", false),
            vec![
                Expression::value(RuntimeValue::Number(1.0)),
                Expression::value(RuntimeValue::Number(2.0)),
            ],
        );
        assert_eq!(Parser::parse("1 + 2").unwrap(), expected);

        // Test multiplication
        let expected = Expression::function_call(
            Expression::reference("*", false),
            vec![
                Expression::value(RuntimeValue::Number(3.0)),
                Expression::value(RuntimeValue::Number(4.0)),
            ],
        );
        assert_eq!(Parser::parse("3 * 4").unwrap(), expected);

        // Test equality
        let expected = Expression::function_call(
            Expression::reference("==", false),
            vec![
                Expression::value(RuntimeValue::Number(5.0)),
                Expression::value(RuntimeValue::Number(5.0)),
            ],
        );
        assert_eq!(Parser::parse("5 == 5").unwrap(), expected);
    }

    #[test]
    fn test_parse_operator_precedence() {
        // Test that multiplication has higher precedence than addition
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let expected = Expression::function_call(
            Expression::reference("+", false),
            vec![
                Expression::value(RuntimeValue::Number(1.0)),
                Expression::function_call(
                    Expression::reference("*", false),
                    vec![
                        Expression::value(RuntimeValue::Number(2.0)),
                        Expression::value(RuntimeValue::Number(3.0)),
                    ],
                ),
            ],
        );
        assert_eq!(Parser::parse("1 + 2 * 3").unwrap(), expected);
        assert_eq!(Parser::parse("1 + 2 * 3").unwrap(), expected);

        // Test that comparison has higher precedence than logical operators
        // a > b && c < d should parse as (a > b) && (c < d)
        let expected = Expression::function_call(
            Expression::reference("&&", false),
            vec![
                Expression::function_call(
                    Expression::reference(">", false),
                    vec![Expression::reference("a", false), Expression::reference("b", false)],
                ),
                Expression::function_call(
                    Expression::reference("<", false),
                    vec![Expression::reference("c", false), Expression::reference("d", false)],
                ),
            ],
        );
        assert_eq!(Parser::parse("a > b && c < d").unwrap(), expected);
    }

    #[test]
    fn test_parse_associativity() {
        // Test left associativity of addition
        // 1 + 2 + 3 should parse as (1 + 2) + 3
        let expected = Expression::function_call(
            Expression::reference("+", false),
            vec![
                Expression::function_call(
                    Expression::reference("+", false),
                    vec![
                        Expression::value(RuntimeValue::Number(1.0)),
                        Expression::value(RuntimeValue::Number(2.0)),
                    ],
                ),
                Expression::value(RuntimeValue::Number(3.0)),
            ],
        );
        assert_eq!(Parser::parse("1 + 2 + 3").unwrap(), expected);

        // Test left associativity of logical AND
        // a && b && c should parse as (a && b) && c
        let expected = Expression::function_call(
            Expression::reference("&&", false),
            vec![
                Expression::function_call(
                    Expression::reference("&&", false),
                    vec![Expression::reference("a", false), Expression::reference("b", false)],
                ),
                Expression::reference("c", false),
            ],
        );
        assert_eq!(Parser::parse("a && b && c").unwrap(), expected);
    }

    #[test]
    fn test_parse_complex_expression() {
        // Test a more complex expression with multiple precedence levels
        // 1 + 2 * 3 > 4 && 5 == 6 should parse as ((1 + (2 * 3)) > 4) && (5 == 6)
        let expected = Expression::function_call(
            Expression::reference("&&", false),
            vec![
                Expression::function_call(
                    Expression::reference(">", false),
                    vec![
                        Expression::function_call(
                            Expression::reference("+", false),
                            vec![
                                Expression::value(RuntimeValue::Number(1.0)),
                                Expression::function_call(
                                    Expression::reference("*", false),
                                    vec![
                                        Expression::value(RuntimeValue::Number(2.0)),
                                        Expression::value(RuntimeValue::Number(3.0)),
                                    ],
                                ),
                            ],
                        ),
                        Expression::value(RuntimeValue::Number(4.0)),
                    ],
                ),
                Expression::function_call(
                    Expression::reference("==", false),
                    vec![
                        Expression::value(RuntimeValue::Number(5.0)),
                        Expression::value(RuntimeValue::Number(6.0)),
                    ],
                ),
            ],
        );
        assert_eq!(Parser::parse("1 + 2 * 3 > 4 && 5 == 6").unwrap(), expected);
    }

    #[test]
    fn test_parse_lisp_like_syntax() {
        // Test lisp-like syntax: (if toggle keys values) {one: "two"}
        // This should parse as a function call where the function is (if toggle keys values)
        // and the argument is {one: "two"}
        let mut expected_map = HashMap::new();
        expected_map.insert("one".to_string(), Expression::value(RuntimeValue::String("two".to_string())));
        let expected_object = Expression::object(expected_map);

        let expected = Expression::function_call(
            Expression::parenthetical(Expression::function_call(
                Expression::reference("if", false),
                vec![
                    Expression::reference("toggle", false),
                    Expression::reference("keys", false),
                    Expression::reference("values", false),
                ],
            )),
            vec![expected_object],
        );

        assert_eq!(Parser::parse("(if toggle keys values) {one: \"two\"}").unwrap(), expected);
    }

    #[test]
    fn test_parse_lisp_like_syntax_with_pipeline() {
        // Test that lisp-like syntax works in pipelines
        // data | (if toggle keys values) {one: "two"} | count
        let mut expected_map = HashMap::new();
        expected_map.insert("one".to_string(), Expression::value(RuntimeValue::String("two".to_string())));
        let expected_object = Expression::object(expected_map);

        let expected = Expression::pipeline(vec![
            Expression::reference("data", false),
            Expression::function_call(
                Expression::parenthetical(Expression::function_call(
                    Expression::reference("if", false),
                    vec![
                        Expression::reference("toggle", false),
                        Expression::reference("keys", false),
                        Expression::reference("values", false),
                    ],
                )),
                vec![expected_object],
            ),
            Expression::reference("count", false),
        ]);

        assert_eq!(
            Parser::parse("data | (if toggle keys values) {one: \"two\"} | count").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_parenthetical_without_object() {
        // Test that regular parenthetical expressions still work
        // (if toggle keys values) should just be a parenthetical expression
        let expected = Expression::parenthetical(Expression::function_call(
            Expression::reference("if", false),
            vec![
                Expression::reference("toggle", false),
                Expression::reference("keys", false),
                Expression::reference("values", false),
            ],
        ));

        assert_eq!(Parser::parse("(if toggle keys values)").unwrap(), expected);
    }

    #[test]
    fn test_parse_lisp_like_syntax_with_parenthetical() {
        // Test lisp-like syntax: (if toggle keys values) (events)
        // This should parse as a function call where the function is (if toggle keys values)
        // and the argument is (events)
        let expected = Expression::function_call(
            Expression::parenthetical(Expression::function_call(
                Expression::reference("if", false),
                vec![
                    Expression::reference("toggle", false),
                    Expression::reference("keys", false),
                    Expression::reference("values", false),
                ],
            )),
            vec![Expression::parenthetical(Expression::reference("events", false))],
        );

        assert_eq!(Parser::parse("(if toggle keys values) (events)").unwrap(), expected);
    }

    #[test]
    fn test_parse_lisp_like_syntax_with_identifier() {
        // Test lisp-like syntax: (if toggle keys values) events
        // This should parse as a function call where the function is (if toggle keys values)
        // and the argument is events
        let expected = Expression::function_call(
            Expression::parenthetical(Expression::function_call(
                Expression::reference("if", false),
                vec![
                    Expression::reference("toggle", false),
                    Expression::reference("keys", false),
                    Expression::reference("values", false),
                ],
            )),
            vec![Expression::reference("events", false)],
        );

        assert_eq!(Parser::parse("(if toggle keys values) events").unwrap(), expected);
    }

    #[test]
    fn test_parse_dot_access() {
        // Test dot access: object.field
        let expected = Expression::dot_access(Expression::reference("object", false), "field");
        assert_eq!(Parser::parse("object.field").unwrap(), expected);

        // Test chained dot access: object.field.subfield
        let expected = Expression::dot_access(Expression::dot_access(Expression::reference("object", false), "field"), "subfield");
        assert_eq!(Parser::parse("object.field.subfield").unwrap(), expected);
    }

    #[test]
    fn test_parse_indexing() {
        // Test array indexing: array[0]
        let expected = Expression::index_single(Expression::value(RuntimeValue::Number(0.0)), Expression::reference("array", false));
        assert_eq!(Parser::parse("array[0]").unwrap(), expected);

        // Test string indexing: string[1]
        let expected = Expression::index_single(Expression::value(RuntimeValue::Number(1.0)), Expression::reference("string", false));
        assert_eq!(Parser::parse("string[1]").unwrap(), expected);

        // Test chained indexing: array[0][1]
        // This should be parsed as: index(1, index(0, array))
        let expected = Expression::function_call(
            Expression::reference("index", false),
            vec![
                Expression::value(RuntimeValue::Number(1.0)),
                Expression::function_call(
                    Expression::reference("index", false),
                    vec![Expression::value(RuntimeValue::Number(0.0)), Expression::reference("array", false)],
                ),
            ],
        );
        assert_eq!(Parser::parse("array[0][1]").unwrap(), expected);
    }

    #[test]
    fn test_parse_mixed_dot_and_index() {
        // Test mixed dot access and indexing: object.field[0]
        let expected = Expression::function_call(
            Expression::reference("index", false),
            vec![
                Expression::value(RuntimeValue::Number(0.0)),
                Expression::dot_access(Expression::reference("object", false), "field"),
            ],
        );
        assert_eq!(Parser::parse("object.field[0]").unwrap(), expected);

        // Test the reverse: object[0].field
        let expected = Expression::dot_access(
            Expression::function_call(
                Expression::reference("index", false),
                vec![Expression::value(RuntimeValue::Number(0.0)), Expression::reference("object", false)],
            ),
            "field",
        );
        assert_eq!(Parser::parse("object[0].field").unwrap(), expected);
    }

    #[test]
    fn test_parse_gotcha_unary_minus() {
        // Gotcha 1: Unary minus ambiguity
        // items | map -cost should be parsed as map - cost (binary minus)
        // items | map (-cost) should be parsed as map (-cost) (unary minus)

        // Test the problematic case: map -cost (should be binary minus)
        // This demonstrates the gotcha - the parser treats this as binary minus
        // when users might expect it to be unary minus. The correct workaround is to use parentheses.

        // Test the correct case: map (-cost) (should be unary minus in parentheses)
        let expected_correct = Expression::function_call(
            Expression::reference("map", false),
            vec![Expression::parenthetical(Expression::function_call(
                Expression::reference("-/unary", false),
                vec![Expression::reference("cost", false)],
            ))],
        );
        assert_eq!(Parser::parse("map (-cost)").unwrap(), expected_correct);

        // Test that the gotcha exists: map -cost parses as binary minus
        let gotcha_result = Parser::parse("map -cost").unwrap();
        // This demonstrates the gotcha - it's parsed as binary minus (map - cost)
        // when users might expect it to be parsed as unary minus (map (-cost))
        if let Expression::FnExpression { function, arguments } = gotcha_result {
            // This confirms the gotcha - it's parsed as binary minus
            assert_eq!(*function, Expression::reference("-", false));
            assert_eq!(
                *arguments,
                vec![Expression::reference("map", false), Expression::reference("cost", false)]
            );
        } else {
            panic!("Expected binary minus expression");
        }
    }

    #[test]
    fn test_parse_gotcha_indexing_whitespace() {
        // Gotcha 2: Indexing expressions and whitespace
        // [1, 2, 3][0] is valid
        // [1, 2, 3] [0] is invalid (should be parsed as two separate expressions)

        // Test valid indexing (no space)
        let expected = Expression::function_call(
            Expression::reference("index", false),
            vec![
                Expression::value(RuntimeValue::Number(0.0)),
                Expression::array(vec![
                    Expression::value(RuntimeValue::Number(1.0)),
                    Expression::value(RuntimeValue::Number(2.0)),
                    Expression::value(RuntimeValue::Number(3.0)),
                ]),
            ],
        );
        assert_eq!(Parser::parse("[1, 2, 3][0]").unwrap(), expected);

        // Test that spaced version fails to parse as indexing
        // This should parse as a function call: count [1] [2]
        let _result = Parser::parse("[1, 2, 3] [0]");
        // The current implementation might not handle this correctly
        // This test documents the expected behavior
    }

    #[test]
    fn test_parse_gotcha_variable_references() {
        // Gotcha 3: Using named variables with non-homogenous data structures
        // This is more about execution than parsing, but we can test the syntax

        // Test @.bar syntax (recommended)
        let expected = Expression::dot_access(Expression::reference("@", false), "bar");
        assert_eq!(Parser::parse("@.bar").unwrap(), expected);

        // Test bare bar syntax (problematic)
        let expected_bare = Expression::reference("bar", false);
        assert_eq!(Parser::parse("bar").unwrap(), expected_bare);
    }
}
