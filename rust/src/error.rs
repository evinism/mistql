use crate::parse::Rule;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    JSON(serde_json::Error),
    QueryParse,
    Query(String),
    UnimplementedEvaluation(String),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    source: Option<Box<dyn StdError + Sync + Send>>,
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|c| &**c as &(dyn StdError + 'static))
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Self::query_parse(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::JSON(ref e) => write!(f, "JSON error: {}", e),
            ErrorKind::QueryParse => write!(f, "Query parsing error: {:?}", self.source),
            ErrorKind::Query(ref msg) => write!(f, "query error: {}", msg),
            ErrorKind::UnimplementedEvaluation(ref msg) => {
                write!(f, "Unimplemented evaluation: {}", msg)
            }
        }
    }
}

impl Error {
    pub fn json(value: serde_json::Error) -> Self {
        Self {
            kind: ErrorKind::JSON(value),
            source: None,
        }
    }

    pub fn query_parse(err: pest::error::Error<Rule>) -> Self {
        Self {
            kind: ErrorKind::QueryParse,
            source: Some(Box::new(err)),
        }
    }

    pub fn query(msg: String) -> Self {
        Self {
            kind: ErrorKind::Query(msg),
            source: None,
        }
    }

    pub fn unimplemented_evaluation(msg: String) -> Self {
        Self {
            kind: ErrorKind::UnimplementedEvaluation(msg),
            source: None,
        }
    }
}
