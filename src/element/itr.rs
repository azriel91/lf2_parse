use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, FrameNumberNext, ObjectDataParser, Rule, SubRuleFn};

pub use self::{
    effect::{Effect, EffectParseError},
    itr_kind::ItrKind,
    itr_kind_parse_error::ItrKindParseError,
};

mod effect;
mod itr_kind;
mod itr_kind_parse_error;

/// Area that hits other objects.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/174-itr-interaction?start=1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Itr {
    /// Interaction variants.
    pub kind: ItrKind,
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// Width.
    pub w: u32,
    /// Height.
    pub h: u32,
    /// Z Width extends in both directions + 1 center pixel.
    ///
    /// `zwidth: 10` means 10 pixels up, 10 pixels down, and one pixel for
    /// center of the shadow for 21 pixels total.
    pub z_width: u32,
    /// Acceleration to place on the hit object in the X axis.
    ///
    /// # Notes
    ///
    /// * `itr/kind:8` ([`ItrKind::HealBall`]) uses this as the frame number for
    ///   the object with this `Itr` to switch to when hitting a character.
    pub d_vx: i64,
    /// Acceleration to place on the hit object in the Y axis.
    pub d_vy: i64,
    /// Delay before another hit may happen, restricts this `Itr` to one object.
    pub a_rest: u32,
    /// Delay before another hit may happen, allows multiple objects to be hit.
    pub v_rest: u32,
    /// How much a character is "off balance".
    ///
    /// The `fall` value determines how an attacked character will react to this
    /// itr by flinching, getting into the stunned frames, or `falling`. If no
    /// value is specified, the default of `20` will be used.
    ///
    /// * If a character accumulates `20` `fall` points, he switches to
    ///   `injured1` (`220`).
    /// * If a character accumulates `40` `fall` points, he switches to
    ///   `injured2` (`222`) or `injured2back` (`224`) depending on the
    ///   direction he is hit, and will fall if he is in mid-air.
    /// * If a character accumulates `60` `fall` points, he switches into the
    ///   `stunned` (`226`) frames where he can be grabbed or hit by
    ///   `super_punch`.
    ///
    /// Attacks with `fall: 41` or more can hit `falling` characters.
    ///
    /// Here are a few values as a rule of thumb for various `fall` values:
    ///
    /// | `fall` | Description                                              |
    /// | -----: | :------------------------------------------------------- |
    /// |     -1 | Does not go into injured frames and harder to knockdown. |
    /// |      1 | Never stun, never fall (Davis DvA shrafe)                |
    /// |     20 | 3 hit stun, 4 hit fall                                   |
    /// |     25 | 2 hit stun, 3 hit fall (Dennis normal kick)              |
    /// |     40 | Does not stun, 2 hit fall (baseball bat normal swing)    |
    /// |     60 | 1 hit stun, 2 hit fall (Henry's arrow)                   |
    /// |     70 | 1 hit fall                                               |
    ///
    /// Every 1 TU, a `fall` point is deducted.
    pub fall: i32,
    /// Broken-defence points.
    ///
    /// `bdefend` points determine if a character is able to block an attack by
    /// defending or if he will go to the broken-defense-frames. As long as he
    /// has 30 or less Bdefend-points, he will be able to block the attack, if
    /// it's 31 or higher, he goes to the broken-defense-frames. If an itr hits
    /// the character while he is not in the defend-frames, his `bdefend`
    /// counter will automatically increase to 45. If you have, for example,
    /// accumulated 31 points and get hit during your defense (assuming you have
    /// specified a positive `bdefend` value in the hitting itr), the character
    /// will go to the broken-defense-frames.
    ///
    /// Here are some common values for various `bdefend` values:
    ///
    /// | `bdefend` | Description                                       |
    /// | --------: | :------------------------------------------------ |
    /// |         0 | never breaks defense (ex: John's D>J shield)      |
    /// |        12 | 4 hit break                                       |
    /// |        16 | 3 hit break                                       |
    /// |        30 | 2 hit break                                       |
    /// |        60 | 1 hit break                                       |
    /// |       100 | ignores defense, sets `bdefend` counter to `45`,\
    ///               and instantly destroys weapons.                   |
    ///
    /// Every 1 TU, a `bdefend` point is deducted, so he will be able to recover
    /// his defense.
    ///
    /// Armor will function as long as a character has not accumulated more
    /// `bdefend` points than the specific armor points of Louis(1), Knight or
    /// Julian(15) at the time of attack. For example, Julian can resist a dash
    /// attack(bdefend 60) even though he only has 15 armor points, but he will
    /// be left completely vulnerable for the next 45 TU until he regains his
    /// 1st armor point.
    pub b_defend: i32,
    /// Amount of damage to inflict on the target object.
    ///
    /// # Notes
    ///
    /// * `itr/kind:5` ([`ItrKind::WeaponStrength`]) ignores this and uses the
    ///   `injury` in `weapon_strength_list`.
    /// * `itr/kind:8` ([`ItrKind::HealBall`]) uses this for the number of HP to
    ///   heal a character by.
    pub injury: i32,
    /// Itr `effect` variants.
    pub effect: Effect,
    /// Frame numbers for where the catching object should switch to.
    ///
    /// Used in `itr/kind: 1` ([`ItrKind::CatchStunned`]) and `itr/kind: 3`
    /// ([`ItrKind::CatchForce`]).
    pub catching_act: FrameNumberNext,
    /// Frame numbers for where the caught character should switch to.
    ///
    /// Used in `itr/kind: 1` ([`ItrKind::CatchStunned`]) and `itr/kind: 3`
    /// ([`ItrKind::CatchForce`]).
    pub caught_act: FrameNumberNext,
}

impl Default for Itr {
    fn default() -> Self {
        Itr {
            kind: Default::default(),
            x: Default::default(),
            y: Default::default(),
            w: Default::default(),
            h: Default::default(),
            z_width: Self::Z_WIDTH_DEFAULT,
            d_vx: Default::default(),
            d_vy: Default::default(),
            a_rest: Default::default(),
            v_rest: Default::default(),
            fall: Default::default(),
            b_defend: Default::default(),
            injury: Default::default(),
            effect: Default::default(),
            catching_act: Default::default(),
            caught_act: Default::default(),
        }
    }
}

impl Itr {
    /// Default `Z_WIDTH` for `Itr` volumes.
    pub const Z_WIDTH_DEFAULT: u32 = 13;

    fn parse_tags<'i>(itr: Itr, itr_data_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        itr_data_pair.into_inner().try_fold(itr, Itr::parse_tag)
    }

    fn parse_tag<'i>(itr: Itr, itr_tag_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        ObjectDataParser::parse_as_type(
            itr,
            itr_tag_pair,
            Rule::ItrTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(mut itr: Itr, itr_tag_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        itr = match itr_tag_pair.as_rule() {
            Rule::TagKind => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_kind_value)?
            }
            Rule::TagX => ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_x_value)?,
            Rule::TagY => ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_y_value)?,
            Rule::TagW => ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_w_value)?,
            Rule::TagH => ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_h_value)?,
            Rule::TagZWidth => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_z_width_value)?
            }
            Rule::TagDVx => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_d_vx_value)?
            }
            Rule::TagDVy => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_d_vy_value)?
            }
            Rule::TagARest => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_a_rest_value)?
            }
            Rule::TagVRest => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_v_rest_value)?
            }
            Rule::TagFall => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_fall_value)?
            }
            Rule::TagBDefend => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_b_defend_value)?
            }
            Rule::TagInjury => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_injury_value)?
            }
            Rule::TagEffect => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_effect_value)?
            }
            Rule::TagCatchingAct => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_catching_act_value)?
            }
            Rule::TagCaughtAct => {
                ObjectDataParser::parse_value(itr, itr_tag_pair, Self::parse_caught_act_value)?
            }
            _ => itr,
        };
        Ok(itr)
    }

    fn parse_kind_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let kind = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseItrKind { value_pair, error })?;
        itr.kind = kind;
        Ok(itr)
    }

    fn parse_x_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let x = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(x),
                value_pair,
                error,
            })?;
        itr.x = x;
        Ok(itr)
    }

    fn parse_y_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let y = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(y),
                value_pair,
                error,
            })?;
        itr.y = y;
        Ok(itr)
    }

    fn parse_w_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let w = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(w),
                value_pair,
                error,
            })?;
        itr.w = w;
        Ok(itr)
    }

    fn parse_h_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let h = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(h),
                value_pair,
                error,
            })?;
        itr.h = h;
        Ok(itr)
    }

    fn parse_z_width_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let z_width = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(zwidth),
                value_pair,
                error,
            })?;
        itr.z_width = z_width;
        Ok(itr)
    }

    fn parse_d_vx_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let d_vx = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(d_vx),
                value_pair,
                error,
            })?;
        itr.d_vx = d_vx;
        Ok(itr)
    }

    fn parse_d_vy_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let d_vy = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(d_vy),
                value_pair,
                error,
            })?;
        itr.d_vy = d_vy;
        Ok(itr)
    }

    fn parse_a_rest_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let a_rest = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(arest),
                value_pair,
                error,
            })?;
        itr.a_rest = a_rest;
        Ok(itr)
    }

    fn parse_v_rest_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let v_rest = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(vrest),
                value_pair,
                error,
            })?;
        itr.v_rest = v_rest;
        Ok(itr)
    }

    fn parse_fall_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let fall = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(fall),
                value_pair,
                error,
            })?;
        itr.fall = fall;
        Ok(itr)
    }

    fn parse_b_defend_value<'i>(
        mut itr: Itr,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Itr, Error<'i>> {
        let b_defend = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(b_defend),
                value_pair,
                error,
            })?;
        itr.b_defend = b_defend;
        Ok(itr)
    }

    fn parse_injury_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let injury = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(injury),
                value_pair,
                error,
            })?;
        itr.injury = injury;
        Ok(itr)
    }

    fn parse_effect_value<'i>(mut itr: Itr, value_pair: Pair<'i, Rule>) -> Result<Itr, Error<'i>> {
        let effect = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseItrEffect { value_pair, error })?;
        itr.effect = effect;
        Ok(itr)
    }

    fn parse_catching_act_value<'i>(
        mut itr: Itr,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Itr, Error<'i>> {
        let catching_act = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(catching_act),
                value_pair,
                error,
            })?;
        itr.catching_act = catching_act;
        Ok(itr)
    }

    fn parse_caught_act_value<'i>(
        mut itr: Itr,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Itr, Error<'i>> {
        let caught_act = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(caught_act),
                value_pair,
                error,
            })?;
        itr.caught_act = caught_act;
        Ok(itr)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Itr {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<_>] = &[Itr::parse_tags];
        ObjectDataParser::parse_as_type(Itr::default(), pair, Rule::Itr, sub_rule_fns)
    }
}
