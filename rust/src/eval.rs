use crate::parse::{Expression, Literal, Reference, Value};

impl<'a> Expression<'a> {
    pub fn evaluate(&'a self, context: &serde_json::Value) -> Result<serde_json::Value, String> {
        match self {
            Expression::Value(value) => value.evaluate(&context),
            _ => Err("Unknown expression type".to_string()),
        }
    }
}

impl<'a> Value<'a> {
    pub fn evaluate(&'a self, context: &serde_json::Value) -> Result<serde_json::Value, String> {
        match self {
            Value::Reference(reference) => reference.evaluate(&context),
            Value::Literal(literal) => literal.evaluate(),
            _ => Err("Unknown value type".to_string()),
        }
    }
}

impl<'a> Reference<'a> {
    pub fn evaluate(&'a self, context: &'a serde_json::Value) -> Result<serde_json::Value, String> {
        match self {
            Reference::At => Ok(context.clone()),
            _ => Err("Unknown reference type".to_string()),
        }
    }
}

impl<'a> Literal<'a> {
    pub fn evaluate(&'a self) -> Result<serde_json::Value, String> {
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
            _ => Err("Unknown literal type".to_string()),
        }
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
