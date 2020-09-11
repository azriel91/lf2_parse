use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule, SubRuleFn};

pub use self::{bdy_kind::BdyKind, bdy_kind_parse_error::BdyKindParseError};

mod bdy_kind;
mod bdy_kind_parse_error;

/// Hittable body of the object.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bdy {
    /// Only used in criminal (type 5) objects.
    ///
    /// If you use `kind: 1050` (1000 + Frame number) and the bdy is hit by some
    /// `itr`s, the object switches to frame 50.
    pub kind: BdyKind,
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// Width.
    pub w: u32,
    /// Height.
    pub h: u32,
}

impl Bdy {
    fn parse_tags<'i>(bdy: Bdy, bdy_data_pair: Pair<'i, Rule>) -> Result<Bdy, Error<'i>> {
        bdy_data_pair.into_inner().try_fold(bdy, Bdy::parse_tag)
    }

    fn parse_tag<'i>(bdy: Bdy, bdy_tag_pair: Pair<'i, Rule>) -> Result<Bdy, Error<'i>> {
        ObjectDataParser::parse_as_type(
            bdy,
            bdy_tag_pair,
            Rule::BdyTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(mut bdy: Bdy, bdy_tag_pair: Pair<'i, Rule>) -> Result<Bdy, Error<'i>> {
        bdy = match bdy_tag_pair.as_rule() {
            Rule::TagKind => {
                ObjectDataParser::parse_value(bdy, bdy_tag_pair, Self::parse_kind_value)?
            }
            Rule::TagX => ObjectDataParser::parse_value(bdy, bdy_tag_pair, Self::parse_x_value)?,
            Rule::TagY => ObjectDataParser::parse_value(bdy, bdy_tag_pair, Self::parse_y_value)?,
            Rule::TagW => ObjectDataParser::parse_value(bdy, bdy_tag_pair, Self::parse_w_value)?,
            Rule::TagH => ObjectDataParser::parse_value(bdy, bdy_tag_pair, Self::parse_h_value)?,
            _ => bdy,
        };
        Ok(bdy)
    }

    fn parse_kind_value<'i>(mut bdy: Bdy, value_pair: Pair<'i, Rule>) -> Result<Bdy, Error<'i>> {
        let kind = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseBdyKind { value_pair, error })?;
        bdy.kind = kind;
        Ok(bdy)
    }

    fn parse_x_value<'i>(mut bdy: Bdy, value_pair: Pair<'i, Rule>) -> Result<Bdy, Error<'i>> {
        let x = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(x),
                value_pair,
                error,
            })?;
        bdy.x = x;
        Ok(bdy)
    }

    fn parse_y_value<'i>(mut bdy: Bdy, value_pair: Pair<'i, Rule>) -> Result<Bdy, Error<'i>> {
        let y = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(y),
                value_pair,
                error,
            })?;
        bdy.y = y;
        Ok(bdy)
    }

    fn parse_w_value<'i>(mut bdy: Bdy, value_pair: Pair<'i, Rule>) -> Result<Bdy, Error<'i>> {
        let w = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(w),
                value_pair,
                error,
            })?;
        bdy.w = w;
        Ok(bdy)
    }

    fn parse_h_value<'i>(mut bdy: Bdy, value_pair: Pair<'i, Rule>) -> Result<Bdy, Error<'i>> {
        let h = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(h),
                value_pair,
                error,
            })?;
        bdy.h = h;
        Ok(bdy)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Bdy {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<_>] = &[Bdy::parse_tags];
        ObjectDataParser::parse_as_type(Bdy::default(), pair, Rule::Bdy, sub_rule_fns)
    }
}
