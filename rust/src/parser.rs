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
    /// Unary operation: `!expression`, `-expression`
    UnaryExpression {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    /// Binary operation: `left operator right`
    BinaryExpression {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

/// Unary operators in MistQL
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    /// Logical NOT: `!`
    Not,
    /// Unary minus: `-`
    Negate,
}

/// Binary operators in MistQL
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// Logical OR: `||`
    Or,
    /// Logical AND: `&&`
    And,
    /// Equality: `==`
    Eq,
    /// Inequality: `!=`
    Neq,
    /// Regex match: `=~`
    Match,
    /// Greater than: `>`
    Gt,
    /// Less than: `<`
    Lt,
    /// Greater than or equal: `>=`
    Gte,
    /// Less than or equal: `<=`
    Lte,
    /// Addition: `+`
    Plus,
    /// Subtraction: `-`
    Minus,
    /// Multiplication: `*`
    Mul,
    /// Division: `/`
    Div,
    /// Modulo: `%`
    Mod,
}

/// Operator precedence levels (highest to lowest)
/// Based on MistQL documentation and Python grammar
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrecedenceLevel {
    /// Level 1: Dot access (highest precedence)
    Dot = 1,
    /// Level 2: Unary operators (!, -)
    Unary = 2,
    /// Level 3: Multiplication, division, modulo (*, /, %)
    MulDivMod = 3,
    /// Level 4: Addition, subtraction (+, -)
    AddSub = 4,
    /// Level 5: Comparison (<, >, <=, >=)
    Comparison = 5,
    /// Level 6: Equality (==, !=, =~)
    Equality = 6,
    /// Level 7: Logical AND (&&)
    LogicalAnd = 7,
    /// Level 8: Logical OR (||)
    LogicalOr = 8,
    /// Level 9: Function application
    FunctionApplication = 9,
    /// Level 10: Pipeline (|) (lowest precedence)
    Pipeline = 10,
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

    /// Create a unary expression
    pub fn unary(operator: UnaryOperator, operand: Expression) -> Self {
        Expression::UnaryExpression {
            operator,
            operand: Box::new(operand),
        }
    }

    /// Create a binary expression
    pub fn binary(operator: BinaryOperator, left: Expression, right: Expression) -> Self {
        Expression::BinaryExpression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
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

/// Parse a pipeline expression (lowest precedence: |)
/// op_a: op_b | op_a "||" op_b
fn parse_pipeline(input: &str) -> IResult<&str, Expression> {
    map(
        separated_list1(
            pair(ws, char('|')),
            pair(ws, parse_logical_or),
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

/// Parse logical OR expressions (||)
/// op_a: op_b | op_a "||" op_b
fn parse_logical_or(input: &str) -> IResult<&str, Expression> {
    map(
        separated_list1(
            pair(ws, tag("||")),
            pair(ws, parse_logical_and),
        ),
        |operands| {
            let operands: Vec<Expression> = operands.into_iter().map(|(_, operand)| operand).collect();
            if operands.len() == 1 {
                operands.into_iter().next().unwrap()
            } else {
                // Left-associative: a || b || c = (a || b) || c
                operands.into_iter().reduce(|left, right| {
                    Expression::binary(BinaryOperator::Or, left, right)
                }).unwrap()
            }
        },
    )(input)
}

/// Parse logical AND expressions (&&)
/// op_b: op_c | op_b "&&" op_c
fn parse_logical_and(input: &str) -> IResult<&str, Expression> {
    map(
        separated_list1(
            pair(ws, tag("&&")),
            pair(ws, parse_equality),
        ),
        |operands| {
            let operands: Vec<Expression> = operands.into_iter().map(|(_, operand)| operand).collect();
            if operands.len() == 1 {
                operands.into_iter().next().unwrap()
            } else {
                // Left-associative: a && b && c = (a && b) && c
                operands.into_iter().reduce(|left, right| {
                    Expression::binary(BinaryOperator::And, left, right)
                }).unwrap()
            }
        },
    )(input)
}

/// Parse equality expressions (==, !=, =~)
/// op_c: op_d | op_c ("==" | "!=" | "=~") op_d
fn parse_equality(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_comparison,
            many0(pair(
                pair(ws, alt((
                    map(tag("=="), |_| BinaryOperator::Eq),
                    map(tag("!="), |_| BinaryOperator::Neq),
                    map(tag("=~"), |_| BinaryOperator::Match),
                ))),
                pair(ws, parse_comparison),
            )),
        ),
        |(left, rest)| {
            rest.into_iter().fold(left, |left, ((_, operator), (_, right))| {
                Expression::binary(operator, left, right)
            })
        },
    )(input)
}

/// Parse comparison expressions (<, >, <=, >=)
/// op_d: op_e | op_d (">" | "<" | ">=" | "<=") op_e
fn parse_comparison(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_add_sub,
            many0(pair(
                pair(ws, alt((
                    map(tag(">"), |_| BinaryOperator::Gt),
                    map(tag("<"), |_| BinaryOperator::Lt),
                    map(tag(">="), |_| BinaryOperator::Gte),
                    map(tag("<="), |_| BinaryOperator::Lte),
                ))),
                pair(ws, parse_add_sub),
            )),
        ),
        |(left, rest)| {
            rest.into_iter().fold(left, |left, ((_, operator), (_, right))| {
                Expression::binary(operator, left, right)
            })
        },
    )(input)
}

/// Parse addition and subtraction expressions (+, -)
/// op_e: op_f | op_e ("+" | "-") op_f
fn parse_add_sub(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_mul_div_mod,
            many0(pair(
                pair(ws, alt((
                    map(tag("+"), |_| BinaryOperator::Plus),
                    map(tag("-"), |_| BinaryOperator::Minus),
                ))),
                pair(ws, parse_mul_div_mod),
            )),
        ),
        |(left, rest)| {
            rest.into_iter().fold(left, |left, ((_, operator), (_, right))| {
                Expression::binary(operator, left, right)
            })
        },
    )(input)
}

/// Parse multiplication, division, and modulo expressions (*, /, %)
/// op_f: op_g | op_f ("*" | "/" | "%") op_g
fn parse_mul_div_mod(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            parse_unary_expression,
            many0(pair(
                pair(ws, alt((
                    map(tag("*"), |_| BinaryOperator::Mul),
                    map(tag("/"), |_| BinaryOperator::Div),
                    map(tag("%"), |_| BinaryOperator::Mod),
                ))),
                pair(ws, parse_unary_expression),
            )),
        ),
        |(left, rest)| {
            rest.into_iter().fold(left, |left, ((_, operator), (_, right))| {
                Expression::binary(operator, left, right)
            })
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


/// Parse a unary expression (!expression or -expression)
fn parse_unary_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        // Parse logical NOT: !expression (with optional whitespace)
        map(
            pair(
                char('!'),
                pair(ws, parse_unary_expression), // Recursive to handle multiple unary operators
            ),
            |(_, (_, operand))| Expression::unary(UnaryOperator::Not, operand),
        ),
        // Parse unary minus: -expression (with optional whitespace)
        map(
            pair(
                char('-'),
                pair(ws, parse_unary_expression), // Recursive to handle multiple unary operators
            ),
            |(_, (_, operand))| Expression::unary(UnaryOperator::Negate, operand),
        ),
        // Fall back to function calls (which will handle primary expressions)
        parse_function_call,
    ))(input)
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
            Expression::unary(
                UnaryOperator::Negate,
                Expression::value(RuntimeValue::Number(10.0))
            )
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

    #[test]
    fn test_parse_unary_not() {
        // Test logical NOT with boolean
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::value(RuntimeValue::Boolean(true))
        );
        assert_eq!(
            Parser::parse("!true").unwrap(),
            expected
        );

        // Test logical NOT with number
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::value(RuntimeValue::Number(42.0))
        );
        assert_eq!(
            Parser::parse("!42").unwrap(),
            expected
        );

        // Test logical NOT with function call (bare identifier)
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::function_call(
                Expression::reference("condition", false),
                vec![]
            )
        );
        assert_eq!(
            Parser::parse("!condition").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_unary_negate() {
        // Test unary minus with number
        let expected = Expression::unary(
            UnaryOperator::Negate,
            Expression::value(RuntimeValue::Number(42.0))
        );
        assert_eq!(
            Parser::parse("-42").unwrap(),
            expected
        );

        // Test unary minus with float
        let expected = Expression::unary(
            UnaryOperator::Negate,
            Expression::value(RuntimeValue::Number(3.14))
        );
        assert_eq!(
            Parser::parse("-3.14").unwrap(),
            expected
        );

        // Test unary minus with function call (bare identifier)
        let expected = Expression::unary(
            UnaryOperator::Negate,
            Expression::function_call(
                Expression::reference("value", false),
                vec![]
            )
        );
        assert_eq!(
            Parser::parse("-value").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_multiple_unary_operators() {
        // Test double negation
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::unary(
                UnaryOperator::Not,
                Expression::value(RuntimeValue::Boolean(true))
            )
        );
        assert_eq!(
            Parser::parse("!!true").unwrap(),
            expected
        );

        // Test NOT with negate
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::unary(
                UnaryOperator::Negate,
                Expression::value(RuntimeValue::Number(42.0))
            )
        );
        assert_eq!(
            Parser::parse("!-42").unwrap(),
            expected
        );

        // Test negate with NOT
        let expected = Expression::unary(
            UnaryOperator::Negate,
            Expression::unary(
                UnaryOperator::Not,
                Expression::value(RuntimeValue::Boolean(false))
            )
        );
        assert_eq!(
            Parser::parse("-!false").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_unary_with_whitespace() {
        // Test with spaces around operators
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::value(RuntimeValue::Boolean(true))
        );
        assert_eq!(
            Parser::parse("! true").unwrap(),
            expected
        );

        let expected = Expression::unary(
            UnaryOperator::Negate,
            Expression::value(RuntimeValue::Number(42.0))
        );
        assert_eq!(
            Parser::parse("- 42").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_unary_with_parentheses() {
        // Test unary operator with parenthetical expression
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::parenthetical(
                Expression::value(RuntimeValue::Number(42.0))
            )
        );
        assert_eq!(
            Parser::parse("!(42)").unwrap(),
            expected
        );

        let expected = Expression::unary(
            UnaryOperator::Negate,
            Expression::parenthetical(
                Expression::value(RuntimeValue::Number(10.0))
            )
        );
        assert_eq!(
            Parser::parse("-(10)").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_unary_with_function_calls() {
        // Test unary operator with function call
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::function_call(
                Expression::reference("count", false),
                vec![]
            )
        );
        assert_eq!(
            Parser::parse("!count").unwrap(),
            expected
        );

        // Test unary operator with function call that has arguments
        let expected = Expression::unary(
            UnaryOperator::Negate,
            Expression::function_call(
                Expression::reference("sum", false),
                vec![Expression::value(RuntimeValue::Number(1.0))]
            )
        );
        assert_eq!(
            Parser::parse("-sum 1").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_unary_precedence() {
        // Test that unary operators have higher precedence than function calls
        // This should parse as: !(count) not (!count)()
        let expected = Expression::unary(
            UnaryOperator::Not,
            Expression::function_call(
                Expression::reference("count", false),
                vec![]
            )
        );
        assert_eq!(
            Parser::parse("!count").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_binary_operators() {
        // Test addition
        let expected = Expression::binary(
            BinaryOperator::Plus,
            Expression::value(RuntimeValue::Number(1.0)),
            Expression::value(RuntimeValue::Number(2.0))
        );
        assert_eq!(
            Parser::parse("1 + 2").unwrap(),
            expected
        );

        // Test multiplication
        let expected = Expression::binary(
            BinaryOperator::Mul,
            Expression::value(RuntimeValue::Number(3.0)),
            Expression::value(RuntimeValue::Number(4.0))
        );
        assert_eq!(
            Parser::parse("3 * 4").unwrap(),
            expected
        );

        // Test equality
        let expected = Expression::binary(
            BinaryOperator::Eq,
            Expression::value(RuntimeValue::Number(5.0)),
            Expression::value(RuntimeValue::Number(5.0))
        );
        assert_eq!(
            Parser::parse("5 == 5").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_operator_precedence() {
        // Test that multiplication has higher precedence than addition
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let expected = Expression::binary(
            BinaryOperator::Plus,
            Expression::value(RuntimeValue::Number(1.0)),
            Expression::binary(
                BinaryOperator::Mul,
                Expression::value(RuntimeValue::Number(2.0)),
                Expression::value(RuntimeValue::Number(3.0))
            )
        );
        assert_eq!(
            Parser::parse("1 + 2 * 3").unwrap(),
            expected
        );

        // Test that comparison has higher precedence than logical operators
        // a > b && c < d should parse as (a > b) && (c < d)
        let expected = Expression::binary(
            BinaryOperator::And,
            Expression::binary(
                BinaryOperator::Gt,
                Expression::function_call(
                    Expression::reference("a", false),
                    vec![]
                ),
                Expression::function_call(
                    Expression::reference("b", false),
                    vec![]
                )
            ),
            Expression::binary(
                BinaryOperator::Lt,
                Expression::function_call(
                    Expression::reference("c", false),
                    vec![]
                ),
                Expression::function_call(
                    Expression::reference("d", false),
                    vec![]
                )
            )
        );
        assert_eq!(
            Parser::parse("a > b && c < d").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_associativity() {
        // Test left associativity of addition
        // 1 + 2 + 3 should parse as (1 + 2) + 3
        let expected = Expression::binary(
            BinaryOperator::Plus,
            Expression::binary(
                BinaryOperator::Plus,
                Expression::value(RuntimeValue::Number(1.0)),
                Expression::value(RuntimeValue::Number(2.0))
            ),
            Expression::value(RuntimeValue::Number(3.0))
        );
        assert_eq!(
            Parser::parse("1 + 2 + 3").unwrap(),
            expected
        );

        // Test left associativity of logical AND
        // a && b && c should parse as (a && b) && c
        let expected = Expression::binary(
            BinaryOperator::And,
            Expression::binary(
                BinaryOperator::And,
                Expression::function_call(
                    Expression::reference("a", false),
                    vec![]
                ),
                Expression::function_call(
                    Expression::reference("b", false),
                    vec![]
                )
            ),
            Expression::function_call(
                Expression::reference("c", false),
                vec![]
            )
        );
        assert_eq!(
            Parser::parse("a && b && c").unwrap(),
            expected
        );
    }

    #[test]
    fn test_parse_complex_expression() {
        // Test a more complex expression with multiple precedence levels
        // 1 + 2 * 3 > 4 && 5 == 6 should parse as ((1 + (2 * 3)) > 4) && (5 == 6)
        let expected = Expression::binary(
            BinaryOperator::And,
            Expression::binary(
                BinaryOperator::Gt,
                Expression::binary(
                    BinaryOperator::Plus,
                    Expression::value(RuntimeValue::Number(1.0)),
                    Expression::binary(
                        BinaryOperator::Mul,
                        Expression::value(RuntimeValue::Number(2.0)),
                        Expression::value(RuntimeValue::Number(3.0))
                    )
                ),
                Expression::value(RuntimeValue::Number(4.0))
            ),
            Expression::binary(
                BinaryOperator::Eq,
                Expression::value(RuntimeValue::Number(5.0)),
                Expression::value(RuntimeValue::Number(6.0))
            )
        );
        assert_eq!(
            Parser::parse("1 + 2 * 3 > 4 && 5 == 6").unwrap(),
            expected
        );
    }
}