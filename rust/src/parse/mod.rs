use crate::error::{Error, Result};

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mistql.pest"]
pub struct MistQLParser;

pub enum Query {}

pub fn parse_query(_query: &str) -> Result<Query> {
    Err(Error::query("unimplemented".to_string()))
}

#[cfg(test)]
mod test {}
