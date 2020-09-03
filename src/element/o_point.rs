use crate::{FrameNumberNext, ObjectId};

pub use self::{
    o_point_facing::OPointFacing, o_point_facing_dir::OPointFacingDir, o_point_kind::OPointKind,
};

mod o_point_facing;
mod o_point_facing_dir;
mod o_point_kind;

/// Spawns an object during a game.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/178-opoint-object-point
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OPoint {
    /// Object spawning variants.
    pub kind: OPointKind,
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// Frame number that the spawned object starts with.
    pub action: FrameNumberNext,
    /// Initial acceleration on the X axis.
    ///
    /// Positive value moves forward in the direction the spawned object is
    /// facing. See [`OPoint::facing`].
    pub d_vx: i64,
    /// Initial acceleration on the Y axis.
    ///
    /// Positive value is downwards.
    pub d_vy: i64,
    /// ID of the object to spawn.
    pub object_id: ObjectId,
    /// Number of objects to spawn, and their facing direction.
    pub facing: OPointFacing,
}
