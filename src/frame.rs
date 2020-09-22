use std::{convert::TryFrom, path::PathBuf};

use pest::iterators::Pair;

use crate::{Element, Error, ObjectDataParser, Rule, SubRuleFn};

pub use self::{
    frame_number::FrameNumber,
    frame_number_next::FrameNumberNext,
    pic::Pic,
    state::{State, StateParseError},
    wait::Wait,
};

mod frame_number;
mod frame_number_next;
mod pic;
mod state;
mod wait;

#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub number: FrameNumber,
    pub name: String,
    pub center_x: i64,
    pub center_y: i64,
    pub d_vx: i64,
    pub d_vy: i64,
    pub d_vz: i64,
    pub elements: Vec<Element>,
    pub hit_a: FrameNumberNext,
    pub hit_d: FrameNumberNext,
    pub hit_da: FrameNumberNext,
    pub hit_dj: FrameNumberNext,
    pub hit_fa: FrameNumberNext,
    pub hit_fj: FrameNumberNext,
    pub hit_j: FrameNumberNext,
    pub hit_ja: FrameNumberNext,
    pub hit_ua: FrameNumberNext,
    pub hit_uj: FrameNumberNext,
    pub mp: i64,
    pub next_frame: FrameNumberNext,
    pub pic: Pic,
    pub sound: Option<PathBuf>,
    pub state: State,
    pub wait: Wait,
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            number: Default::default(),
            name: Default::default(),
            center_x: Default::default(),
            center_y: Default::default(),
            d_vx: Default::default(),
            d_vy: Default::default(),
            d_vz: Default::default(),
            elements: Default::default(),
            hit_a: Default::default(),
            hit_d: Default::default(),
            hit_da: Default::default(),
            hit_dj: Default::default(),
            hit_fa: Default::default(),
            hit_fj: Default::default(),
            hit_j: Default::default(),
            hit_ja: Default::default(),
            hit_ua: Default::default(),
            hit_uj: Default::default(),
            mp: Default::default(),
            next_frame: Default::default(),
            pic: Default::default(),
            sound: Default::default(),
            state: State::Uninitialized,
            wait: Default::default(),
        }
    }
}

impl Frame {
    fn parse_number<'i>(
        frame: Frame,
        frame_number_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            frame_number_pair,
            Rule::FrameNumber,
            &[Self::parse_number_value as SubRuleFn<_>],
        )
    }

    fn parse_number_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let number = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(frame number),
                value_pair,
                error,
            })?;
        frame.number = number;
        Ok(frame)
    }

    fn parse_name<'i>(frame: Frame, frame_name_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            frame_name_pair,
            Rule::FrameName,
            &[Self::parse_name_value as SubRuleFn<_>],
        )
    }

    fn parse_name_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let name = value_pair.as_str().to_string();
        frame.name = name;
        Ok(frame)
    }

    fn parse_data<'i>(frame: Frame, frame_data_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        frame_data_pair
            .into_inner()
            .try_fold(
                frame,
                |mut frame, frame_tag_or_element_pair| match frame_tag_or_element_pair.as_rule() {
                    Rule::FrameTag => Frame::parse_tag(frame, frame_tag_or_element_pair),
                    Rule::Element => {
                        if let Ok(element) = Element::try_from(frame_tag_or_element_pair) {
                            frame.elements.push(element);
                        }
                        Ok(frame)
                    }
                    _ => Err(Error::Grammar {
                        rules_expected: &[Rule::Element, Rule::FrameTag],
                        pair_found: Some(frame_tag_or_element_pair),
                    }),
                },
            )
    }

    fn parse_tag<'i>(frame: Frame, frame_tag_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            frame_tag_pair,
            Rule::FrameTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(
        mut frame: Frame,
        frame_tag_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        match frame_tag_pair.as_rule() {
            Rule::TagCenterX => {
                frame = Self::parse_center_x(frame, frame_tag_pair)?;
            }
            Rule::TagCenterY => {
                frame = Self::parse_center_y(frame, frame_tag_pair)?;
            }
            Rule::TagDVx => {
                frame = Self::parse_d_vx(frame, frame_tag_pair)?;
            }
            Rule::TagDVy => {
                frame = Self::parse_d_vy(frame, frame_tag_pair)?;
            }
            Rule::TagDVz => {
                frame = Self::parse_d_vz(frame, frame_tag_pair)?;
            }
            Rule::TagHitA => {
                frame = Self::parse_hit_a(frame, frame_tag_pair)?;
            }
            Rule::TagHitD => {
                frame = Self::parse_hit_d(frame, frame_tag_pair)?;
            }
            Rule::TagHitDa => {
                frame = Self::parse_hit_da(frame, frame_tag_pair)?;
            }
            Rule::TagHitDj => {
                frame = Self::parse_hit_dj(frame, frame_tag_pair)?;
            }
            Rule::TagHitFa => {
                frame = Self::parse_hit_fa(frame, frame_tag_pair)?;
            }
            Rule::TagHitFj => {
                frame = Self::parse_hit_fj(frame, frame_tag_pair)?;
            }
            Rule::TagHitJ => {
                frame = Self::parse_hit_j(frame, frame_tag_pair)?;
            }
            Rule::TagHitJa => {
                frame = Self::parse_hit_ja(frame, frame_tag_pair)?;
            }
            Rule::TagHitUa => {
                frame = Self::parse_hit_ua(frame, frame_tag_pair)?;
            }
            Rule::TagHitUj => {
                frame = Self::parse_hit_uj(frame, frame_tag_pair)?;
            }
            Rule::TagMp => {
                frame = Self::parse_mp(frame, frame_tag_pair)?;
            }
            Rule::TagNext => {
                frame = Self::parse_next_frame(frame, frame_tag_pair)?;
            }
            Rule::TagPic => {
                frame = Self::parse_pic(frame, frame_tag_pair)?;
            }
            Rule::TagSound => {
                frame = Self::parse_sound(frame, frame_tag_pair)?;
            }
            Rule::TagState => {
                frame = Self::parse_state(frame, frame_tag_pair)?;
            }
            Rule::TagWait => {
                frame = Self::parse_wait(frame, frame_tag_pair)?;
            }
            _ => {}
        }
        Ok(frame)
    }

    fn parse_center_x<'i>(frame: Frame, center_x_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            center_x_pair,
            Rule::TagCenterX,
            &[Self::parse_center_x_value as SubRuleFn<_>],
        )
    }

    fn parse_center_x_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let center_x = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(centerx),
                value_pair,
                error,
            })?;
        frame.center_x = center_x;
        Ok(frame)
    }

    fn parse_center_y<'i>(frame: Frame, center_y_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            center_y_pair,
            Rule::TagCenterY,
            &[Self::parse_center_y_value as SubRuleFn<_>],
        )
    }

    fn parse_center_y_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let center_y = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(centery),
                value_pair,
                error,
            })?;
        frame.center_y = center_y;
        Ok(frame)
    }

    fn parse_d_vx<'i>(frame: Frame, d_vx_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            d_vx_pair,
            Rule::TagDVx,
            &[Self::parse_d_vx_value as SubRuleFn<_>],
        )
    }

    fn parse_d_vx_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let d_vx = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(dvx),
                value_pair,
                error,
            })?;
        frame.d_vx = d_vx;
        Ok(frame)
    }

    fn parse_d_vy<'i>(frame: Frame, d_vy_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            d_vy_pair,
            Rule::TagDVy,
            &[Self::parse_d_vy_value as SubRuleFn<_>],
        )
    }

    fn parse_d_vy_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let d_vy = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(dvy),
                value_pair,
                error,
            })?;
        frame.d_vy = d_vy;
        Ok(frame)
    }

    fn parse_d_vz<'i>(frame: Frame, d_vz_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            d_vz_pair,
            Rule::TagDVz,
            &[Self::parse_d_vz_value as SubRuleFn<_>],
        )
    }

    fn parse_d_vz_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let d_vz = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(dvz),
                value_pair,
                error,
            })?;
        frame.d_vz = d_vz;
        Ok(frame)
    }

    fn parse_hit_a<'i>(frame: Frame, hit_a_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_a_pair,
            Rule::TagHitA,
            &[Self::parse_hit_a_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_a_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_a = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_a),
                value_pair,
                error,
            })?;
        frame.hit_a = hit_a;
        Ok(frame)
    }

    fn parse_hit_d<'i>(frame: Frame, hit_d_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_d_pair,
            Rule::TagHitD,
            &[Self::parse_hit_d_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_d_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_d = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_d),
                value_pair,
                error,
            })?;
        frame.hit_d = hit_d;
        Ok(frame)
    }

    fn parse_hit_da<'i>(frame: Frame, hit_da_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_da_pair,
            Rule::TagHitDa,
            &[Self::parse_hit_da_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_da_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_da = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_Da),
                value_pair,
                error,
            })?;
        frame.hit_da = hit_da;
        Ok(frame)
    }

    fn parse_hit_dj<'i>(frame: Frame, hit_dj_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_dj_pair,
            Rule::TagHitDj,
            &[Self::parse_hit_dj_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_dj_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_dj = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_Dj),
                value_pair,
                error,
            })?;
        frame.hit_dj = hit_dj;
        Ok(frame)
    }

    fn parse_hit_fa<'i>(frame: Frame, hit_fa_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_fa_pair,
            Rule::TagHitFa,
            &[Self::parse_hit_fa_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_fa_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_fa = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_Fa),
                value_pair,
                error,
            })?;
        frame.hit_fa = hit_fa;
        Ok(frame)
    }

    fn parse_hit_fj<'i>(frame: Frame, hit_fj_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_fj_pair,
            Rule::TagHitFj,
            &[Self::parse_hit_fj_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_fj_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_fj = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_Fj),
                value_pair,
                error,
            })?;
        frame.hit_fj = hit_fj;
        Ok(frame)
    }

    fn parse_hit_j<'i>(frame: Frame, hit_j_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_j_pair,
            Rule::TagHitJ,
            &[Self::parse_hit_j_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_j_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_j = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_j),
                value_pair,
                error,
            })?;
        frame.hit_j = hit_j;
        Ok(frame)
    }

    fn parse_hit_ja<'i>(frame: Frame, hit_ja_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_ja_pair,
            Rule::TagHitJa,
            &[Self::parse_hit_ja_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_ja_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_ja = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_ja),
                value_pair,
                error,
            })?;
        frame.hit_ja = hit_ja;
        Ok(frame)
    }

    fn parse_hit_ua<'i>(frame: Frame, hit_ua_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_ua_pair,
            Rule::TagHitUa,
            &[Self::parse_hit_ua_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_ua_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_ua = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_ua),
                value_pair,
                error,
            })?;
        frame.hit_ua = hit_ua;
        Ok(frame)
    }

    fn parse_hit_uj<'i>(frame: Frame, hit_uj_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            hit_uj_pair,
            Rule::TagHitUj,
            &[Self::parse_hit_uj_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_uj_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let hit_uj = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(hit_uj),
                value_pair,
                error,
            })?;
        frame.hit_uj = hit_uj;
        Ok(frame)
    }

    fn parse_mp<'i>(frame: Frame, mp_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            mp_pair,
            Rule::TagMp,
            &[Self::parse_mp_value as SubRuleFn<_>],
        )
    }

    fn parse_mp_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let mp = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(mp),
                value_pair,
                error,
            })?;
        frame.mp = mp;
        Ok(frame)
    }

    fn parse_next_frame<'i>(
        frame: Frame,
        next_frame_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            next_frame_pair,
            Rule::TagNext,
            &[Self::parse_next_frame_value as SubRuleFn<_>],
        )
    }

    fn parse_next_frame_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let next_frame = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(next),
                value_pair,
                error,
            })?;
        frame.next_frame = next_frame;
        Ok(frame)
    }

    fn parse_pic<'i>(frame: Frame, pic_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            pic_pair,
            Rule::TagPic,
            &[Self::parse_pic_value as SubRuleFn<_>],
        )
    }

    fn parse_pic_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let pic = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(pic),
                value_pair,
                error,
            })?;
        frame.pic = pic;
        Ok(frame)
    }

    fn parse_sound<'i>(frame: Frame, sound_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            sound_pair,
            Rule::TagSound,
            &[Self::parse_sound_value as SubRuleFn<_>],
        )
    }

    fn parse_sound_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let sound = value_pair
            .as_str()
            .parse()
            .map_err(|_error| Error::ParsePath {
                field: stringify!(sound),
                value_pair,
            })?;
        frame.sound = Some(sound);
        Ok(frame)
    }

    fn parse_state<'i>(frame: Frame, state_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            state_pair,
            Rule::TagState,
            &[Self::parse_state_value as SubRuleFn<_>],
        )
    }

    fn parse_state_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let state = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::StateParse { value_pair, error })?;
        frame.state = state;
        Ok(frame)
    }

    fn parse_wait<'i>(frame: Frame, wait_pair: Pair<'i, Rule>) -> Result<Frame, Error<'i>> {
        ObjectDataParser::parse_as_type(
            frame,
            wait_pair,
            Rule::TagWait,
            &[Self::parse_wait_value as SubRuleFn<_>],
        )
    }

    fn parse_wait_value<'i>(
        mut frame: Frame,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Frame, Error<'i>> {
        let wait = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(wait),
                value_pair,
                error,
            })?;
        frame.wait = wait;
        Ok(frame)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Frame {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            Frame::default(),
            pair,
            Rule::Frame,
            &[Frame::parse_number, Frame::parse_name, Frame::parse_data],
        )
        // We do not have to validate the following, as they are protected by
        // the grammar:
        //
        // * `name.is_empty()`
        // * `state == State::Uninitialized`
    }
}
