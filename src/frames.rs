use std::{
    collections::BTreeMap,
    convert::TryFrom,
    ops::{Deref, DerefMut},
};

use pest::iterators::Pair;
use tinyvec::TinyVec;

use crate::{Error, Frame, ObjectDataParser, Rule, SubRuleWrapper};

/// `Vec<Frame>` newtype.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Frames(pub Vec<Frame>);

impl Frames {
    /// Maximum number of frames that a data file may have.
    ///
    /// LF2 has a hard limit of 400 by default. There are exe modifications that
    /// increase this, but we will not support this for now.
    const FRAME_COUNT_MAX: usize = 400;

    fn parse_frame<'i>(
        (mut frame_pairs, mut frames): (Vec<Pair<'i, Rule>>, Frames),
        frame_pair: Pair<'i, Rule>,
    ) -> Result<(Vec<Pair<'i, Rule>>, Frames), Error<'i>> {
        let frame_pair_clone = frame_pair.clone();
        if let Ok(frame) = Frame::try_from(frame_pair) {
            frame_pairs.push(frame_pair_clone);
            frames.push(frame);
        }

        Ok((frame_pairs, frames))
    }

    fn validate<'i>(
        (frame_pairs, frames): (Vec<Pair<'i, Rule>>, Frames),
    ) -> Result<Self, Error<'i>> {
        let frame_number_indiceses = frames.0.iter().enumerate().fold(
            BTreeMap::new(),
            |mut frame_number_indiceses, (index, frame)| {
                let frame_number_indices = frame_number_indiceses
                    .entry(frame.number)
                    .or_insert_with(TinyVec::<[usize; 2]>::default);
                frame_number_indices.reserve(1);
                frame_number_indices.push(index);
                frame_number_indiceses
            },
        );
        frame_number_indiceses
            .into_iter()
            .find(|(_, frame_number_indices)| frame_number_indices.len() > 1)
            .map(|(frame_number, frame_number_indices)| {
                // Find the pairs corresponding to this frame number.
                // The frame numbers in the map should be exactly the same as the pairs in the
                // `frame_pairs` vector.
                let frame_pairs_non_unique = frame_number_indices
                    .into_iter()
                    .filter_map(|index| frame_pairs.get(index).cloned())
                    .collect::<Vec<Pair<'i, Rule>>>();

                Err(Error::FrameNumberNonUnique {
                    frame_number,
                    frame_pairs: frame_pairs_non_unique,
                })
            })
            .unwrap_or_else(|| Ok(frames))
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
        let frame_pairs = Vec::<Pair<'i, Rule>>::with_capacity(Self::FRAME_COUNT_MAX);
        let frames = Frames(Vec::with_capacity(Self::FRAME_COUNT_MAX));
        let frame_pairs_and_frames = ObjectDataParser::parse_as_type(
            (frame_pairs, frames),
            pair,
            Rule::Frames,
            Iterator::cycle([SubRuleWrapper::new(Self::parse_frame)].iter()),
        )?;
        Self::validate(frame_pairs_and_frames)
    }
}
