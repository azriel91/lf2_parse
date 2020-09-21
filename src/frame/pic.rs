use std::{
    convert::TryFrom,
    fmt::{self, Display},
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Represents the sprite number to use.
///
/// This uses an `isize` as a negative number indicates the sprite should be
/// flipped.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pic(pub isize);

impl Pic {
    /// Returns the absolute value of this pic number.
    pub fn abs(self) -> usize {
        TryFrom::<isize>::try_from(self.0.abs()).unwrap_or_else(|e| {
            panic!("Failed to convert `Pic` to `usize`. {}", e);
        })
    }

    /// Returns `true` if the object's facing direction should change.
    pub fn facing_switch(self) -> bool {
        self.0 < 0
    }
}

impl Deref for Pic {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Pic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Pic {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Pic, ParseIntError> {
        s.parse::<isize>().map(Self)
    }
}
