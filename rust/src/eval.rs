use std::fmt;

use crate::parse::{BinaryOp, Expression, Literal, ObjectKey, Reference, UnaryOp, Value};
use crate::MistQLError;

impl<'a> Expression<'a> {
    pub fn evaluate(
        &'a self,
        context: &serde_json::Value,
    ) -> Result<serde_json::Value, MistQLError> {
        match self {
            Self::Value(value) => value.evaluate(&context),
            Self::UnaryOp(op, expression) => op.evaluate(expression.evaluate(&context)?),
            Self::BinaryOp(op, left, right) => {
                op.evaluate(left.evaluate(&context)?, right.evaluate(&context)?)
            }
            Self::Fn(func, args) => apply_fn(func, args, context),
            _ => Err(MistQLError::UnimplementedEvaluation(format!(
                "expression type {:?}",
                self
            ))),
        }
    }
}

impl<'a> Value<'a> {
    pub fn evaluate(
        &'a self,
        context: &serde_json::Value,
    ) -> Result<serde_json::Value, MistQLError> {
        match self {
            Self::Reference(reference) => reference.evaluate(&context),
            Self::Literal(literal) => literal.evaluate(),
            _ => Err(MistQLError::UnimplementedEvaluation(format!(
                "value type {:?}",
                self
            ))),
        }
    }
}

impl<'a> Reference<'a> {
    pub fn evaluate(
        &'a self,
        context: &'a serde_json::Value,
    ) -> Result<serde_json::Value, MistQLError> {
        match self {
            Self::At => Ok(context.clone()),
            Self::Ident(str) => Ok(str.clone().into()),
            _ => Err(MistQLError::UnimplementedEvaluation(format!(
                "reference type {:?}",
                self
            ))),
        }
    }
}

impl<'a> fmt::Display for ObjectKey<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Str(str) => write!(f, "{}", str),
            Self::Ident(str) => write!(f, "{}", str),
        }
    }
}

impl UnaryOp {
    pub fn evaluate(self, arg: serde_json::Value) -> Result<serde_json::Value, MistQLError> {
        match self {
            Self::Neg => {
                if let Some(num) = arg.as_i64() {
                    Ok((num * -1).into())
                } else if let Some(num) = arg.as_f64() {
                    Ok((num * -1.0).into())
                } else {
                    Err(MistQLError::ArgumentError(
                        "Negation only applies to numbers".to_string(),
                    ))
                }
            }
            _ => Err(MistQLError::UnimplementedEvaluation(format!(
                "UnaryOp type {:?}",
                self
            ))),
        }
    }
}

impl BinaryOp {
    pub fn evaluate(
        self,
        left: serde_json::Value,
        right: serde_json::Value,
    ) -> Result<serde_json::Value, MistQLError> {
        match self {
            Self::Add => match (left.as_i64(), left.as_f64(), right.as_i64(), right.as_f64()) {
                (Some(left_num), _, Some(right_num), _) => Ok((left_num + right_num).into()),
                (Some(left_num), _, _, Some(right_num)) => Ok((left_num as f64 + right_num).into()),
                (_, Some(left_num), _, Some(right_num)) => Ok((left_num + right_num).into()),
                (_, Some(left_num), Some(right_num), _) => Ok((left_num + right_num as f64).into()),
                (_, _, _, _) => Err(MistQLError::ArgumentError(
                    "unsupported types for addition".to_string(),
                )),
            },
            Self::Sub => match (left.as_i64(), left.as_f64(), right.as_i64(), right.as_f64()) {
                (Some(left_num), _, Some(right_num), _) => Ok((left_num - right_num).into()),
                (Some(left_num), _, _, Some(right_num)) => Ok((left_num as f64 - right_num).into()),
                (_, Some(left_num), _, Some(right_num)) => Ok((left_num - right_num).into()),
                (_, Some(left_num), Some(right_num), _) => Ok((left_num - right_num as f64).into()),
                (_, _, _, _) => Err(MistQLError::ArgumentError(
                    "unsupported types for subtraction".to_string(),
                )),
            },
            Self::Mul => match (left.as_i64(), left.as_f64(), right.as_i64(), right.as_f64()) {
                (Some(left_num), _, Some(right_num), _) => Ok((left_num * right_num).into()),
                (Some(left_num), _, _, Some(right_num)) => Ok((left_num as f64 * right_num).into()),
                (_, Some(left_num), _, Some(right_num)) => Ok((left_num * right_num).into()),
                (_, Some(left_num), Some(right_num), _) => Ok((left_num * right_num as f64).into()),
                (_, _, _, _) => Err(MistQLError::ArgumentError(
                    "unsupported types for multiplication".to_string(),
                )),
            },
            Self::Div => match (left.as_i64(), left.as_f64(), right.as_i64(), right.as_f64()) {
                (Some(left_num), _, Some(right_num), _) => Ok((left_num / right_num).into()),
                (Some(left_num), _, _, Some(right_num)) => Ok((left_num as f64 / right_num).into()),
                (_, Some(left_num), _, Some(right_num)) => Ok((left_num / right_num).into()),
                (_, Some(left_num), Some(right_num), _) => Ok((left_num / right_num as f64).into()),
                (_, _, _, _) => Err(MistQLError::ArgumentError(
                    "unsupported types for division".to_string(),
                )),
            },
            _ => Err(MistQLError::UnimplementedEvaluation(format!(
                "BinaryOp type {:?}",
                self
            ))),
        }
    }
}

impl<'a> Literal<'a> {
    pub fn evaluate(&'a self) -> Result<serde_json::Value, MistQLError> {
        match self {
            Literal::Num(num) if num.fract() == 0.0 => Ok((*num as i32).into()),
            Literal::Num(num) => Ok(num.clone().into()),
            Literal::Str(str) => Ok(str.clone().into()),
            Literal::True => Ok(true.into()),
            Literal::False => Ok(false.into()),
            Literal::Null => Ok(serde_json::Value::Null),
            Literal::Array(array) => array
                .iter()
                .map(|elt| elt.evaluate(&serde_json::Value::Null))
                .collect(),
            Literal::Object(obj) => {
                let mut map = serde_json::Map::new();
                // this is gross but it's hard to deal with Result inside an iterator
                for (raw_key, raw_val) in obj.iter() {
                    match raw_val.evaluate(&serde_json::Value::Null) {
                        Ok(val) => {
                            map.insert(raw_key.to_string(), val);
                        }
                        Err(err) => return Err(err),
                    }
                }
                Ok(map.into())
            }
        }
    }
}

enum Function {
    Count,
    String,
}

// TODO convert this into impl TryFrom<Expression> on Function
fn fn_from_expression(func_expr: &Expression) -> Result<Function, MistQLError> {
    let func_name: &str = match func_expr {
        Expression::Value(Value::Reference(Reference::Ident(name))) => Ok(name),
        _ => Err(MistQLError::QueryParseError(
            "function name must be an identifier".to_string(),
        )),
    }?;

    match func_name {
        "count" => Ok(Function::Count),
        "string" => Ok(Function::String),
        _ => Err(MistQLError::UnimplementedFunction(func_name.to_string())),
    }
}

fn apply_fn(
    func_expr: &Expression,
    args_expr: &Vec<Expression>,
    context: &serde_json::Value,
) -> Result<serde_json::Value, MistQLError> {
    // get the function name as a &str
    let func = fn_from_expression(func_expr)?;

    // ensure all of the args are valid before we proceed
    let args = args_expr
        .iter()
        .map(|arg| arg.evaluate(context))
        .collect::<Result<Vec<serde_json::Value>, MistQLError>>()?;

    match func {
        Function::Count => match args.get(0) {
            None => Err(MistQLError::ArgumentError(
                "count takes one argument".to_string(),
            )),
            Some(serde_json::Value::Array(arr)) => Ok(arr.len().into()),
            Some(serde_json::Value::Object(obj)) => Ok(obj.iter().count().into()),
            _ => Err(MistQLError::ArgumentError(
                "count must take an object or an array".to_string(),
            )),
        },
        Function::String => match args.get(0) {
            None => Err(MistQLError::ArgumentError(
                "count takes one argument".to_string(),
            )),
            Some(arg) => Ok(arg.to_string().into()),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::*;

    use serde_json::json;

    #[test]
    fn at_reflects_context() {
        let ast = Reference::At;
        let data = json!({
            "foo": "bar"
        });

        assert_eq!(ast.evaluate(&data).unwrap(), data)
    }

    #[test]
    fn numeric_int_literals() {
        let ast = Literal::Num(8675309.0);

        assert_eq!(ast.evaluate().unwrap(), json!(8675309))
    }

    #[test]
    fn numeric_float_literals() {
        let ast = Literal::Num(867.5309);

        assert_eq!(ast.evaluate().unwrap(), json!(867.5309))
    }

    #[test]
    fn negative_double_literals() {
        let ast = Expression::UnaryOp(
            UnaryOp::Neg,
            Box::new(Expression::Value(Value::Literal(Literal::Num(867.5309)))),
        );
        let context = json!(null);

        assert_eq!(ast.evaluate(&context).unwrap(), json!(-867.5309))
    }

    #[test]
    fn negative_int_literals() {
        let ast = Expression::UnaryOp(
            UnaryOp::Neg,
            Box::new(Expression::Value(Value::Literal(Literal::Num(
                8675309 as f64,
            )))),
        );
        let context = json!(null);

        assert_eq!(ast.evaluate(&context).unwrap(), json!(-8675309))
    }

    #[test]
    fn string_literals() {
        let ast = Literal::Str("abc123".to_string());

        assert_eq!(ast.evaluate().unwrap(), json!("abc123"))
    }

    #[test]
    fn boolean_literals() {
        let ast = Literal::True;
        assert_eq!(ast.evaluate().unwrap(), json!(true));

        let ast = Literal::False;
        assert_eq!(ast.evaluate().unwrap(), json!(false));
    }

    #[test]
    fn null_literals() {
        let ast = Literal::Null;

        assert_eq!(ast.evaluate().unwrap(), json!(null))
    }
}
