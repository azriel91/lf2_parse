use std::convert::TryFrom;

use pest::iterators::Pair;

use crate::{Error, Frames, Header, ObjectDataParser, Rule, SubRuleFn};

#[derive(Debug, Default, PartialEq)]
pub struct ObjectData {
    pub header: Header,
    pub frames: Frames,
}

impl<'i> TryFrom<Pair<'i, Rule>> for ObjectData {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<ObjectData>] = &[
            |mut object_data, header_pair| {
                Header::try_from(header_pair).map(|header| {
                    object_data.header = header;
                    object_data
                })
            },
            |mut object_data, frames_pair| {
                Frames::try_from(frames_pair).map(|frames| {
                    object_data.frames = frames;
                    object_data
                })
            },
        ];

        ObjectDataParser::parse_as_type(ObjectData::default(), pair, Rule::Object, sub_rule_fns)
    }
}
