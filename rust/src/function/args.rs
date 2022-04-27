use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct ArgParser<'a> {
    pair: Pair<'a, Rule>,
    pub data: &'a Value,
    context: Option<Value>,
    pub function: String,
}

impl<'a> ArgParser<'a> {
    pub fn new(pair: Pair<'a, Rule>, data: &'a Value, context: Option<Value>) -> Result<Self> {
        let function = match pair.as_rule() {
            Rule::ident => pair.as_str().to_string(),
            Rule::function => ident_from_fn(pair.clone().into_inner().next().unwrap(), data)?,
            _ => unreachable!(),
        };

        Ok(Self {
            pair: pair,
            data: data,
            context: context,
            function: function,
        })
    }

    fn eager_args(&self, skip: usize) -> Result<Vec<Value>> {
        let mut args = self
            .pair
            .clone()
            .into_inner()
            .skip(skip)
            .map(|arg| expr::eval(arg, self.data, None))
            .collect::<Result<Vec<Value>>>()?;

        if let Some(ctx) = self.context.clone() {
            args.push(ctx);
        }

        Ok(args)
    }

    fn lazy_arg(&self) -> Result<Pair<Rule>> {
        match self.pair.clone().into_inner().skip(1).next() {
            None => Err(Error::eval("expected fn as argument, got none".to_string())),
            Some(pair) => Ok(pair),
        }
    }

    pub fn one_arg(&self) -> Result<Value> {
        let args = self.eager_args(1)?;
        if args.len() == 1 {
            Ok(args[0].clone())
        } else {
            Err(Error::arity(self.function.clone(), 1, args.len()))
        }
    }

    pub fn two_args(&self) -> Result<(Value, Value)> {
        let args = self.eager_args(1)?;
        if args.len() == 2 {
            Ok((args[0].clone(), args[1].clone()))
        } else {
            Err(Error::arity(self.function.clone(), 2, args.len()))
        }
    }

    pub fn three_args(&self) -> Result<(Value, Value, Value)> {
        let args = self.eager_args(1)?;
        if args.len() == 3 {
            Ok((args[0].clone(), args[1].clone(), args[2].clone()))
        } else {
            Err(Error::arity(self.function.clone(), 3, args.len()))
        }
    }

    pub fn one_func_one_arg(&self) -> Result<(Pair<Rule>, Value)> {
        let func = self.lazy_arg()?;
        let args = self.eager_args(2)?;

        if args.len() == 1 {
            Ok((func, args[0].clone()))
        } else {
            Err(Error::arity(self.function.clone(), 1, args.len()))
        }
    }

    pub fn one_func_two_args(&self) -> Result<(Pair<Rule>, Value, Value)> {
        let func = self.lazy_arg()?;
        let args = self.eager_args(2)?;

        if args.len() == 2 {
            Ok((func, args[0].clone(), args[1].clone()))
        } else {
            Err(Error::arity(self.function.clone(), 2, args.len()))
        }
    }
}

fn ident_from_fn<'a>(pair: Pair<'a, Rule>, data: &'a Value) -> Result<String> {
    dbg!(pair.as_str());
    match pair.as_rule() {
        Rule::ident => Ok(pair.as_str().to_string()),
        Rule::function => match super::eval(pair, data, None)? {
            Value::Ident(s) => Ok(s),
            _ => unreachable!(),
        },
        rule => Err(Error::unimplemented(format!("fn rule {:?}", rule))),
    }
}
