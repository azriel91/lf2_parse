use std::{fmt, fmt::Display, num::ParseIntError};

/// Errors when parsing a string as a `CPointKind`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CPointKindParseError {
    /// The string could not be parsed as a `u32`.
    ParseIntError(ParseIntError),
    /// The value is not recognized as a valid `CPointKind`.
    InvalidValue(u32),
}

impl Display for CPointKindParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseIntError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::InvalidValue(value) => write!(
                f,
                "`{}` is not recognized as a valid `CPointKind` value.\n\
                Valid values are [1 (Catcher), 2 (Caught)].",
                value
            ),
        }
    }
}

impl std::error::Error for CPointKindParseError {}
