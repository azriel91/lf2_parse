use std::{
    fmt::{self, Display},
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Represents the frame number.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameNumber(pub usize);

impl Deref for FrameNumber {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FrameNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for FrameNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for FrameNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<FrameNumber, ParseIntError> {
        s.parse::<usize>().map(Self)
    }
}
