use std::{num::ParseIntError, str::FromStr};

use crate::OPointFacingDir;

/// Number of objects to spawn, and their facing direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OPointFacing {
    /// Number of objects to spawn.
    pub count: u32,
    /// Whether the same / opposite of parent, or always to the right.
    pub direction: OPointFacingDir,
}

impl FromStr for OPointFacing {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<OPointFacing, ParseIntError> {
        s.parse::<u32>().map(|value| match value {
            0 => OPointFacing {
                count: 1,
                direction: OPointFacingDir::ParentSame,
            },
            1 => OPointFacing {
                count: 1,
                direction: OPointFacingDir::ParentOpposite,
            },
            // In LF2, `facing: 10` spawns an object always facing right.
            10 => OPointFacing {
                count: 1,
                direction: OPointFacingDir::Right,
            },
            _ => {
                let count = value / 10;
                let direction = if value & 1 == 0 {
                    OPointFacingDir::ParentSame
                } else {
                    OPointFacingDir::ParentOpposite
                };

                OPointFacing { count, direction }
            }
        })
    }
}
