use crate::{Error, Result};

pub fn string(args: Vec<serde_json::Value>) -> Result<serde_json::Value> {
    if let Some(val) = args.get(0) {
        match val {
            serde_json::Value::Number(num) => match num.as_f64() {
                Some(float_num) => Ok(from_number(float_num)),
                None => Ok(val.to_string().into()),
            },
            _ => Ok(val.to_string().into()),
        }
    } else {
        Err(Error::eval("string requires one argument".to_string()))
    }
}

fn from_number(num: f64) -> serde_json::Value {
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

    lexical::to_string_with_options::<f64, FORMAT>(num, &options).into()
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
