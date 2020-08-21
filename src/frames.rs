use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
};

use pest::iterators::Pair;

use crate::{Error, Frame, ObjectDataParser, Rule, SubRuleFn};

/// `Vec<Frame>` newtype.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Frames(pub Vec<Frame>);

impl Frames {
    fn parse_frame<'i>(
        mut frames: Frames,
        frame_pair: Pair<'i, Rule>,
    ) -> Result<Frames, Error<'i>> {
        Frame::try_from(frame_pair).map(|frame| {
            frames.push(frame);
            frames
        })
    }
}

impl Deref for Frames {
    type Target = Vec<Frame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Frames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Frames {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            Frames::default(),
            pair,
            Rule::Frames,
            &[Self::parse_frame as SubRuleFn<_>],
        )
    }
}
