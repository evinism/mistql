use crate::error::{Error, Result};

pub enum Expression {}

pub fn parse_query(_query: &str) -> Result<Expression> {
    Err(Error::query("no parse yet".to_string()))
}

#[cfg(test)]
mod tests {}
