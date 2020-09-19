use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule, SubRuleFn};

pub use self::{
    b_point::BPoint,
    bdy::{Bdy, BdyKind, BdyKindParseError},
    c_point::{CPoint, CPointKind},
    itr::{Effect, EffectParseError, Itr, ItrKind},
    o_point::{OPoint, OPointFacing, OPointFacingDir, OPointKind},
    w_point::{WPoint, WPointKind, WPointKindParseError},
};

mod b_point;
mod bdy;
mod c_point;
mod itr;
mod o_point;
mod w_point;

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    /// Hittable body of the object.
    Bdy(Bdy),
    /// Bleeding coordinates when the character has low HP.
    BPoint(BPoint),
    /// Aligns the character that is holding and the one that is held.
    CPoint(CPoint),
    /// Interaction that this object places on another.
    Itr(Itr),
    /// Spawns an object.
    OPoint(OPoint),
    /// Controls a held light / heavy weapon.
    WPoint(WPoint),
}

impl Element {
    fn parse_element<'i>(
        element: Option<Element>,
        element_pair: Pair<'i, Rule>,
    ) -> Result<Option<Element>, Error<'i>> {
        let element_parsed = match element_pair.as_rule() {
            Rule::Bdy => Bdy::try_from(element_pair).map(Self::Bdy),
            Rule::BPoint => BPoint::try_from(element_pair).map(Self::BPoint),
            // Rule::CPoint => CPoint::try_from(element_pair).map(Self::CPoint),
            // Rule::Itr => Itr::try_from(element_pair).map(Self::Itr),
            // Rule::OPoint => OPoint::try_from(element_pair).map(Self::OPoint),
            Rule::WPoint => WPoint::try_from(element_pair).map(Self::WPoint),
            Rule::CPoint | Rule::Itr | Rule::OPoint => return Ok(element),
            _ => Err(Error::Grammar {
                rules_expected: &[
                    Rule::Bdy,
                    Rule::BPoint,
                    Rule::CPoint,
                    Rule::Itr,
                    Rule::OPoint,
                    Rule::WPoint,
                ],
                pair_found: Some(element_pair),
            }),
        }?;
        Ok(element.or(Some(element_parsed)))
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Element {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<Option<Self>>] = &[Self::parse_element];

        ObjectDataParser::parse_as_type(None, pair.clone(), Rule::Element, sub_rule_fns)
            .and_then(|element| element.ok_or_else(|| Error::ElementBuildNone(pair)))
    }
}
