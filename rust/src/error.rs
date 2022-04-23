use snailquote::UnescapeError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    JSON,
    Regex,
    Query(String),
    Eval(String),
    Unimplemented(String),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::JSON => write!(f, "JSON error: {:?}", self.source),
            ErrorKind::Regex => write!(f, "Regex error: {:?}", self.source),
            ErrorKind::Query(msg) => write!(f, "Query error: {}", msg),
            ErrorKind::Eval(msg) => write!(f, "Eval error: {}", msg),
            ErrorKind::Unimplemented(msg) => write!(f, "Unimplemented: {}", msg),
        }
    }
}

impl From<UnescapeError> for Error {
    fn from(err: UnescapeError) -> Self {
        Self {
            kind: ErrorKind::JSON,
            source: Some(Box::new(err)),
        }
    }
}

impl Error {
    pub fn json(err: serde_json::Error) -> Self {
        Self {
            kind: ErrorKind::JSON,
            source: Some(Box::new(err)),
        }
    }

    pub fn regex(err: regex::Error) -> Self {
        Self {
            kind: ErrorKind::Regex,
            source: Some(Box::new(err)),
        }
    }

    pub fn query(msg: String) -> Self {
        Self {
            kind: ErrorKind::Query(msg),
            source: None,
        }
    }

    pub fn eval(msg: String) -> Self {
        Self {
            kind: ErrorKind::Eval(msg),
            source: None,
        }
    }

    pub fn unimplemented(msg: String) -> Self {
        Self {
            kind: ErrorKind::Unimplemented(msg),
            source: None,
        }
    }
}
