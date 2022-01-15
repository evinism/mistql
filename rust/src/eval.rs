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
            Literal::Num(num) => Ok(num.clone().into()),
            Literal::Str(str) => Ok(str.clone().into()),
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
    fn numeric_literals_return_themselves() {
        let ast = Literal::Num(8675309.0);

        assert_eq!(ast.evaluate().unwrap(), json!(8675309.0))
    }

    #[test]
    fn string_literals_return_themselves() {
        let ast = Literal::Str("abc123".to_string());

        assert_eq!(ast.evaluate().unwrap(), json!("abc123"))
    }
}
