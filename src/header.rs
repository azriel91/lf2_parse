use std::{convert::TryFrom, path::PathBuf};

use derive_builder::Builder;
use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule, SpriteFile, SubRuleFn};

#[derive(Builder, Debug, PartialEq)]
#[builder(pattern = "owned")]
pub struct Header {
    pub name: String,
    pub head: PathBuf,
    pub small: PathBuf,
    pub file: Vec<SpriteFile>,
    pub walking_frame_rate: u32,
    pub walking_speed: f32,
    pub walking_speed_z: f32,
    pub running_frame_rate: u32,
    pub running_speed: f32,
    pub running_speed_z: f32,
    pub heavy_walking_speed: f32,
    pub heavy_walking_speed_z: f32,
    pub heavy_running_speed: f32,
    pub heavy_running_speed_z: f32,
    pub jump_height: f32,
    pub jump_distance: f32,
    pub jump_distance_z: f32,
    pub dash_height: f32,
    pub dash_distance: f32,
    pub dash_distance_z: f32,
    pub rowing_height: f32,
    pub rowing_distance: f32,
}

impl Header {
    fn parse_tags<'i>(
        builder: HeaderBuilder,
        header_data_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        header_data_pair
            .into_inner()
            .try_fold(builder, Header::parse_tag)
    }

    fn parse_tag<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::HeaderTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(
        mut builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        match header_tag_pair.as_rule() {
            Rule::TagName => {
                builder = Self::parse_name(builder, header_tag_pair)?;
            }
            Rule::TagHead => {
                builder = Self::parse_head(builder, header_tag_pair)?;
            }
            Rule::TagSmall => {
                builder = Self::parse_small(builder, header_tag_pair)?;
            }
            Rule::SpriteFile => {
                let file = SpriteFile::try_from(header_tag_pair)?;
                if let Some(ref mut files) = builder.file {
                    files.push(file);
                } else {
                    builder = builder.file(vec![file]);
                }
            }
            Rule::TagWalkingFrameRate => {
                builder = Self::parse_walking_frame_rate(builder, header_tag_pair)?;
            }
            Rule::TagWalkingSpeed => {
                builder = Self::parse_walking_speed(builder, header_tag_pair)?;
            }
            Rule::TagWalkingSpeedZ => {
                builder = Self::parse_walking_speed_z(builder, header_tag_pair)?;
            }
            Rule::TagRunningFrameRate => {
                builder = Self::parse_running_frame_rate(builder, header_tag_pair)?;
            }
            Rule::TagRunningSpeed => {
                builder = Self::parse_running_speed(builder, header_tag_pair)?;
            }
            Rule::TagRunningSpeedZ => {
                builder = Self::parse_running_speed_z(builder, header_tag_pair)?;
            }
            Rule::TagHeavyWalkingSpeed => {
                builder = Self::parse_heavy_walking_speed(builder, header_tag_pair)?;
            }
            Rule::TagHeavyWalkingSpeedZ => {
                builder = Self::parse_heavy_walking_speed_z(builder, header_tag_pair)?;
            }
            Rule::TagHeavyRunningSpeed => {
                builder = Self::parse_heavy_running_speed(builder, header_tag_pair)?;
            }
            Rule::TagHeavyRunningSpeedZ => {
                builder = Self::parse_heavy_running_speed_z(builder, header_tag_pair)?;
            }
            Rule::TagJumpHeight => {
                builder = Self::parse_jump_height(builder, header_tag_pair)?;
            }
            Rule::TagJumpDistance => {
                builder = Self::parse_jump_distance(builder, header_tag_pair)?;
            }
            Rule::TagJumpDistanceZ => {
                builder = Self::parse_jump_distance_z(builder, header_tag_pair)?;
            }
            Rule::TagDashHeight => {
                builder = Self::parse_dash_height(builder, header_tag_pair)?;
            }
            Rule::TagDashDistance => {
                builder = Self::parse_dash_distance(builder, header_tag_pair)?;
            }
            Rule::TagDashDistanceZ => {
                builder = Self::parse_dash_distance_z(builder, header_tag_pair)?;
            }
            Rule::TagRowingHeight => {
                builder = Self::parse_rowing_height(builder, header_tag_pair)?;
            }
            Rule::TagRowingDistance => {
                builder = Self::parse_rowing_distance(builder, header_tag_pair)?;
            }
            _ => {}
        }
        Ok(builder)
    }

    fn parse_name<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagName,
            &[Self::parse_name_value as SubRuleFn<_>],
        )
    }

    fn parse_name_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let name = value_pair.as_str().to_string();
        builder = builder.name(name);
        Ok(builder)
    }

    fn parse_head<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagHead,
            &[Self::parse_head_value as SubRuleFn<_>],
        )
    }

    fn parse_head_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let head = value_pair.as_str().parse().map_err(|_| Error::ParsePath {
            field: stringify!(walking_frame_rate),
            value_pair,
        })?;
        builder = builder.head(head);
        Ok(builder)
    }

    fn parse_small<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagSmall,
            &[Self::parse_small_value as SubRuleFn<_>],
        )
    }

    fn parse_small_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let small = value_pair.as_str().parse().map_err(|_| Error::ParsePath {
            field: stringify!(walking_frame_rate),
            value_pair,
        })?;
        builder = builder.small(small);
        Ok(builder)
    }

    fn parse_walking_frame_rate<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagWalkingFrameRate,
            &[Self::parse_walking_frame_rate_value as SubRuleFn<_>],
        )
    }

    fn parse_walking_frame_rate_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let walking_frame_rate = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(walking_frame_rate),
                value_pair,
                error,
            })?;
        builder = builder.walking_frame_rate(walking_frame_rate);
        Ok(builder)
    }

    fn parse_walking_speed<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagWalkingSpeed,
            &[Self::parse_walking_speed_value as SubRuleFn<_>],
        )
    }

    fn parse_walking_speed_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let walking_speed = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(walking_speed),
                value_pair,
                error,
            })?;
        builder = builder.walking_speed(walking_speed);
        Ok(builder)
    }

    fn parse_walking_speed_z<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagWalkingSpeedZ,
            &[Self::parse_walking_speed_z_value as SubRuleFn<_>],
        )
    }

    fn parse_walking_speed_z_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let walking_speed_z = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(walking_speed_z),
                value_pair,
                error,
            })?;
        builder = builder.walking_speed_z(walking_speed_z);
        Ok(builder)
    }

    fn parse_running_frame_rate<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagRunningFrameRate,
            &[Self::parse_running_frame_rate_value as SubRuleFn<_>],
        )
    }

    fn parse_running_frame_rate_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let running_frame_rate = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(running_frame_rate),
                value_pair,
                error,
            })?;
        builder = builder.running_frame_rate(running_frame_rate);
        Ok(builder)
    }

    fn parse_running_speed<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagRunningSpeed,
            &[Self::parse_running_speed_value as SubRuleFn<_>],
        )
    }

    fn parse_running_speed_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let running_speed = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(running_speed),
                value_pair,
                error,
            })?;
        builder = builder.running_speed(running_speed);
        Ok(builder)
    }

    fn parse_running_speed_z<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagRunningSpeedZ,
            &[Self::parse_running_speed_z_value as SubRuleFn<_>],
        )
    }

    fn parse_running_speed_z_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let running_speed_z = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(running_speed_z),
                value_pair,
                error,
            })?;
        builder = builder.running_speed_z(running_speed_z);
        Ok(builder)
    }

    fn parse_heavy_walking_speed<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagHeavyWalkingSpeed,
            &[Self::parse_heavy_walking_speed_value as SubRuleFn<_>],
        )
    }

    fn parse_heavy_walking_speed_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let heavy_walking_speed =
            value_pair
                .as_str()
                .parse()
                .map_err(|error| Error::ParseFloat {
                    field: stringify!(heavy_walking_speed),
                    value_pair,
                    error,
                })?;
        builder = builder.heavy_walking_speed(heavy_walking_speed);
        Ok(builder)
    }

    fn parse_heavy_walking_speed_z<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagHeavyWalkingSpeedZ,
            &[Self::parse_heavy_walking_speed_z_value as SubRuleFn<_>],
        )
    }

    fn parse_heavy_walking_speed_z_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let heavy_walking_speed_z =
            value_pair
                .as_str()
                .parse()
                .map_err(|error| Error::ParseFloat {
                    field: stringify!(heavy_walking_speed_z),
                    value_pair,
                    error,
                })?;
        builder = builder.heavy_walking_speed_z(heavy_walking_speed_z);
        Ok(builder)
    }

    fn parse_heavy_running_speed<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagHeavyRunningSpeed,
            &[Self::parse_heavy_running_speed_value as SubRuleFn<_>],
        )
    }

    fn parse_heavy_running_speed_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let heavy_running_speed =
            value_pair
                .as_str()
                .parse()
                .map_err(|error| Error::ParseFloat {
                    field: stringify!(heavy_running_speed),
                    value_pair,
                    error,
                })?;
        builder = builder.heavy_running_speed(heavy_running_speed);
        Ok(builder)
    }

    fn parse_heavy_running_speed_z<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagHeavyRunningSpeedZ,
            &[Self::parse_heavy_running_speed_z_value as SubRuleFn<_>],
        )
    }

    fn parse_heavy_running_speed_z_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let heavy_running_speed_z =
            value_pair
                .as_str()
                .parse()
                .map_err(|error| Error::ParseFloat {
                    field: stringify!(heavy_running_speed_z),
                    value_pair,
                    error,
                })?;
        builder = builder.heavy_running_speed_z(heavy_running_speed_z);
        Ok(builder)
    }

    fn parse_jump_height<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagJumpHeight,
            &[Self::parse_jump_height_value as SubRuleFn<_>],
        )
    }

    fn parse_jump_height_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let jump_height = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(jump_height),
                value_pair,
                error,
            })?;
        builder = builder.jump_height(jump_height);
        Ok(builder)
    }

    fn parse_jump_distance<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagJumpDistance,
            &[Self::parse_jump_distance_value as SubRuleFn<_>],
        )
    }

    fn parse_jump_distance_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let jump_distance = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(jump_distance),
                value_pair,
                error,
            })?;
        builder = builder.jump_distance(jump_distance);
        Ok(builder)
    }

    fn parse_jump_distance_z<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagJumpDistanceZ,
            &[Self::parse_jump_distance_z_value as SubRuleFn<_>],
        )
    }

    fn parse_jump_distance_z_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let jump_distance_z = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(jump_distance_z),
                value_pair,
                error,
            })?;
        builder = builder.jump_distance_z(jump_distance_z);
        Ok(builder)
    }

    fn parse_dash_height<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagDashHeight,
            &[Self::parse_dash_height_value as SubRuleFn<_>],
        )
    }

    fn parse_dash_height_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let dash_height = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(dash_height),
                value_pair,
                error,
            })?;
        builder = builder.dash_height(dash_height);
        Ok(builder)
    }

    fn parse_dash_distance<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagDashDistance,
            &[Self::parse_dash_distance_value as SubRuleFn<_>],
        )
    }

    fn parse_dash_distance_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let dash_distance = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(dash_distance),
                value_pair,
                error,
            })?;
        builder = builder.dash_distance(dash_distance);
        Ok(builder)
    }

    fn parse_dash_distance_z<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagDashDistanceZ,
            &[Self::parse_dash_distance_z_value as SubRuleFn<_>],
        )
    }

    fn parse_dash_distance_z_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let dash_distance_z = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(dash_distance_z),
                value_pair,
                error,
            })?;
        builder = builder.dash_distance_z(dash_distance_z);
        Ok(builder)
    }

    fn parse_rowing_height<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagRowingHeight,
            &[Self::parse_rowing_height_value as SubRuleFn<_>],
        )
    }

    fn parse_rowing_height_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let rowing_height = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(rowing_height),
                value_pair,
                error,
            })?;
        builder = builder.rowing_height(rowing_height);
        Ok(builder)
    }

    fn parse_rowing_distance<'i>(
        builder: HeaderBuilder,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_tag_pair,
            Rule::TagRowingDistance,
            &[Self::parse_rowing_distance_value as SubRuleFn<_>],
        )
    }

    fn parse_rowing_distance_value<'i>(
        mut builder: HeaderBuilder,
        value_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        let rowing_distance = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(rowing_distance),
                value_pair,
                error,
            })?;
        builder = builder.rowing_distance(rowing_distance);
        Ok(builder)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Header {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            HeaderBuilder::default(),
            pair,
            Rule::Header,
            &[Header::parse_tags as SubRuleFn<_>],
        )
        .and_then(|builder| builder.build().map_err(Error::DataBuildFailed))
    }
}
