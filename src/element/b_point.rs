use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule, SubRuleFn};

/// Bleeding coordinates when the character has low HP.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BPoint {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl BPoint {
    fn parse_tags<'i>(
        b_point: BPoint,
        b_point_data_pair: Pair<'i, Rule>,
    ) -> Result<BPoint, Error<'i>> {
        b_point_data_pair
            .into_inner()
            .try_fold(b_point, BPoint::parse_tag)
    }

    fn parse_tag<'i>(
        b_point: BPoint,
        b_point_tag_pair: Pair<'i, Rule>,
    ) -> Result<BPoint, Error<'i>> {
        ObjectDataParser::parse_as_type(
            b_point,
            b_point_tag_pair,
            Rule::BPointTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(
        mut b_point: BPoint,
        b_point_tag_pair: Pair<'i, Rule>,
    ) -> Result<BPoint, Error<'i>> {
        b_point = match b_point_tag_pair.as_rule() {
            Rule::TagX => {
                ObjectDataParser::parse_value(b_point, b_point_tag_pair, Self::parse_x_value)?
            }
            Rule::TagY => {
                ObjectDataParser::parse_value(b_point, b_point_tag_pair, Self::parse_y_value)?
            }
            _ => b_point,
        };
        Ok(b_point)
    }

    fn parse_x_value<'i>(
        mut b_point: BPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<BPoint, Error<'i>> {
        let x = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(x),
                value_pair,
                error,
            })?;
        b_point.x = x;
        Ok(b_point)
    }

    fn parse_y_value<'i>(
        mut b_point: BPoint,
        value_pair: Pair<'i, Rule>,
    ) -> Result<BPoint, Error<'i>> {
        let y = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(y),
                value_pair,
                error,
            })?;
        b_point.y = y;
        Ok(b_point)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for BPoint {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<_>] = &[BPoint::parse_tags];
        ObjectDataParser::parse_as_type(BPoint::default(), pair, Rule::BPoint, sub_rule_fns)
    }
}
