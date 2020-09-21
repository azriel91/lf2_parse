use std::str::FromStr;

use crate::CPointKindParseError;

/// Variants of `CPoint`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPointKind {
    /// The object that is holding the character.
    Catcher = 1,
    /// The held character.
    Caught = 2,
}

impl Default for CPointKind {
    fn default() -> Self {
        Self::Catcher
    }
}

impl FromStr for CPointKind {
    type Err = CPointKindParseError;

    fn from_str(s: &str) -> Result<CPointKind, CPointKindParseError> {
        s.parse::<u32>()
            .map_err(CPointKindParseError::ParseIntError)
            .and_then(|value| match value {
                1 => Ok(CPointKind::Catcher),
                2 => Ok(CPointKind::Caught),
                value => Err(CPointKindParseError::InvalidValue(value)),
            })
    }
}
