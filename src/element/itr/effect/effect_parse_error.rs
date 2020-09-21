use std::{fmt, fmt::Display, num::ParseIntError};

/// Errors when parsing a string as a `Effect`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectParseError {
    /// The string could not be parsed as a `u32`.
    ParseIntError(ParseIntError),
    /// The value is not recognized as a valid `Effect`.
    InvalidValue(u32),
}

impl Display for EffectParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseIntError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::InvalidValue(value) => write!(
                f,
                "`{}` is not recognized as a valid `Effect` value.\n\
                \n\
                Valid values are:\n\
                \n\
                - 0 (Normal)\n\
                - 1 (Blood)\n\
                - 2 (Fire)\n\
                - 3 (Ice)\n\
                - 4 (Reflect)\n\
                - 5 (Reflects)\n\
                - 20 (FireNoReburn)\n\
                - 21 (FireBreath)\n\
                - 22 (FireExplode)\n\
                - 23 (PowerExplode)\n\
                - 30 (Icicle)\n\
                \n\
                ",
                value
            ),
        }
    }
}

impl std::error::Error for EffectParseError {}
