use std::{self, fmt};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    BadInt(u32),
    BadChar(char),
    BadString(String),
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::BadInt(i) => write!(f, "failed to parse from unexpected integer {i}"),
            ParseError::BadChar(c) => write!(f, "failed to parse from unexpected character {c}"),
            ParseError::BadString(s) => write!(f, "failed to parse from unexpected string \"{s}\""),
        }
    }
}
