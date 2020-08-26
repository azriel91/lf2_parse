use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule, SubRuleFn};

pub use self::{
    b_point::BPoint, bdy::Bdy, c_point::CPoint, itr::Itr, o_point::OPoint, w_point::WPoint,
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
        element_pair
            .into_inner()
            .try_fold(element, |element, element_inner_pair| {
                let element_parsed = match element_inner_pair.as_rule() {
                    Rule::Bdy => Bdy::try_from(element_inner_pair).map(Self::Bdy),
                    Rule::BPoint => BPoint::try_from(element_inner_pair).map(Self::BPoint),
                    Rule::CPoint => CPoint::try_from(element_inner_pair).map(Self::CPoint),
                    Rule::Itr => Itr::try_from(element_inner_pair).map(Self::Itr),
                    Rule::OPoint => OPoint::try_from(element_inner_pair).map(Self::OPoint),
                    Rule::WPoint => WPoint::try_from(element_inner_pair).map(Self::WPoint),
                    _ => Err(Error::Grammar {
                        rule_expected: Rule::Bdy, // TODO: Take in multiple expected rules.
                        pair_found: Some(element_inner_pair),
                    }),
                }?;
                Ok(element.or(element_parsed))
            })
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Element {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<ElementBuilder>] = &[Self::parse_element];

        ObjectDataParser::parse_as_type(None, pair, Rule::Element, sub_rule_fns)
            .and_then(|element| element.ok_or_else(|| Error::DataBuildFailed(pair)))
    }
}
