use std::convert::TryFrom;

use derive_builder::Builder;
use pest::iterators::Pair;

use crate::{Error, Frames, Header, ObjectDataParser, Rule, SubRuleFn};

#[derive(Builder, Debug, PartialEq)]
#[builder(pattern = "owned")]
pub struct ObjectData {
    header: Header,
    frames: Frames,
}

impl<'i> TryFrom<Pair<'i, Rule>> for ObjectData {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<ObjectDataBuilder>] = &[
            |builder, header_pair| {
                Header::try_from(header_pair).map(|header| builder.header(header))
            },
            |builder, frames_pair| {
                Frames::try_from(frames_pair).map(|frames| builder.frames(frames))
            },
        ];

        ObjectDataParser::parse_as_type(
            ObjectDataBuilder::default(),
            pair,
            Rule::Object,
            sub_rule_fns,
        )
        .and_then(|builder| builder.build().map_err(Error::DataBuildFailed))
    }
}
