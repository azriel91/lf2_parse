use std::{
    fmt::{self, Display},
    num::{NonZeroU32, ParseIntError},
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Default wait value of 1.
pub const WAIT_DEFAULT: Wait = Wait(unsafe { NonZeroU32::new_unchecked(1) });

/// Represents the frame number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wait(pub NonZeroU32);

impl Default for Wait {
    fn default() -> Self {
        WAIT_DEFAULT
    }
}

impl Deref for Wait {
    type Target = NonZeroU32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wait {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Wait {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Wait {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Wait, ParseIntError> {
        s.parse::<u32>()
            .map(|wait| NonZeroU32::new(wait).map(Self).unwrap_or(WAIT_DEFAULT))
    }
}
