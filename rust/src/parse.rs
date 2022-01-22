use crate::error::MistQLError;

pub enum Expression {}

pub fn parse_query(query: &str) -> Result<Expression, MistQLError> {
    Err(MistQLError::QueryParseError("no parse yet".to_string()))
}

#[cfg(test)]
mod tests {}
