use std::{fmt, fmt::Display, num::ParseIntError};

/// Errors when parsing a string as a `State`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateParseError {
    /// The string could not be parsed as a `u32`.
    ParseIntError(ParseIntError),
    /// The value is not recognized as a valid `State`.
    InvalidValue(u32),
}

impl Display for StateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseIntError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::InvalidValue(value) => {
                write!(f, "`{}` is not recognized as a valid `State` value.", value)
            }
        }
    }
}

impl std::error::Error for StateParseError {}
