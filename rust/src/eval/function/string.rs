use crate::eval::Value;
use crate::{Error, Result};

pub fn string(args: Vec<Value>) -> Result<Value> {
    if let Some(val) = args.get(0) {
        match val {
            Value::Null => Ok(Value::String("null".to_string())),
            Value::Boolean(b) => Ok(Value::String(b.to_string())),
            Value::Float(num) => Ok(from_number(*num)),
            Value::Int(num) => Ok(from_number(*num as f64)),
            Value::String(_) => Ok(val.clone()),
            Value::Ident(s) => Ok(Value::String(s.clone())),
            Value::Array(_) | Value::Object(_) => Err(Error::eval(
                "can't cast object or array to string".to_string(),
            )),
        }
    } else {
        Err(Error::eval("string requires one argument".to_string()))
    }
}

fn from_number(num: f64) -> Value {
    const FORMAT: u128 = lexical::NumberFormatBuilder::new()
        .required_digits(true)
        .no_positive_mantissa_sign(true)
        .no_special(true)
        .no_integer_leading_zeros(true)
        .no_float_leading_zeros(true)
        .required_exponent_sign(true)
        .build();
    let options = lexical::WriteFloatOptions::builder()
        .trim_floats(true)
        .positive_exponent_break(std::num::NonZeroI32::new(20))
        .negative_exponent_break(std::num::NonZeroI32::new(-6))
        .build()
        .unwrap();

    Value::String(lexical::to_string_with_options::<f64, FORMAT>(
        num, &options,
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn casts_int_to_string() {
        let result = crate::query("string 1".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("1".to_string()));
    }

    #[test]
    fn casts_float_to_string() {
        let result = crate::query("string 1.5".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("1.5".to_string()));

        // omits the .0
        let result = crate::query("string 1.0".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("1".to_string()));

        // overflows f64 (requires arbitrary precision)
        let result = crate::query("string 1e50".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("1e+50".to_string()));

        let result = crate::query("string 1e-50".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("1e-50".to_string()));

        // exponent notation breakpoints
        let result = crate::query("string 1e-6".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("0.000001".to_string()));

        let result = crate::query("string 1e-7".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("1e-7".to_string()));

        let result = crate::query("string 3e20".to_string(), "null".to_string()).unwrap();
        assert_eq!(
            result,
            serde_json::Value::String("300000000000000000000".to_string())
        );

        let result = crate::query("string 3e21".to_string(), "null".to_string()).unwrap();
        assert_eq!(result, serde_json::Value::String("3e+21".to_string()));
    }
}
