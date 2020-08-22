use crate::SubRuleFn;
use std::{convert::TryFrom, path::PathBuf};

use derive_builder::Builder;
use pest::iterators::Pair;

use crate::{Error, FrameNumber, FrameNumberNext, ObjectDataParser, Pic, Rule, State, Wait};

#[derive(Builder, Clone, Debug, PartialEq)]
#[builder(pattern = "owned")]
pub struct Frame {
    number: FrameNumber,
    name: String,
    center_x: i64,
    center_y: i64,
    d_vx: i64,
    d_vy: i64,
    d_vz: i64,
    #[builder(default)]
    hit_a: FrameNumberNext,
    #[builder(default)]
    hit_d: FrameNumberNext,
    #[builder(default)]
    hit_da: FrameNumberNext,
    #[builder(default)]
    hit_dj: FrameNumberNext,
    #[builder(default)]
    hit_fa: FrameNumberNext,
    #[builder(default)]
    hit_fj: FrameNumberNext,
    #[builder(default)]
    hit_j: FrameNumberNext,
    #[builder(default)]
    hit_ja: FrameNumberNext,
    #[builder(default)]
    hit_ua: FrameNumberNext,
    #[builder(default)]
    hit_uj: FrameNumberNext,
    #[builder(default)]
    mp: i64,
    next_frame: FrameNumberNext,
    pic: Pic,
    #[builder(default)]
    sound: Option<PathBuf>,
    state: State,
    #[builder(default)]
    wait: Wait,
}

impl Frame {
    fn parse_number<'i>(
        builder: FrameBuilder,
        frame_number_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            frame_number_pair,
            Rule::FrameNumber,
            &[Self::parse_number_value as SubRuleFn<_>],
        )
    }

    fn parse_number_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        let number = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(frame number),
                value_pair,
                error,
            })?;
        builder = builder.number(number);
        Ok(builder)
    }

    fn parse_name<'i>(
        builder: FrameBuilder,
        frame_name_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            frame_name_pair,
            Rule::FrameName,
            &[Self::parse_name_value as SubRuleFn<_>],
        )
    }

    fn parse_name_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        let name = value_pair.as_str().to_string();
        builder = builder.name(name);
        Ok(builder)
    }

    fn parse_data<'i>(
        builder: FrameBuilder,
        frame_data_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        frame_data_pair
            .into_inner()
            .try_fold(
                builder,
                |builder, frame_tag_or_element_pair| match frame_tag_or_element_pair.as_rule() {
                    Rule::FrameTag => Frame::parse_tag(builder, frame_tag_or_element_pair),
                    Rule::Element => {
                        // TODO: implement
                        Ok(builder)
                    }
                    _ => Err(Error::Grammar {
                        rule_expected: Rule::FrameTag, // TODO: Take in multiple expected rules.
                        pair_found: Some(frame_tag_or_element_pair),
                    }),
                },
            )
    }

    fn parse_tag<'i>(
        builder: FrameBuilder,
        frame_tag_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            frame_tag_pair,
            Rule::FrameTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(
        mut builder: FrameBuilder,
        frame_tag_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_center_x<'i>(
        builder: FrameBuilder,
        center_x_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            center_x_pair,
            Rule::TagCenterX,
            &[Self::parse_center_x_value as SubRuleFn<_>],
        )
    }

    fn parse_center_x_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_center_y<'i>(
        builder: FrameBuilder,
        center_y_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            center_y_pair,
            Rule::TagCenterY,
            &[Self::parse_center_y_value as SubRuleFn<_>],
        )
    }

    fn parse_center_y_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_d_vx<'i>(
        builder: FrameBuilder,
        d_vx_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            d_vx_pair,
            Rule::TagDVx,
            &[Self::parse_d_vx_value as SubRuleFn<_>],
        )
    }

    fn parse_d_vx_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_d_vy<'i>(
        builder: FrameBuilder,
        d_vy_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            d_vy_pair,
            Rule::TagDVy,
            &[Self::parse_d_vy_value as SubRuleFn<_>],
        )
    }

    fn parse_d_vy_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_d_vz<'i>(
        builder: FrameBuilder,
        d_vz_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            d_vz_pair,
            Rule::TagDVz,
            &[Self::parse_d_vz_value as SubRuleFn<_>],
        )
    }

    fn parse_d_vz_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_a<'i>(
        builder: FrameBuilder,
        hit_a_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_a_pair,
            Rule::TagHitA,
            &[Self::parse_hit_a_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_a_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_d<'i>(
        builder: FrameBuilder,
        hit_d_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_d_pair,
            Rule::TagHitD,
            &[Self::parse_hit_d_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_d_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_da<'i>(
        builder: FrameBuilder,
        hit_da_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_da_pair,
            Rule::TagHitDa,
            &[Self::parse_hit_da_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_da_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_dj<'i>(
        builder: FrameBuilder,
        hit_dj_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_dj_pair,
            Rule::TagHitDj,
            &[Self::parse_hit_dj_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_dj_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_fa<'i>(
        builder: FrameBuilder,
        hit_fa_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_fa_pair,
            Rule::TagHitFa,
            &[Self::parse_hit_fa_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_fa_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_fj<'i>(
        builder: FrameBuilder,
        hit_fj_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_fj_pair,
            Rule::TagHitFj,
            &[Self::parse_hit_fj_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_fj_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_j<'i>(
        builder: FrameBuilder,
        hit_j_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_j_pair,
            Rule::TagHitJ,
            &[Self::parse_hit_j_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_j_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_ja<'i>(
        builder: FrameBuilder,
        hit_ja_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_ja_pair,
            Rule::TagHitJa,
            &[Self::parse_hit_ja_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_ja_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_ua<'i>(
        builder: FrameBuilder,
        hit_ua_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_ua_pair,
            Rule::TagHitUa,
            &[Self::parse_hit_ua_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_ua_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_hit_uj<'i>(
        builder: FrameBuilder,
        hit_uj_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            hit_uj_pair,
            Rule::TagHitUj,
            &[Self::parse_hit_uj_value as SubRuleFn<_>],
        )
    }

    fn parse_hit_uj_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_mp<'i>(
        builder: FrameBuilder,
        mp_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            mp_pair,
            Rule::TagMp,
            &[Self::parse_mp_value as SubRuleFn<_>],
        )
    }

    fn parse_mp_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_next_frame<'i>(
        builder: FrameBuilder,
        next_frame_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            next_frame_pair,
            Rule::TagNext,
            &[Self::parse_next_frame_value as SubRuleFn<_>],
        )
    }

    fn parse_next_frame_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_pic<'i>(
        builder: FrameBuilder,
        pic_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            pic_pair,
            Rule::TagPic,
            &[Self::parse_pic_value as SubRuleFn<_>],
        )
    }

    fn parse_pic_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }

    fn parse_sound<'i>(
        builder: FrameBuilder,
        sound_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            sound_pair,
            Rule::TagSound,
            &[Self::parse_sound_value as SubRuleFn<_>],
        )
    }

    fn parse_sound_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        let sound = value_pair
            .as_str()
            .parse()
            .map_err(|_error| Error::ParsePath {
                field: stringify!(sound),
                value_pair,
            })?;
        builder = builder.sound(Some(sound));
        Ok(builder)
    }

    fn parse_state<'i>(
        builder: FrameBuilder,
        state_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            state_pair,
            Rule::TagState,
            &[Self::parse_state_value as SubRuleFn<_>],
        )
    }

    fn parse_state_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        let state = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::StateParse { value_pair, error })?;
        builder = builder.state(state);
        Ok(builder)
    }

    fn parse_wait<'i>(
        builder: FrameBuilder,
        wait_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            wait_pair,
            Rule::TagWait,
            &[Self::parse_wait_value as SubRuleFn<_>],
        )
    }

    fn parse_wait_value<'i>(
        mut builder: FrameBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<FrameBuilder, Error<'i>> {
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
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Frame {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            FrameBuilder::default(),
            pair,
            Rule::Frame,
            &[Frame::parse_number, Frame::parse_name, Frame::parse_data],
        )
        .and_then(|builder| builder.build().map_err(Error::DataBuildFailed))
    }
}
