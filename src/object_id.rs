use std::{
    fmt::{self, Display},
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Object ID in `data.txt`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ObjectId(pub usize);

impl Deref for ObjectId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ObjectId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ObjectId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<ObjectId, ParseIntError> {
        s.parse::<usize>().map(Self)
    }
}
