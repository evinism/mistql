//! Parser implementation for MistQL syntax
//!
//! This module implements a parser for MistQL expressions using nom parser combinators.
//! It converts MistQL syntax into an Abstract Syntax Tree (AST) that can be executed.

use crate::types::RuntimeValue;
use std::collections::HashMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1},
    combinator::{map, opt, recognize, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};

/// AST expression types for MistQL
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Function call: `function arg1 arg2`
    FnExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    /// Variable reference: `@`, `$`, `variable_name`
    RefExpression {
        name: String,
        absolute: bool,
    },
    /// Literal value: `42`, `"hello"`, `true`, `null`
    ValueExpression {
        value: RuntimeValue,
    },
    /// Array literal: `[1, 2, 3]`
    ArrayExpression {
        items: Vec<Expression>,
    },
    /// Object literal: `{"key": "value"}`
    ObjectExpression {
        entries: HashMap<String, Expression>,
    },
    /// Pipeline: `data | filter condition | map field`
    PipeExpression {
        stages: Vec<Expression>,
    },
    /// Parenthetical expression: `(expression)`
    ParentheticalExpression {
        expression: Box<Expression>,
    },
}

impl Expression {
    /// Create a reference expression
    pub fn reference(name: &str, absolute: bool) -> Self {
        Expression::RefExpression {
            name: name.to_string(),
            absolute,
        }
    }

    /// Create a value expression
    pub fn value(value: RuntimeValue) -> Self {
        Expression::ValueExpression { value }
    }

    /// Create a function call expression
    pub fn function_call(function: Expression, arguments: Vec<Expression>) -> Self {
        Expression::FnExpression {
            function: Box::new(function),
            arguments,
        }
    }

    /// Create a pipeline expression
    pub fn pipeline(stages: Vec<Expression>) -> Self {
        Expression::PipeExpression { stages }
    }

    /// Create an array expression
    pub fn array(items: Vec<Expression>) -> Self {
        Expression::ArrayExpression { items }
    }

    /// Create an object expression
    pub fn object(entries: HashMap<String, Expression>) -> Self {
        Expression::ObjectExpression { entries }
    }

    /// Create a parenthetical expression
    pub fn parenthetical(expression: Expression) -> Self {
        Expression::ParentheticalExpression {
            expression: Box::new(expression),
        }
    }
}

/// Parse whitespace
fn ws(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

/// Parse whitespace with at least one space
fn ws1(input: &str) -> IResult<&str, &str> {
    multispace1(input)
}

/// Parse a float number (with decimal point)
fn parse_float(input: &str) -> IResult<&str, &str> {
    recognize(pair(digit1, pair(char('.'), digit1)))(input)
}

/// Parse an integer number
fn parse_integer(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

/// Parse a number (integer or float)
fn parse_number(input: &str) -> IResult<&str, RuntimeValue> {
    map(
        recognize(pair(
            opt(char('-')),
            alt((parse_float, parse_integer)),
        )),
        |s: &str| {
            s.parse::<f64>()
                .map(RuntimeValue::Number)
                .unwrap_or(RuntimeValue::Number(0.0))
        },
    )(input)
}

/// Parse a string literal
fn parse_string(input: &str) -> IResult<&str, RuntimeValue> {
    map(
        delimited(
            char('"'),
            take_while(|c| c != '"'),
            char('"'),
        ),
        |s: &str| RuntimeValue::String(s.to_string()),
    )(input)
}

/// Parse boolean literals
fn parse_boolean(input: &str) -> IResult<&str, RuntimeValue> {
    alt((
        value(RuntimeValue::Boolean(true), tag("true")),
        value(RuntimeValue::Boolean(false), tag("false")),
    ))(input)
}

/// Parse null literal
fn parse_null(input: &str) -> IResult<&str, RuntimeValue> {
    value(RuntimeValue::Null, tag("null"))(input)
}

/// Parse a literal value
fn parse_literal(input: &str) -> IResult<&str, Expression> {
    map(
        alt((
            parse_number,
            parse_string,
            parse_boolean,
            parse_null,
        )),
        Expression::value,
    )(input)
}

/// Parse a variable name (identifier)
fn parse_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

/// Parse a reference (@ or $)
fn parse_reference(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("@"), |_| Expression::reference("@", false)),
        map(tag("$"), |_| Expression::reference("$", true)),
    ))(input)
}

/// Parse an array literal
fn parse_array(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            pair(char('['), ws),
            separated_list0(
                pair(ws, char(',')),
                pair(ws, parse_expression),
            ),
            pair(ws, char(']')),
        ),
        |items| Expression::array(items.into_iter().map(|(_, expr)| expr).collect()),
    )(input)
}

/// Parse an object literal
fn parse_object(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            pair(char('{'), ws),
            separated_list0(
                pair(ws, char(',')),
                pair(ws, parse_object_entry),
            ),
            pair(ws, char('}')),
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

/// Parse an object entry (key: value)
fn parse_object_entry(input: &str) -> IResult<&str, (String, Expression)> {
    separated_pair(
        alt((
            map(parse_string, |rv| match rv {
                RuntimeValue::String(s) => s,
                _ => unreachable!(),
            }),
            map(parse_identifier, |s| s.to_string()),
        )),
        pair(ws, char(':')),
        parse_expression,
    )(input)
}

/// Parse a parenthetical expression
fn parse_parenthetical(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            pair(char('('), ws),
            parse_pipeline,
            pair(ws, char(')')),
        ),
        Expression::parenthetical,
    )(input)
}


/// Parse a function call
fn parse_function_call(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_primary,
            many0(pair(ws1, parse_primary)),
        ),
        |(function, args)| {
            let arguments: Vec<Expression> = args.into_iter().map(|(_, arg)| arg).collect();
            // Check if the function is a function name (identifier) or something else
            match &function {
                Expression::RefExpression { name, absolute: false } if name != "@" && name != "$" => {
                    // This is a function name, always create a function call
                    Expression::function_call(function, arguments)
                }
                _ => {
                    // This is a literal, reference, array, object, or parenthetical
                    if arguments.is_empty() {
                        function
                    } else {
                        Expression::function_call(function, arguments)
                    }
                }
            }
        },
    )(input)
}

/// Parse a pipeline expression
fn parse_pipeline(input: &str) -> IResult<&str, Expression> {
    map(
        separated_list1(
            pair(ws, char('|')),
            pair(ws, parse_function_call),
        ),
        |stages| {
            let stages: Vec<Expression> = stages.into_iter().map(|(_, stage)| stage).collect();
            if stages.len() == 1 {
                stages.into_iter().next().unwrap()
            } else {
                Expression::pipeline(stages)
            }
        },
    )(input)
}

/// Parse a complete expression (top-level entry point)
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_pipeline(input)
}

/// Parse a function name (identifier)
fn parse_function_name(input: &str) -> IResult<&str, Expression> {
    map(parse_identifier, |s| Expression::reference(s, false))(input)
}

/// Parse a primary expression (literals, references, function names, arrays, objects, parenthetical)
fn parse_primary(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_literal,
        parse_reference,
        parse_function_name,
        parse_array,
        parse_object,
        parse_parenthetical,
    ))(input)
}

/// Parser for MistQL expressions
pub struct Parser;

impl Parser {
    /// Parse a MistQL expression string into an AST
    pub fn parse(input: &str) -> Result<Expression, String> {
        let (remaining, result) = parse_expression(input)
            .map_err(|e| format!("Parse error: {:?}", e))?;

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
        assert_eq!(
            Parser::parse("42").unwrap(),
            Expression::value(RuntimeValue::Number(42.0))
        );
        assert_eq!(
            Parser::parse("3.14").unwrap(),
            Expression::value(RuntimeValue::Number(3.14))
        );
        assert_eq!(
            Parser::parse("-10").unwrap(),
            Expression::value(RuntimeValue::Number(-10.0))
        );

        // Test strings
        assert_eq!(
            Parser::parse("\"hello\"").unwrap(),
            Expression::value(RuntimeValue::String("hello".to_string()))
        );

        // Test booleans
        assert_eq!(
            Parser::parse("true").unwrap(),
            Expression::value(RuntimeValue::Boolean(true))
        );
        assert_eq!(
            Parser::parse("false").unwrap(),
            Expression::value(RuntimeValue::Boolean(false))
        );

        // Test null
        assert_eq!(
            Parser::parse("null").unwrap(),
            Expression::value(RuntimeValue::Null)
        );
    }

    #[test]
    fn test_parse_references() {
        // Test @ reference
        assert_eq!(
            Parser::parse("@").unwrap(),
            Expression::reference("@", false)
        );

        // Test $ reference
        assert_eq!(
            Parser::parse("$").unwrap(),
            Expression::reference("$", true)
        );
    }

    #[test]
    fn test_parse_arrays() {
        // Test empty array
        assert_eq!(
            Parser::parse("[]").unwrap(),
            Expression::array(vec![])
        );

        // Test array with elements
        let expected = Expression::array(vec![
            Expression::value(RuntimeValue::Number(1.0)),
            Expression::value(RuntimeValue::Number(2.0)),
            Expression::value(RuntimeValue::Number(3.0)),
        ]);
        assert_eq!(
            Parser::parse("[1, 2, 3]").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_objects() {
        // Test empty object
        assert_eq!(
            Parser::parse("{}").unwrap(),
            Expression::object(HashMap::new())
        );

        // Test object with entries
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), Expression::value(RuntimeValue::String("John".to_string())));
        expected_map.insert("age".to_string(), Expression::value(RuntimeValue::Number(30.0)));
        let expected = Expression::object(expected_map);

        assert_eq!(
            Parser::parse("{\"name\": \"John\", \"age\": 30}").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_function_calls() {
        // Test simple function call
        let expected = Expression::function_call(
            Expression::reference("count", false),
            vec![]
        );
        assert_eq!(
            Parser::parse("count").unwrap(),
            expected
        );

        // Test function call with arguments
        let expected = Expression::function_call(
            Expression::reference("filter", false),
            vec![Expression::reference("condition", false)]
        );
        assert_eq!(
            Parser::parse("filter condition").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_pipelines() {
        // Test simple pipeline
        let expected = Expression::pipeline(vec![
            Expression::function_call(
                Expression::reference("data", false),
                vec![]
            ),
            Expression::function_call(
                Expression::reference("filter", false),
                vec![Expression::reference("condition", false)]
            ),
        ]);
        assert_eq!(
            Parser::parse("data | filter condition").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_parenthetical() {
        let expected = Expression::parenthetical(
            Expression::value(RuntimeValue::Number(42.0))
        );
        assert_eq!(
            Parser::parse("(42)").unwrap(),
            expected
        );
    }

    // TODO: Implement operators before enabling this test
    // #[test]
    // fn test_parse_complex_expression() {
    //     // Test a more complex expression
    //     let result = Parser::parse("[1, 2, 3] | filter @ > 1 | count");
    //     assert!(result.is_ok());
    // }
}