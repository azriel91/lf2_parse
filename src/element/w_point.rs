use crate::{FrameNumberNext, WeaponStrengthIndex};

pub use self::{w_point_kind::WPointKind, w_point_kind_parse_error::WPointKindParseError};

mod w_point_kind;
mod w_point_kind_parse_error;

/// Holds a weapon / weapon is held.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/179-wpoint-weapon-point
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WPoint {
    /// Whether this describes holding a weapon, held as one, or dropping one.
    pub kind: WPointKind,
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// Frame number that the held weapon uses.
    ///
    /// Only used for `WPoint` `kind: 1`.
    pub weapon_act: FrameNumberNext,
    /// When holding a light weapon, which attack strength to use.
    pub attacking: WeaponStrengthIndex,
    /// Acceleration on the X axis to throw the weapon.
    ///
    /// Leave this at 0 if you don't want to throw the weapon.
    pub d_vx: i64,
    /// Acceleration on the Y axis to throw the weapon.
    ///
    /// Negative values go up.
    ///
    /// Leave this at 0 if you don't want to throw the weapon.
    pub d_vy: i64,
}
