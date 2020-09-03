use crate::{FrameNumber, FrameNumberNext};

pub use self::c_point_kind::CPointKind;

mod c_point_kind;

/// Aligns the character that is holding and the one that is held.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/177-cpoint-catch-point?showall=1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CPoint {
    /// Catching object or caught character.
    pub kind: CPointKind,
    /// X coordinate.
    ///
    /// The catching character and the caught character's `CPoint` coordinates
    /// align with each other.
    pub x: i32,
    /// Y coordinate.
    ///
    /// The catching character and the caught character's `CPoint` coordinates
    /// align with each other.
    pub y: i32,

    // --- Catching `CPoint` fields --- //
    /// Adjusts the facing direction and whether it is drawn behind the catcher.
    ///
    /// Here you can set whether the caught character is shown behind or in
    /// front of your character. Leaving this tag out will let the caught
    /// character be faced towards the catcher while behind behind him.
    ///
    /// Assume the tag written as "cover: AB" with A and B being the values
    /// being displayed below:
    ///
    /// A:
    ///
    /// * `1` aligns the caught character to face the same direction.
    /// * `2` aligns the caught character to face the opposite direction.
    ///
    /// Anything else does does not change the caught characters facing.
    ///
    /// B:
    ///
    /// * `0` displays the caught character behind the catcher (z position -1).
    /// * `1-9` displays the caught character in front of the catcher (z
    ///   position +1).
    ///
    /// Because these work via z positioning they can fail on the z edges of a
    /// background.
    pub cover: bool,
    /// Catching timer decrease value.
    ///
    /// As soon as a decrease is used, the cpoint will only last for a certain
    /// period of time. Here you can set which frame should be used for the held
    /// character after the time is up: If you set the value to `3` the
    /// character goes to frame `211` (jump), if you use `7` or `-7` the
    /// character switches to frame `181` (falling).
    ///
    /// `decrease:` seems to be something like `fall:` on an itr, however
    /// inverted as negative values actually lower the "time" left till the
    /// caught character drops (always falling) and positive values do
    /// nothing / increase the value determining the drop (might actually use
    /// the fall value). The character is always released in the jump frame
    /// if you leave into a frame without a `cpoint` before `decrease` takes
    /// effect (or using `throwvx:` or a non cpoint `vaction:`), regardless
    /// of the last decrease value.
    pub decrease: i32,
    /// Whether the catcher switches direction when a direction key is pressed.
    ///
    /// Used in `throw_lying_man` frames. If the value is `1` you can change
    /// directions, if it's `0` you can't.
    ///
    /// **Note:** `dircontrol:` requires at least `wait: 2` to work.
    pub dir_control: bool,
    /// If other characters can hurt the caught character
    ///
    /// `1` if they can, `0` if not.
    pub hurtable: bool,
    ///
    pub injury: i32,
    /// Frame number to switch to when the `Attack` button is pressed.
    pub a_action: FrameNumberNext,
    /// Frame upon pressing jump.
    ///
    /// `jaction:` will ignore the `mp` value of the target frame, however using
    /// `hit_j:` will completely ignore the `jaction:`.
    pub j_action: FrameNumberNext,
    /// Frame number that the caught character should be on.
    pub v_action: FrameNumber,
    /// Frame upon holding a direction and pressing attack. Default is 232.
    ///
    /// Comment by YinYin:
    ///
    /// > I much prefer a positive value on the `taction:` myself so I don't
    /// > throw backwards when only holding up/down and pressing attack.
    pub t_action: FrameNumberNext,
    /// Health points the caught character loses when thrown onto the ground.
    ///
    /// Details:
    ///
    /// * Sometimes, the strange value `-842150451` is used here as well.
    /// * Without a positive throwinjury the thrown character will not damage
    ///   anyone.
    pub throw_injury: i32,
    /// Velocity in the X Axis when the caught character is thrown.
    ///
    /// Details:
    ///
    /// * Sometimes the value `-842150451` is used, but it does not seem to
    ///   serve any purpose.
    /// * For `throwvx:`, every value except 0 causes the caught character to
    ///   get dropped. `throwvy:`, `throwvz:` and `throwinjury:` only work
    ///   together with `throwvx:`.
    pub throw_vx: i32,
    /// Velocity in the Y Axis when the caught character is thrown.
    ///
    /// Details:
    ///
    /// * Sometimes the value `-842150451` is used, but it does not seem to
    ///   serve any purpose.
    /// * For `throwvx:`, every value except 0 causes the caught character to
    ///   get dropped. `throwvy:`, `throwvz:` and `throwinjury:` only work
    ///   together with `throwvx:`.
    pub throw_vy: i32,
    /// Velocity in the Z Axis when the caught character is thrown.
    ///
    /// Details:
    ///
    /// * Sometimes the value `-842150451` is used, but it does not seem to
    ///   serve any purpose.
    /// * For `throwvx:`, every value except 0 causes the caught character to
    ///   get dropped. `throwvy:`, `throwvz:` and `throwinjury:` only work
    ///   together with `throwvx:`.
    pub throw_vz: i32,

    // --- Caught `CPoint` fields --- //
    /// Frame to switch to when caught character is hit from the front.
    pub front_hurt_act: FrameNumberNext,
    /// Frame to switch to when caught character is hit from the back.
    pub back_hurt_act: FrameNumberNext,
}
