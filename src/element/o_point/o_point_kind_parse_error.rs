use std::{fmt, fmt::Display, num::ParseIntError};

/// Errors when parsing a string as an `OPointKind`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OPointKindParseError {
    /// The string could not be parsed as a `u32`.
    ParseIntError(ParseIntError),
    /// The value is not recognized as a valid `OPointKind`.
    InvalidValue(u32),
}

impl Display for OPointKindParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseIntError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::InvalidValue(value) => write!(
                f,
                "`{}` is not recognized as a valid `OPointKind` value.\n\
                Valid values are [1 (Spawn), 2 (HoldLightWeapon)].",
                value
            ),
        }
    }
}

impl std::error::Error for OPointKindParseError {}
