use std::{fmt, fmt::Display, num::ParseIntError};

/// Errors when parsing a string as a `BdyKind`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BdyKindParseError {
    /// The string could not be parsed as an `isize`.
    ParseIntError(ParseIntError),
    /// The value is not recognized as a valid `BdyKind`.
    InvalidValue(isize),
}

impl Display for BdyKindParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseIntError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::InvalidValue(value) => {
                write!(
                    f,
                    "`{}` is not a valid `BdyKind` value. Valid values are:\n\
                    \n\
                    * 0: Normal\n\
                    * 1000 to 1999: Go to frame 0 - 999\n\
                    * -1000 to -1999: Go to frame 0 - 999, change facing\n\
                    \n\
                    ", value)
            }
        }
    }
}

impl std::error::Error for BdyKindParseError {}
