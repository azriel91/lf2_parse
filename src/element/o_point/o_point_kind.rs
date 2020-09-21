use std::str::FromStr;

use crate::OPointKindParseError;

/// Object spawning variants.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/178-opoint-object-point
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPointKind {
    /// Spawns an object on the same team.
    ///
    /// Note that when spawning type: 0 objects (characters), ID 5 (Rudolf) and
    /// ID 52 (Julian) are spawned with 10 HP, and all other IDs are spawned
    /// with 500 HP.
    Spawn = 1,
    /// Object is spawned and held as a light weapon.
    ///
    /// Ensure the spawned object has `WPoint` kind: `2` in its spawned frame.
    HoldLightWeapon = 2,
}

impl Default for OPointKind {
    fn default() -> Self {
        Self::Spawn
    }
}

impl FromStr for OPointKind {
    type Err = OPointKindParseError;

    fn from_str(s: &str) -> Result<OPointKind, OPointKindParseError> {
        s.parse::<u32>()
            .map_err(OPointKindParseError::ParseIntError)
            .and_then(|value| match value {
                1 => Ok(OPointKind::Spawn),
                2 => Ok(OPointKind::HoldLightWeapon),
                value => Err(OPointKindParseError::InvalidValue(value)),
            })
    }
}
