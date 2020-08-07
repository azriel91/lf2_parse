use std::{
    convert::TryFrom,
    fmt::{self, Display},
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::FrameNumber;

/// Represents the next frame number to go to.
///
/// This uses an `isize` as a negative number indicates the object's facing direction should be flipped.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameNumberNext(pub isize);

impl FrameNumberNext {
    /// Returns a positive `FrameNumber`.
    pub fn abs(self) -> FrameNumber {
        let n = TryFrom::<isize>::try_from(self.0.abs()).unwrap_or_else(|e| {
            panic!(
                "Failed to convert `FrameNumberNext` to `FrameNumber`. {}",
                e
            );
        });
        FrameNumber(n)
    }

    /// Returns `true` if the object's facing direction should change.
    pub fn facing_switch(self) -> bool {
        self.0 < 0
    }
}

impl Deref for FrameNumberNext {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FrameNumberNext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for FrameNumberNext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for FrameNumberNext {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<FrameNumberNext, ParseIntError> {
        s.parse::<isize>().map(Self)
    }
}
