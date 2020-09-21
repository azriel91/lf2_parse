use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, FrameNumberNext, ObjectDataParser, ObjectId, Rule, SubRuleFn};

pub use self::{
    o_point_facing::OPointFacing, o_point_facing_dir::OPointFacingDir, o_point_kind::OPointKind,
    o_point_kind_parse_error::OPointKindParseError,
};

mod o_point_facing;
mod o_point_facing_dir;
mod o_point_kind;
mod o_point_kind_parse_error;

/// Spawns an object during a game.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/178-opoint-object-point
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

impl OPoint {
    fn parse_tags<'i>(
        o_point: OPoint,
        o_point_data_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        o_point_data_pair
            .into_inner()
            .try_fold(o_point, OPoint::parse_tag)
    }

    fn parse_tag<'i>(
        o_point: OPoint,
        o_point_tag_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        ObjectDataParser::parse_as_type(
            o_point,
            o_point_tag_pair,
            Rule::OPointTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(
        mut o_point: OPoint,
        o_point_tag_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        o_point = match o_point_tag_pair.as_rule() {
            Rule::TagKind => {
                ObjectDataParser::parse_value(o_point, o_point_tag_pair, Self::parse_kind_value)?
            }
            Rule::TagX => {
                ObjectDataParser::parse_value(o_point, o_point_tag_pair, Self::parse_x_value)?
            }
            Rule::TagY => {
                ObjectDataParser::parse_value(o_point, o_point_tag_pair, Self::parse_y_value)?
            }
            Rule::TagAction => {
                ObjectDataParser::parse_value(o_point, o_point_tag_pair, Self::parse_action_value)?
            }
            Rule::TagDVx => {
                ObjectDataParser::parse_value(o_point, o_point_tag_pair, Self::parse_d_vx_value)?
            }
            Rule::TagDVy => {
                ObjectDataParser::parse_value(o_point, o_point_tag_pair, Self::parse_d_vy_value)?
            }
            Rule::TagOid => {
                ObjectDataParser::parse_value(o_point, o_point_tag_pair, Self::parse_oid_value)?
            }
            Rule::TagFacing => {
                ObjectDataParser::parse_value(o_point, o_point_tag_pair, Self::parse_facing_value)?
            }
            _ => o_point,
        };
        Ok(o_point)
    }

    fn parse_kind_value<'i>(
        mut o_point: OPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        let kind = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseOPointKind { value_pair, error })?;
        o_point.kind = kind;
        Ok(o_point)
    }

    fn parse_x_value<'i>(
        mut o_point: OPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        let x = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(x),
                value_pair,
                error,
            })?;
        o_point.x = x;
        Ok(o_point)
    }

    fn parse_y_value<'i>(
        mut o_point: OPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        let y = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(y),
                value_pair,
                error,
            })?;
        o_point.y = y;
        Ok(o_point)
    }

    fn parse_action_value<'i>(
        mut o_point: OPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        let action = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseOPointAction { value_pair, error })?;
        o_point.action = action;
        Ok(o_point)
    }

    fn parse_d_vx_value<'i>(
        mut o_point: OPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        let d_vx = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(dvx),
                value_pair,
                error,
            })?;
        o_point.d_vx = d_vx;
        Ok(o_point)
    }

    fn parse_d_vy_value<'i>(
        mut o_point: OPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        let d_vy = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(dvy),
                value_pair,
                error,
            })?;
        o_point.d_vy = d_vy;
        Ok(o_point)
    }

    fn parse_oid_value<'i>(
        mut o_point: OPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        let object_id = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(oid),
                value_pair,
                error,
            })?;
        o_point.object_id = object_id;
        Ok(o_point)
    }

    fn parse_facing_value<'i>(
        mut o_point: OPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<OPoint, Error<'i>> {
        let facing = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(facing),
                value_pair,
                error,
            })?;
        o_point.facing = facing;
        Ok(o_point)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for OPoint {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<_>] = &[OPoint::parse_tags];
        ObjectDataParser::parse_as_type(OPoint::default(), pair, Rule::OPoint, sub_rule_fns)
    }
}
