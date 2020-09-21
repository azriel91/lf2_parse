use std::str::FromStr;

use crate::{BdyKindParseError, FrameNumberNext};

/// Hittable volume of an object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BdyKind {
    /// Regular hittable body.
    Normal,
    /// For type 5, frame number to switch to when hit by [`ItrKind::Normal`].
    Hostage {
        /// Frame number to switch to when freed.
        freed_frame: FrameNumberNext,
    },
}

impl BdyKind {
    fn from_frame_number(value: isize) -> Self {
        let freed_frame = FrameNumberNext(value);
        BdyKind::Hostage { freed_frame }
    }
}

impl Default for BdyKind {
    fn default() -> Self {
        BdyKind::Normal
    }
}

impl FromStr for BdyKind {
    type Err = BdyKindParseError;

    fn from_str(s: &str) -> Result<BdyKind, BdyKindParseError> {
        s.parse::<isize>()
            .map_err(BdyKindParseError::ParseIntError)
            .and_then(|value| match value {
                -1999..=-1000 => Ok(BdyKind::from_frame_number(value + 1000)),
                -999..=-1 => Err(BdyKindParseError::InvalidValue(value)),
                0 => Ok(BdyKind::Normal),
                1..=999 => Err(BdyKindParseError::InvalidValue(value)),
                1000..=1999 => Ok(BdyKind::from_frame_number(value - 1000)),
                value => Err(BdyKindParseError::InvalidValue(value)),
            })
    }
}
