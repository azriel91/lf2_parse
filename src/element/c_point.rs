use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, FrameNumber, FrameNumberNext, ObjectDataParser, Rule, SubRuleFn};

pub use self::{c_point_kind::CPointKind, c_point_kind_parse_error::CPointKindParseError};

mod c_point_kind;
mod c_point_kind_parse_error;

/// Aligns the character that is holding and the one that is held.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/177-cpoint-catch-point?showall=1
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

impl CPoint {
    fn parse_tags<'i>(
        c_point: CPoint,
        c_point_data_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        c_point_data_pair
            .into_inner()
            .try_fold(c_point, CPoint::parse_tag)
    }

    fn parse_tag<'i>(
        c_point: CPoint,
        c_point_tag_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        ObjectDataParser::parse_as_type(
            c_point,
            c_point_tag_pair,
            Rule::CPointTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(
        mut c_point: CPoint,
        c_point_tag_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        c_point = match c_point_tag_pair.as_rule() {
            Rule::TagKind => {
                ObjectDataParser::parse_value(c_point, c_point_tag_pair, Self::parse_kind_value)?
            }
            Rule::TagX => {
                ObjectDataParser::parse_value(c_point, c_point_tag_pair, Self::parse_x_value)?
            }
            Rule::TagY => {
                ObjectDataParser::parse_value(c_point, c_point_tag_pair, Self::parse_y_value)?
            }
            Rule::TagCover => {
                ObjectDataParser::parse_value(c_point, c_point_tag_pair, Self::parse_cover_value)?
            }
            Rule::TagDecrease => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_decrease_value,
            )?,
            Rule::TagDirControl => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_dir_control_value,
            )?,
            Rule::TagHurtable => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_hurtable_value,
            )?,
            Rule::TagInjury => {
                ObjectDataParser::parse_value(c_point, c_point_tag_pair, Self::parse_injury_value)?
            }
            Rule::TagAAction => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_a_action_value,
            )?,
            Rule::TagJAction => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_j_action_value,
            )?,
            Rule::TagVAction => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_v_action_value,
            )?,
            Rule::TagTAction => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_t_action_value,
            )?,
            Rule::TagThrowInjury => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_throw_injury_value,
            )?,
            Rule::TagThrowVx => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_throw_vx_value,
            )?,
            Rule::TagThrowVy => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_throw_vy_value,
            )?,
            Rule::TagThrowVz => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_throw_vz_value,
            )?,
            Rule::TagFrontHurtAct => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_front_hurt_act_value,
            )?,
            Rule::TagBackHurtAct => ObjectDataParser::parse_value(
                c_point,
                c_point_tag_pair,
                Self::parse_back_hurt_act_value,
            )?,
            _ => c_point,
        };
        Ok(c_point)
    }

    fn parse_kind_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let kind = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseCPointKind { value_pair, error })?;
        c_point.kind = kind;
        Ok(c_point)
    }

    fn parse_x_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let x = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(x),
                value_pair,
                error,
            })?;
        c_point.x = x;
        Ok(c_point)
    }

    fn parse_y_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let y = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(y),
                value_pair,
                error,
            })?;
        c_point.y = y;
        Ok(c_point)
    }

    fn parse_cover_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let cover = value_pair
            .as_str()
            .parse::<u32>()
            // Cover held character if value is non-zero
            .map(|value| value != 0)
            .map_err(|error| Error::ParseInt {
                field: stringify!(cover),
                value_pair,
                error,
            })?;
        c_point.cover = cover;
        Ok(c_point)
    }

    fn parse_decrease_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let decrease = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(decrease),
                value_pair,
                error,
            })?;
        c_point.decrease = decrease;
        Ok(c_point)
    }

    fn parse_dir_control_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let dir_control = value_pair
            .as_str()
            .parse::<u32>()
            // Allow direction control if value is non-zero
            .map(|value| value != 0)
            .map_err(|error| Error::ParseInt {
                field: stringify!(dir_control),
                value_pair,
                error,
            })?;
        c_point.dir_control = dir_control;
        Ok(c_point)
    }

    fn parse_hurtable_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let hurtable = value_pair
            .as_str()
            .parse::<u32>()
            // Allow held object to be hit if value is non-zero
            .map(|value| value != 0)
            .map_err(|error| Error::ParseInt {
                field: stringify!(hurtable),
                value_pair,
                error,
            })?;
        c_point.hurtable = hurtable;
        Ok(c_point)
    }

    fn parse_injury_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let injury = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(injury),
                value_pair,
                error,
            })?;
        c_point.injury = injury;
        Ok(c_point)
    }

    fn parse_a_action_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let a_action = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(a_action),
                value_pair,
                error,
            })?;
        c_point.a_action = a_action;
        Ok(c_point)
    }

    fn parse_j_action_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let j_action = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(j_action),
                value_pair,
                error,
            })?;
        c_point.j_action = j_action;
        Ok(c_point)
    }

    fn parse_v_action_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let v_action = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(v_action),
                value_pair,
                error,
            })?;
        c_point.v_action = v_action;
        Ok(c_point)
    }

    fn parse_t_action_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let t_action = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(t_action),
                value_pair,
                error,
            })?;
        c_point.t_action = t_action;
        Ok(c_point)
    }

    fn parse_throw_injury_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let throw_injury = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(throw_injury),
                value_pair,
                error,
            })?;
        c_point.throw_injury = throw_injury;
        Ok(c_point)
    }

    fn parse_throw_vx_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let throw_vx = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(throw_vx),
                value_pair,
                error,
            })?;
        c_point.throw_vx = throw_vx;
        Ok(c_point)
    }

    fn parse_throw_vy_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let throw_vy = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(throw_vy),
                value_pair,
                error,
            })?;
        c_point.throw_vy = throw_vy;
        Ok(c_point)
    }

    fn parse_throw_vz_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let throw_vz = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(throw_vz),
                value_pair,
                error,
            })?;
        c_point.throw_vz = throw_vz;
        Ok(c_point)
    }

    fn parse_front_hurt_act_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let front_hurt_act = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(front_hurt_act),
                value_pair,
                error,
            })?;
        c_point.front_hurt_act = front_hurt_act;
        Ok(c_point)
    }

    fn parse_back_hurt_act_value<'i>(
        mut c_point: CPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<CPoint, Error<'i>> {
        let back_hurt_act = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(back_hurt_act),
                value_pair,
                error,
            })?;
        c_point.back_hurt_act = back_hurt_act;
        Ok(c_point)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for CPoint {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<_>] = &[CPoint::parse_tags];
        ObjectDataParser::parse_as_type(CPoint::default(), pair, Rule::CPoint, sub_rule_fns)
    }
}
