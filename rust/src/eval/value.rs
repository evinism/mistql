use pest::iterators::Pair;

use crate::{Error, Result, Rule};

pub fn eval(pair: Pair<Rule>, _context: &serde_json::Value) -> Result<serde_json::Value> {
    let parsed: std::result::Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(pair.as_str());
    match parsed {
        Ok(val) => Ok(val.clone().into()),
        Err(_) => Err(Error::query(format!("unparseable value {:?}", pair))),
    }
}
