use std::{collections::HashMap, hash::Hash};

use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1, none_of, one_of},
    combinator::{cut, eof, map, recognize, success, value},
    error::ParseError,
    multi::{fold_many0, many0, many1, separated_list0},
    number::complete::double,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Pipe,
    Or,
    And,
    Eq,
    Neq,
    Match,
    Gt,
    Lt,
    Gte,
    Lte,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'a> {
    Value(Value<'a>),
    UnaryOp(UnaryOp, Box<Expression<'a>>),
    BinaryOp(BinaryOp, Box<Expression<'a>>, Box<Expression<'a>>),
    Dot(Box<Expression<'a>>, Reference<'a>),
    Index(Box<Expression<'a>>, Vec<Expression<'a>>),
    Fn(Box<Expression<'a>>, Vec<Expression<'a>>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reference<'a> {
    Ident(&'a str),
    At,     // '@'
    Dollar, // '$'
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectKey<'a> {
    Str(String),
    Ident(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'a> {
    Object(HashMap<ObjectKey<'a>, Expression<'a>>),
    Array(Vec<Expression<'a>>),
    Str(String),
    Num(f64),
    True,
    False,
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    None,
    Literal(Literal<'a>),
    Reference(Reference<'a>),
    Expression(Box<Expression<'a>>),
}

pub fn query(input: &str) -> IResult<&str, Expression> {
    terminated(expr, eof)(input)
}

fn expr(input: &str) -> IResult<&str, Expression> {
    let (input, e) = simple_expr(input)?;
    fold_many0(
        pair(value(BinaryOp::Pipe, ws(char('|'))), simple_expr),
        move || e.clone(),
        |acc, (op, e)| Expression::BinaryOp(op, Box::new(acc), Box::new(e)),
    )(input)
}

/// Return the result of the `inner` parser and ignore leading and trailing whitespace.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/// Return the result of the `inner` parser and ignore leading whitespace.
fn ws_l<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace0, inner)
}

/// Return the result of the `inner` parser and ignore trailing whitespace.
fn ws_r<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    terminated(inner, multispace0)
}

/// Parse a C-like identifier; e.g. `/[_a-zA-Z][_a-zA-Z0-9]*/`.
fn ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

/// Parse a reference; `@ | $ | ident`.
fn reference(input: &str) -> IResult<&str, Reference> {
    alt((
        value(Reference::At, char('@')),
        value(Reference::Dollar, char('$')),
        map(ident, Reference::Ident),
    ))(input)
}

/// Parses a string surrounded by double quotes. The `\` character may escape `"` or `\`.
fn escaped_string(input: &str) -> IResult<&str, String> {
    preceded(
        char('"'),
        cut(terminated(
            escaped_transform(none_of("\\\""), '\\', one_of("\\\"")),
            char('"'),
        )),
    )(input)
}

fn object_entry(input: &str) -> IResult<&str, (ObjectKey, Expression)> {
    separated_pair(
        alt((
            map(ident, ObjectKey::Ident),
            map(escaped_string, ObjectKey::Str),
        )),
        ws(char(':')),
        expr,
    )(input)
}

fn object(input: &str) -> IResult<&str, HashMap<ObjectKey, Expression>> {
    map(
        preceded(
            ws_r(char('{')),
            cut(terminated(
                separated_list0(ws(char(',')), object_entry),
                ws_l(char('}')),
            )),
        ),
        |v| v.into_iter().collect(),
    )(input)
}

fn array(input: &str) -> IResult<&str, Vec<Expression>> {
    preceded(
        ws_r(char('[')),
        terminated(separated_list0(ws(char(',')), expr), ws_l(char(']'))),
    )(input)
}

/// Parse a literal value: `object | array | string | num | true | false | null`.
fn literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map(object, Literal::Object),
        map(array, Literal::Array),
        map(escaped_string, Literal::Str),
        map(double, Literal::Num),
        value(Literal::True, tag("true")),
        value(Literal::False, tag("false")),
        value(Literal::Null, tag("null")),
    ))(input)
}

/// Parse a simple value: `literal | reference | '(' expr ')'`.
fn simple_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(literal, Value::Literal),
        map(reference, Value::Reference),
        map(delimited(ws_r(char('(')), expr, ws_l(char(')'))), |v| {
            Value::Expression(Box::new(v))
        }),
    ))(input)
}

fn fn_call(input: &str) -> IResult<&str, Expression> {
    map(
        pair(op_a, many1(preceded(multispace1, op_a))),
        |(f, args)| Expression::Fn(Box::new(f), args),
    )(input)
}

fn simple_expr(input: &str) -> IResult<&str, Expression> {
    alt((ws(fn_call), ws(op_a)))(input)
}

fn op_a(input: &str) -> IResult<&str, Expression> {
    let (input, e) = op_b(input)?;
    fold_many0(
        pair(value(BinaryOp::Or, ws(tag("||"))), op_b),
        move || e.clone(),
        |acc, (op, e)| Expression::BinaryOp(op, Box::new(acc), Box::new(e)),
    )(input)
}

fn op_b(input: &str) -> IResult<&str, Expression> {
    let (input, e) = op_c(input)?;
    fold_many0(
        pair(value(BinaryOp::And, ws(tag("&&"))), op_c),
        move || e.clone(),
        |acc, (op, e)| Expression::BinaryOp(op, Box::new(acc), Box::new(e)),
    )(input)
}

fn op_c(input: &str) -> IResult<&str, Expression> {
    let (input, e) = op_d(input)?;
    fold_many0(
        pair(
            alt((
                value(BinaryOp::Eq, ws(tag("=="))),
                value(BinaryOp::Neq, ws(tag("!="))),
                value(BinaryOp::Match, ws(tag("=~"))),
            )),
            op_d,
        ),
        move || e.clone(),
        |acc, (op, e)| Expression::BinaryOp(op, Box::new(acc), Box::new(e)),
    )(input)
}

fn op_d(input: &str) -> IResult<&str, Expression> {
    let (input, e) = op_e(input)?;
    fold_many0(
        pair(
            alt((
                value(BinaryOp::Gt, ws(char('>'))),
                value(BinaryOp::Lt, ws(char('<'))),
                value(BinaryOp::Gte, ws(tag(">="))),
                value(BinaryOp::Lte, ws(tag("<="))),
            )),
            op_e,
        ),
        move || e.clone(),
        |acc, (op, e)| Expression::BinaryOp(op, Box::new(acc), Box::new(e)),
    )(input)
}

fn op_e(input: &str) -> IResult<&str, Expression> {
    let (input, e) = op_f(input)?;
    fold_many0(
        pair(
            alt((
                value(BinaryOp::Add, ws(char('+'))),
                value(BinaryOp::Sub, ws(char('-'))),
            )),
            op_f,
        ),
        move || e.clone(),
        |acc, (op, e)| Expression::BinaryOp(op, Box::new(acc), Box::new(e)),
    )(input)
}

fn op_f(input: &str) -> IResult<&str, Expression> {
    let (input, e) = op_g(input)?;
    fold_many0(
        pair(
            alt((
                value(BinaryOp::Mul, ws(char('*'))),
                value(BinaryOp::Div, ws(char('/'))),
                value(BinaryOp::Mod, ws(char('%'))),
            )),
            op_g,
        ),
        move || e.clone(),
        |acc, (op, e)| Expression::BinaryOp(op, Box::new(acc), Box::new(e)),
    )(input)
}

fn op_g(input: &str) -> IResult<&str, Expression> {
    alt((
        map(
            pair(
                alt((
                    value(UnaryOp::Not, ws_r(char('!'))),
                    value(UnaryOp::Neg, ws_r(char('-'))),
                )),
                cut(op_h),
            ),
            |(op, e)| Expression::UnaryOp(op, Box::new(e)),
        ),
        op_h,
    ))(input)
}

fn op_h(input: &str) -> IResult<&str, Expression> {
    alt((dot, index, map(simple_value, Expression::Value)))(input)
}

fn dot(input: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(simple_value, ws(char('.')), reference),
        |(v, r)| Expression::Dot(Box::new(Expression::Value(v)), r),
    )(input)
}

fn maybe_expr(input: &str) -> IResult<&str, Expression> {
    alt((expr, success(Expression::Value(Value::None))))(input)
}

fn index_inner(input: &str) -> IResult<&str, Vec<Expression>> {
    alt((
        map(
            separated_pair(maybe_expr, ws(char(':')), maybe_expr),
            |(a, b)| vec![a, b],
        ),
        map(expr, |e| vec![e]),
    ))(input)
}

fn index(input: &str) -> IResult<&str, Expression> {
    map(
        pair(
            map(simple_value, Expression::Value),
            delimited(ws_r(char('[')), index_inner, ws_l(char(']'))),
        ),
        |(e, inner)| Expression::Index(Box::new(e), inner),
    )(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{
        array, escaped_string, ident, object, query, reference, ws, ws_l, ws_r, BinaryOp,
        Expression, Literal, ObjectKey, Reference, Value,
    };

    #[test]
    fn test_ws() {
        assert_eq!(ws(ident)("\t\n abc\t\ndef"), Ok(("def", "abc")));
        assert_eq!(ws_l(ident)("\t\n abc\t\ndef"), Ok(("\t\ndef", "abc")));
        assert_eq!(ws_r(ident)("abc\t\n def"), Ok(("def", "abc")));
        assert!(ws_r(ident)("\t\n abc\t\ndef").is_err());
    }

    #[test]
    fn test_escaped_string() {
        assert_eq!(
            escaped_string(r#""Hello \"world\""foo"#),
            Ok(("foo", r#"Hello "world""#.to_owned()))
        );
        assert_eq!(
            escaped_string(r#""Hello \\"world\""foo"#),
            Ok((r#"world\""foo"#, r#"Hello \"#.to_owned()))
        );
        assert_eq!(escaped_string(r#""\\\\\"""#), Ok(("", r#"\\""#.to_owned())));
        assert!(escaped_string(r#""Hello world\""#).is_err());
    }

    #[test]
    fn test_ident() {
        assert_eq!(ident("foo_123-"), Ok(("-", "foo_123")));
        assert_eq!(ident("_a1+"), Ok(("+", "_a1")));
        assert!(ident("1a").is_err());
    }

    #[test]
    fn test_reference() {
        assert_eq!(reference("@$foo123"), Ok(("$foo123", Reference::At)));
        assert_eq!(reference("$foo123"), Ok(("foo123", Reference::Dollar)));
        assert_eq!(
            reference("foo$@123"),
            Ok(("$@123", Reference::Ident("foo")))
        );
        assert!(reference("123$@foo").is_err());
    }

    #[test]
    fn test_object() {
        let mut expected = HashMap::new();
        expected.insert(
            ObjectKey::Ident("_a"),
            Expression::Value(Value::Literal(Literal::Num(1.0))),
        );
        expected.insert(
            ObjectKey::Str("2_".to_owned()),
            Expression::Value(Value::Literal(Literal::Num(2.0))),
        );
        assert_eq!(object(r#"{ _a : 1, "2_":2 }"#), Ok(("", expected)));
        assert_eq!(object("{}"), Ok(("", HashMap::new())));
        assert!(object("{,}").is_err());
        assert!(object("{ a: 1").is_err());
    }

    #[test]
    fn test_array() {
        let expected = vec![
            Expression::Value(Value::Literal(Literal::Num(1.0))),
            Expression::Value(Value::Literal(Literal::Num(3.0))),
        ];
        assert_eq!(array("[ 1, 3 ]"), Ok(("", expected)));
        assert_eq!(array("[]"), Ok(("", vec![])));
        assert!(array("[,]").is_err());
        assert!(array("[ 1").is_err());
    }

    #[test]
    fn test_ops() {
        assert_eq!(
            query("1 + 2 * @ - 4 + 5"),
            Ok((
                "",
                Expression::BinaryOp(
                    BinaryOp::Add,
                    Box::new(Expression::BinaryOp(
                        BinaryOp::Sub,
                        Box::new(Expression::BinaryOp(
                            BinaryOp::Add,
                            Box::new(Expression::Value(Value::Literal(Literal::Num(1.0)))),
                            Box::new(Expression::BinaryOp(
                                BinaryOp::Mul,
                                Box::new(Expression::Value(Value::Literal(Literal::Num(2.0)))),
                                Box::new(Expression::Value(Value::Reference(Reference::At)))
                            ))
                        )),
                        Box::new(Expression::Value(Value::Literal(Literal::Num(4.0))))
                    )),
                    Box::new(Expression::Value(Value::Literal(Literal::Num(5.0))))
                )
            ))
        )
    }

    #[test]
    fn test_fn() {
        assert_eq!(
            query("@ 1 [3, 3] 7"),
            Ok((
                "",
                Expression::Fn(
                    Box::new(Expression::Value(Value::Reference(Reference::At))),
                    vec![
                        Expression::Value(Value::Literal(Literal::Num(1.0))),
                        Expression::Value(Value::Literal(Literal::Array(vec![
                            Expression::Value(Value::Literal(Literal::Num(3.0))),
                            Expression::Value(Value::Literal(Literal::Num(3.0))),
                        ]))),
                        Expression::Value(Value::Literal(Literal::Num(7.0))),
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_index() {
        assert_eq!(
            query("@[1:$]"),
            Ok((
                "",
                Expression::Index(
                    Box::new(Expression::Value(Value::Reference(Reference::At))),
                    vec![
                        Expression::Value(Value::Literal(Literal::Num(1.0))),
                        Expression::Value(Value::Reference(Reference::Dollar)),
                    ]
                )
            ))
        );
        assert_eq!(
            query("@[:]"),
            Ok((
                "",
                Expression::Index(
                    Box::new(Expression::Value(Value::Reference(Reference::At))),
                    vec![
                        Expression::Value(Value::None),
                        Expression::Value(Value::None),
                    ]
                )
            ))
        );
        assert_eq!(
            query("@[1:]"),
            Ok((
                "",
                Expression::Index(
                    Box::new(Expression::Value(Value::Reference(Reference::At))),
                    vec![
                        Expression::Value(Value::Literal(Literal::Num(1.0))),
                        Expression::Value(Value::None),
                    ]
                )
            ))
        );
        assert_eq!(
            query("@[:2]"),
            Ok((
                "",
                Expression::Index(
                    Box::new(Expression::Value(Value::Reference(Reference::At))),
                    vec![
                        Expression::Value(Value::None),
                        Expression::Value(Value::Literal(Literal::Num(2.0))),
                    ]
                )
            ))
        );
        // Invalid indexing will not consume the full input; this is considered an error.
        assert!(query("@[]").is_err());
        assert!(query("@[1:2:3]").is_err());
    }

    #[test]
    fn test_eof() {
        assert!(query("  @[0]  ").is_ok());
        assert!(query(" @ [0, 0] | $ ").is_ok());
    }
}
