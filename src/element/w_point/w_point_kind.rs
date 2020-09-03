use std::str::FromStr;

use crate::WPointKindParseError;

/// Whether this describes holding a weapon, held as one, or dropping one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPointKind {
    /// Indicates the information when holding a weapon.
    Holding = 1,
    /// Indicates the coordinates when held as a weapon.
    Held = 2,
    /// Indicates a held weapon should be dropped.
    Dropping = 3,
}

impl FromStr for WPointKind {
    type Err = WPointKindParseError;

    fn from_str(s: &str) -> Result<WPointKind, WPointKindParseError> {
        s.parse::<u32>()
            .map_err(WPointKindParseError::ParseIntError)
            .and_then(|value| match value {
                1 => Ok(WPointKind::Holding),
                2 => Ok(WPointKind::Held),
                3 => Ok(WPointKind::Dropping),
                value => Err(WPointKindParseError::InvalidValue(value)),
            })
    }
}
