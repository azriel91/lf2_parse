use std::{
    fmt::{self, Display},
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Represents the index in the [`WeaponStrengthList`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WeaponStrengthIndex(pub usize);

impl Deref for WeaponStrengthIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WeaponStrengthIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for WeaponStrengthIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for WeaponStrengthIndex {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<WeaponStrengthIndex, ParseIntError> {
        s.parse::<usize>().map(Self)
    }
}
