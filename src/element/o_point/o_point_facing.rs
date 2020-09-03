use crate::OPointFacingDir;

/// Number of objects to spawn, and their facing direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OPointFacing {
    /// Number of objects to spawn.
    pub count: u32,
    /// Whether the same / opposite of parent, or always to the right.
    pub direction: OPointFacingDir,
}
