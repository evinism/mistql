use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub enum Arg<'a> {
    Eager(Value),
    Lazy(Pair<'a, Rule>),
}

#[derive(Clone, Debug)]
pub struct ArgParser<'a> {
    pub data: &'a Value,
    pub function: String,
    args: Vec<Arg<'a>>,
}

impl<'a> Arg<'a> {
    pub fn to_pair(&self) -> Result<Pair<'a, Rule>> {
        match self {
            Arg::Lazy(pair) => Ok(pair.clone()),
            Arg::Eager(_) => Err(Error::eval(
                "can't convert evaluated argument to lazy argument".to_string(),
            )),
        }
    }

    pub fn to_value(&self, data: &Value) -> Result<Value> {
        match self {
            Arg::Eager(val) => Ok(val.clone()),
            Arg::Lazy(pair) => expr::eval(pair.clone(), data, None),
        }
    }

    pub fn to_ident(&self) -> Result<Value> {
        match self {
            Arg::Eager(Value::Ident(val)) => Ok(Value::Ident(val.clone())),
            Arg::Eager(val) => Err(Error::eval(format!("expected ident, got {:?}", val))),
            Arg::Lazy(pair) => match pair.as_rule() {
                Rule::ident => Ok(Value::Ident(pair.as_str().to_string())),
                _ => Err(Error::eval(format!("pair isn't an ident: {:?}", pair))),
            },
        }
    }
}

impl<'a> ArgParser<'a> {
    pub fn from_pair(
        pair: Pair<'a, Rule>,
        data: &'a Value,
        context: Option<Value>,
    ) -> Result<Self> {
        let mut components_itr = pair.into_inner();
        let function_pair = components_itr.next().unwrap().into_inner().next().unwrap();

        let function = match function_pair.as_rule() {
            Rule::ident => function_pair.as_str().to_string(),
            Rule::function => match super::fn_ident_eval(function_pair, data, context.clone())? {
                Value::Ident(s) => s,
                value => {
                    return Err(Error::eval(format!(
                        "higher-order function must return an identifier (got {:?}",
                        value
                    )))
                }
            },
            Rule::overwrite => function_pair
                .into_inner()
                .skip(1) // skip the $
                .next() // this must be an ident
                .unwrap()
                .as_str()
                .to_string(),
            _ => {
                return Err(Error::unimplemented(format!(
                    "fn_ident rule {:?}",
                    function_pair
                )))
            }
        };

        let mut args: Vec<Arg<'a>> = components_itr
            .next()
            .unwrap()
            .into_inner()
            .map(|arg| Arg::Lazy(arg))
            .collect();

        if let Some(ctx) = context.clone() {
            args.push(Arg::Eager(ctx));
        }

        Ok(Self {
            data: data,
            function: function,
            args: args,
        })
    }

    pub fn from_ident(
        pair: &Pair<'a, Rule>,
        data: &'a Value,
        context: Option<Value>,
    ) -> Result<Self> {
        let args = match context {
            Some(ctx) => vec![Arg::Eager(ctx)],
            None => vec![Arg::Eager(data.clone())],
        };

        Ok(Self {
            data: data,
            function: pair.as_str().to_string(),
            args: args,
        })
    }

    pub fn one_arg(&self) -> Result<Arg> {
        if self.args.len() == 1 {
            Ok(self.args[0].clone())
        } else {
            Err(Error::arity(self.function.to_string(), 1, self.args.len()))
        }
    }

    pub fn two_args(&self) -> Result<(Arg, Arg)> {
        if self.args.len() == 2 {
            Ok((self.args[0].clone(), self.args[1].clone()))
        } else {
            Err(Error::arity(self.function.to_string(), 2, self.args.len()))
        }
    }

    pub fn three_args(&self) -> Result<(Arg, Arg, Arg)> {
        if self.args.len() == 3 {
            Ok((
                self.args[0].clone(),
                self.args[1].clone(),
                self.args[2].clone(),
            ))
        } else {
            Err(Error::arity(self.function.to_string(), 3, self.args.len()))
        }
    }

    pub fn at_least_n_args(&self, n: usize) -> Result<Vec<Arg>> {
        if self.args.len() >= n {
            Ok(self.args.clone())
        } else {
            Err(Error::arity(self.function.to_string(), 3, self.args.len()))
        }
    }
}
