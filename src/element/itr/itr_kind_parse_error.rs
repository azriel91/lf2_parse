use std::{fmt, fmt::Display, num::ParseIntError};

/// Errors when parsing a string as an `ItrKind`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItrKindParseError {
    /// The string could not be parsed as a `u32`.
    ParseIntError(ParseIntError),
    /// The value is not recognized as a valid `ItrKind`.
    InvalidValue(u32),
}

impl Display for ItrKindParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseIntError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::InvalidValue(value) => write!(
                f,
                "`{}` is not recognized as a valid `ItrKind` value.\n\
                Valid values are:\n\
                \n\
                - 0 (Normal),\n\
                - 1 (CatchStunned),\n\
                - 2 (WeaponPick),\n\
                - 3 (CatchForce),\n\
                - 4 (Falling),\n\
                - 5 (WeaponStrength),\n\
                - 6 (SuperPunch),\n\
                - 7 (RollWeaponPick),\n\
                - 8 (HealBall),\n\
                - 9 (ReflectiveShield),\n\
                - 10 (SonataOfDeath),\n\
                - 11 (SonataOfDeath2),\n\
                - 14 (Wall),\n\
                - 15 (WhirlwindWind),\n\
                - 16 (WhirlwindIce),\n\
                \n",
                value
            ),
        }
    }
}

impl std::error::Error for ItrKindParseError {}
