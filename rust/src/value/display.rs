use crate::Value;
use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Regex(pattern, flags) => {
                write!(f, "{}{}", pattern, regex_flags(flags.clone()))
            }
            Value::Null => write!(f, "null"),
            Value::Boolean(true) => write!(f, "true"),
            Value::Boolean(false) => write!(f, "false"),
            Value::Float(num) => write!(f, "{}", from_number(*num)),
            Value::Int(num) => write!(f, "{}", from_number(*num as f64)),
            Value::String(s) => write!(f, "{}", s),
            Value::Ident(s) => write!(f, "{}", s),
            Value::Array(a) => {
                write!(
                    f,
                    "[{}]",
                    a.iter()
                        .map(|elt| elt.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Value::Object(o) => {
                write!(
                    f,
                    "{{{}}}",
                    o.iter()
                        .map(|(k, v)| format!("\"{}\":{}", k.to_string(), v.to_string()))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}

fn regex_flags(flags: Option<String>) -> String {
    match flags {
        Some(s) => format!("(?{}:exp)", s),
        None => String::new(),
    }
}

fn from_number(num: f64) -> String {
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

    lexical::to_string_with_options::<f64, FORMAT>(num, &options)
}

#[cfg(test)]
mod tests {
    use crate::Value;
    #[test]
    fn casts_int_to_string() {
        let result = Value::Int(1).to_string();
        assert_eq!(result, "1")
    }

    #[test]
    fn casts_float_to_string() {
        for example in vec![
            (1.5, "1.5"),
            (1.0, "1"),
            (1e50, "1e+50"),
            (1e-50, "1e-50"),
            (1e-6, "0.000001"),
            (1e-7, "1e-7"),
            (3e20, "300000000000000000000"),
            (3e21, "3e+21"),
        ] {
            let (input, output) = example;
            assert_eq!(Value::Float(input).to_string(), output);
        }
    }

    #[test]
    fn casts_array_to_string() {
        let result = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]).to_string();
        assert_eq!(result, "[1,2,3]");

        let result =
            Value::Array(vec![Value::Int(1), Value::Boolean(false), Value::Null]).to_string();
        assert_eq!(result, "[1,false,null]");
    }
}
