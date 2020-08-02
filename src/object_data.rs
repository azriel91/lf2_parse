use std::convert::TryFrom;

use derive_builder::Builder;
use pest::iterators::Pair;

use crate::{Error, Header, ObjectDataParser, Rule};

#[derive(Builder, Debug, PartialEq)]
#[builder(pattern = "owned")]
pub struct ObjectData {
    header: Header,
}

impl<'i> TryFrom<Pair<'i, Rule>> for ObjectData {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            ObjectDataBuilder::default(),
            pair,
            Rule::Object,
            &[|builder, header_pair| {
                Header::try_from(header_pair).map(|header| builder.header(header))
            }],
        )
        .and_then(|builder| builder.build().map_err(Error::DataBuildFailed))
    }
}
