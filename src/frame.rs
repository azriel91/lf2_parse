use std::{convert::TryFrom, path::PathBuf};

use derive_builder::Builder;
use pest::iterators::Pair;

use crate::{Error, FrameNumber, FrameNumberNext, ObjectDataParser, Pic, Rule, State, Wait};

#[derive(Builder, Clone, Debug, PartialEq)]
#[builder(pattern = "owned")]
pub struct Frame {
    center_x: i64,
    center_y: i64,
    d_vx: i64,
    d_vy: i64,
    d_vz: i64,
    hit_a: FrameNumberNext,
    hit_d: FrameNumberNext,
    hit_da: FrameNumberNext,
    hit_dj: FrameNumberNext,
    hit_fa: FrameNumberNext,
    hit_fj: FrameNumberNext,
    hit_j: FrameNumberNext,
    hit_ja: FrameNumberNext,
    hit_ua: FrameNumberNext,
    hit_uj: FrameNumberNext,
    mp: i64,
    next_frame: FrameNumber,
    pic: Pic,
    sound: Option<PathBuf>,
    state: State,
    wait: Wait,
}

impl Frame {
    fn parse_data<'i>(
        builder: FrameBuilder,
        frame_data_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        match frame_data_pair.as_rule() {
            Rule::FrameTag => frame_data_pair
                .into_inner()
                .try_fold(builder, Frame::parse_tag),
            _ => Err(Error::Grammar {
                rule_expected: Rule::FrameTag,
                pair_found: Some(frame_data_pair),
            }),
        }
    }

    fn parse_tag<'i>(
        builder: FrameBuilder,
        frame_tag_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            frame_tag_pair,
            Rule::FrameTag,
            &[|mut builder, frame_tag_pair| {
                match frame_tag_pair.as_rule() {
                    Rule::TagCenterX => {
                        builder = Self::parse_center_x(builder, frame_tag_pair)?;
                    }
                    Rule::TagCenterY => {
                        builder = Self::parse_center_y(builder, frame_tag_pair)?;
                    }
                    Rule::TagDVx => {
                        builder = Self::parse_d_vx(builder, frame_tag_pair)?;
                    }
                    Rule::TagDVy => {
                        builder = Self::parse_d_vy(builder, frame_tag_pair)?;
                    }
                    Rule::TagDVz => {
                        builder = Self::parse_d_vz(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitA => {
                        builder = Self::parse_hit_a(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitD => {
                        builder = Self::parse_hit_d(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitDa => {
                        builder = Self::parse_hit_da(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitDj => {
                        builder = Self::parse_hit_dj(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitFa => {
                        builder = Self::parse_hit_fa(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitFj => {
                        builder = Self::parse_hit_fj(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitJ => {
                        builder = Self::parse_hit_j(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitJa => {
                        builder = Self::parse_hit_ja(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitUa => {
                        builder = Self::parse_hit_ua(builder, frame_tag_pair)?;
                    }
                    Rule::TagHitUj => {
                        builder = Self::parse_hit_uj(builder, frame_tag_pair)?;
                    }
                    Rule::TagMp => {
                        builder = Self::parse_mp(builder, frame_tag_pair)?;
                    }
                    Rule::TagNext => {
                        builder = Self::parse_next_frame(builder, frame_tag_pair)?;
                    }
                    Rule::TagPic => {
                        builder = Self::parse_pic(builder, frame_tag_pair)?;
                    }
                    Rule::TagSound => {
                        builder = Self::parse_sound(builder, frame_tag_pair)?;
                    }
                    Rule::TagState => {
                        builder = Self::parse_state(builder, frame_tag_pair)?;
                    }
                    Rule::TagWait => {
                        builder = Self::parse_wait(builder, frame_tag_pair)?;
                    }
                    _ => {}
                }
                Ok(builder)
            }],
        )
    }

    fn parse_center_x<'i>(
        builder: FrameBuilder,
        center_x_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            center_x_pair,
            Rule::TagCenterX,
            &[|mut builder, value_pair| {
                let center_x = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(centerx),
                        value_pair,
                        error,
                    })?;
                builder = builder.center_x(center_x);
                Ok(builder)
            }],
        )
    }

    fn parse_center_y<'i>(
        builder: FrameBuilder,
        center_y_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            center_y_pair,
            Rule::TagCenterY,
            &[|mut builder, value_pair| {
                let center_y = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(centery),
                        value_pair,
                        error,
                    })?;
                builder = builder.center_y(center_y);
                Ok(builder)
            }],
        )
    }

    fn parse_d_vx<'i>(
        builder: FrameBuilder,
        d_vx_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            d_vx_pair,
            Rule::TagDVx,
            &[|mut builder, value_pair| {
                let d_vx = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(dvx),
                        value_pair,
                        error,
                    })?;
                builder = builder.d_vx(d_vx);
                Ok(builder)
            }],
        )
    }

    fn parse_d_vy<'i>(
        builder: FrameBuilder,
        d_vy_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            d_vy_pair,
            Rule::TagDVy,
            &[|mut builder, value_pair| {
                let d_vy = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(dvy),
                        value_pair,
                        error,
                    })?;
                builder = builder.d_vy(d_vy);
                Ok(builder)
            }],
        )
    }

    fn parse_d_vz<'i>(
        builder: FrameBuilder,
        d_vz_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            d_vz_pair,
            Rule::TagDVz,
            &[|mut builder, value_pair| {
                let d_vz = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(dvz),
                        value_pair,
                        error,
                    })?;
                builder = builder.d_vz(d_vz);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_a<'i>(
        builder: FrameBuilder,
        hit_a_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_a_pair,
            Rule::TagHitA,
            &[|mut builder, value_pair| {
                let hit_a = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_a),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_a(hit_a);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_d<'i>(
        builder: FrameBuilder,
        hit_d_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_d_pair,
            Rule::TagHitD,
            &[|mut builder, value_pair| {
                let hit_d = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_d),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_d(hit_d);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_da<'i>(
        builder: FrameBuilder,
        hit_da_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_da_pair,
            Rule::TagHitDa,
            &[|mut builder, value_pair| {
                let hit_da = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_Da),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_da(hit_da);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_dj<'i>(
        builder: FrameBuilder,
        hit_dj_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_dj_pair,
            Rule::TagHitDj,
            &[|mut builder, value_pair| {
                let hit_dj = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_Dj),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_dj(hit_dj);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_fa<'i>(
        builder: FrameBuilder,
        hit_fa_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_fa_pair,
            Rule::TagHitFa,
            &[|mut builder, value_pair| {
                let hit_fa = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_Fa),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_fa(hit_fa);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_fj<'i>(
        builder: FrameBuilder,
        hit_fj_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_fj_pair,
            Rule::TagHitFj,
            &[|mut builder, value_pair| {
                let hit_fj = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_Fj),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_fj(hit_fj);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_j<'i>(
        builder: FrameBuilder,
        hit_j_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_j_pair,
            Rule::TagHitJ,
            &[|mut builder, value_pair| {
                let hit_j = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_j),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_j(hit_j);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_ja<'i>(
        builder: FrameBuilder,
        hit_ja_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_ja_pair,
            Rule::TagHitJa,
            &[|mut builder, value_pair| {
                let hit_ja = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_ja),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_ja(hit_ja);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_ua<'i>(
        builder: FrameBuilder,
        hit_ua_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_ua_pair,
            Rule::TagHitUa,
            &[|mut builder, value_pair| {
                let hit_ua = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_ua),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_ua(hit_ua);
                Ok(builder)
            }],
        )
    }

    fn parse_hit_uj<'i>(
        builder: FrameBuilder,
        hit_uj_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_uj_pair,
            Rule::TagHitUj,
            &[|mut builder, value_pair| {
                let hit_uj = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(hit_uj),
                        value_pair,
                        error,
                    })?;
                builder = builder.hit_uj(hit_uj);
                Ok(builder)
            }],
        )
    }

    fn parse_mp<'i>(
        builder: FrameBuilder,
        mp_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            mp_pair,
            Rule::TagMp,
            &[|mut builder, value_pair| {
                let mp = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(mp),
                        value_pair,
                        error,
                    })?;
                builder = builder.mp(mp);
                Ok(builder)
            }],
        )
    }

    fn parse_next_frame<'i>(
        builder: FrameBuilder,
        next_frame_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            next_frame_pair,
            Rule::TagNext,
            &[|mut builder, value_pair| {
                let next_frame = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(next),
                        value_pair,
                        error,
                    })?;
                builder = builder.next_frame(next_frame);
                Ok(builder)
            }],
        )
    }

    fn parse_pic<'i>(
        builder: FrameBuilder,
        pic_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            pic_pair,
            Rule::TagPic,
            &[|mut builder, value_pair| {
                let pic = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(pic),
                        value_pair,
                        error,
                    })?;
                builder = builder.pic(pic);
                Ok(builder)
            }],
        )
    }

    fn parse_sound<'i>(
        builder: FrameBuilder,
        sound_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            sound_pair,
            Rule::TagSound,
            &[|mut builder, value_pair| {
                let sound = value_pair
                    .as_str()
                    .parse()
                    .map_err(|_error| Error::ParsePath {
                        field: stringify!(sound),
                        value_pair,
                    })?;
                builder = builder.sound(Some(sound));
                Ok(builder)
            }],
        )
    }

    fn parse_state<'i>(
        builder: FrameBuilder,
        state_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            state_pair,
            Rule::TagState,
            &[|mut builder, value_pair| {
                let state = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::StateParse { value_pair, error })?;
                builder = builder.state(state);
                Ok(builder)
            }],
        )
    }

    fn parse_wait<'i>(
        builder: FrameBuilder,
        wait_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            wait_pair,
            Rule::TagWait,
            &[|mut builder, value_pair| {
                let wait = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(wait),
                        value_pair,
                        error,
                    })?;
                builder = builder.wait(wait);
                Ok(builder)
            }],
        )
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Frame {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            FrameBuilder::default(),
            pair,
            Rule::Frame,
            &[Frame::parse_data],
        )
        .and_then(|builder| builder.build().map_err(Error::DataBuildFailed))
    }
}
