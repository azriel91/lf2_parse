use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, FrameNumberNext, ObjectDataParser, Rule, SubRuleFn, WeaponStrengthIndex};

pub use self::{w_point_kind::WPointKind, w_point_kind_parse_error::WPointKindParseError};

mod w_point_kind;
mod w_point_kind_parse_error;

/// Holds a weapon / weapon is held.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/179-wpoint-weapon-point
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

impl WPoint {
    fn parse_tags<'i>(
        w_point: WPoint,
        w_point_data_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        w_point_data_pair
            .into_inner()
            .try_fold(w_point, WPoint::parse_tag)
    }

    fn parse_tag<'i>(
        w_point: WPoint,
        w_point_tag_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        ObjectDataParser::parse_as_type(
            w_point,
            w_point_tag_pair,
            Rule::WPointTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(
        mut w_point: WPoint,
        w_point_tag_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        w_point = match w_point_tag_pair.as_rule() {
            Rule::TagKind => {
                ObjectDataParser::parse_value(w_point, w_point_tag_pair, Self::parse_kind_value)?
            }
            Rule::TagX => {
                ObjectDataParser::parse_value(w_point, w_point_tag_pair, Self::parse_x_value)?
            }
            Rule::TagY => {
                ObjectDataParser::parse_value(w_point, w_point_tag_pair, Self::parse_y_value)?
            }
            Rule::TagWeaponAct => ObjectDataParser::parse_value(
                w_point,
                w_point_tag_pair,
                Self::parse_weapon_act_value,
            )?,
            Rule::TagAttacking => ObjectDataParser::parse_value(
                w_point,
                w_point_tag_pair,
                Self::parse_attacking_value,
            )?,
            Rule::TagDVx => {
                ObjectDataParser::parse_value(w_point, w_point_tag_pair, Self::parse_d_vx_value)?
            }
            Rule::TagDVy => {
                ObjectDataParser::parse_value(w_point, w_point_tag_pair, Self::parse_d_vy_value)?
            }
            _ => w_point,
        };
        Ok(w_point)
    }

    fn parse_kind_value<'i>(
        mut w_point: WPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        let kind = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseWPointKind { value_pair, error })?;
        w_point.kind = kind;
        Ok(w_point)
    }

    fn parse_x_value<'i>(
        mut w_point: WPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        let x = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(x),
                value_pair,
                error,
            })?;
        w_point.x = x;
        Ok(w_point)
    }

    fn parse_y_value<'i>(
        mut w_point: WPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        let y = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(y),
                value_pair,
                error,
            })?;
        w_point.y = y;
        Ok(w_point)
    }

    fn parse_weapon_act_value<'i>(
        mut w_point: WPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        let weapon_act = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseWeaponAct { value_pair, error })?;
        w_point.weapon_act = weapon_act;
        Ok(w_point)
    }

    fn parse_attacking_value<'i>(
        mut w_point: WPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        let attacking = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseWeaponStrengthIndex { value_pair, error })?;
        w_point.attacking = attacking;
        Ok(w_point)
    }

    fn parse_d_vx_value<'i>(
        mut w_point: WPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        let d_vx = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(dvx),
                value_pair,
                error,
            })?;
        w_point.d_vx = d_vx;
        Ok(w_point)
    }

    fn parse_d_vy_value<'i>(
        mut w_point: WPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<WPoint, Error<'i>> {
        let d_vy = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(dvy),
                value_pair,
                error,
            })?;
        w_point.d_vy = d_vy;
        Ok(w_point)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for WPoint {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<_>] = &[WPoint::parse_tags];
        ObjectDataParser::parse_as_type(WPoint::default(), pair, Rule::WPoint, sub_rule_fns)
    }
}
