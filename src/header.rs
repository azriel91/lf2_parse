use std::{convert::TryFrom, path::PathBuf};

use derive_builder::Builder;
use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule, SpriteFile};

#[derive(Builder, Debug, PartialEq)]
#[builder(pattern = "owned")]
pub struct Header {
    name: String,
    head: PathBuf,
    small: PathBuf,
    file: Vec<SpriteFile>,
    walking_frame_rate: u32,
    walking_speed: f32,
    walking_speedz: f32,
    running_frame_rate: u32,
    running_speed: f32,
    running_speedz: f32,
    heavy_walking_speed: f32,
    heavy_walking_speedz: f32,
    heavy_running_speed: f32,
    heavy_running_speedz: f32,
    jump_height: f32,
    jump_distance: f32,
    jump_distancez: f32,
    dash_height: f32,
    dash_distance: f32,
    dash_distancez: f32,
    rowing_height: f32,
    rowing_distance: f32,
}

impl Header {
    fn parse_fields<'i>(
        builder: HeaderBuilder,
        header_data_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        header_data_pair
            .into_inner()
            .try_fold(builder, Header::parse_field)
    }

    fn parse_field<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::HeaderTag,
            &[|mut builder, header_field_pair| {
                match header_field_pair.as_rule() {
                    Rule::TagName => {
                        builder = Self::parse_name(builder, header_field_pair)?;
                    }
                    Rule::TagHead => {
                        builder = Self::parse_head(builder, header_field_pair)?;
                    }
                    Rule::TagSmall => {
                        builder = Self::parse_small(builder, header_field_pair)?;
                    }
                    Rule::SpriteFile => {
                        let file = SpriteFile::try_from(header_field_pair)?;
                        if let Some(ref mut files) = builder.file {
                            files.push(file);
                        } else {
                            builder = builder.file(vec![file]);
                        }
                    }
                    Rule::TagWalkingFrameRate => {
                        builder = Self::parse_walking_frame_rate(builder, header_field_pair)?;
                    }
                    Rule::TagWalkingSpeed => {
                        builder = Self::parse_walking_speed(builder, header_field_pair)?;
                    }
                    Rule::TagWalkingSpeedz => {
                        builder = Self::parse_walking_speedz(builder, header_field_pair)?;
                    }
                    Rule::TagRunningFrameRate => {
                        builder = Self::parse_running_frame_rate(builder, header_field_pair)?;
                    }
                    Rule::TagRunningSpeed => {
                        builder = Self::parse_running_speed(builder, header_field_pair)?;
                    }
                    Rule::TagRunningSpeedz => {
                        builder = Self::parse_running_speedz(builder, header_field_pair)?;
                    }
                    Rule::TagHeavyWalkingSpeed => {
                        builder = Self::parse_heavy_walking_speed(builder, header_field_pair)?;
                    }
                    Rule::TagHeavyWalkingSpeedz => {
                        builder = Self::parse_heavy_walking_speedz(builder, header_field_pair)?;
                    }
                    Rule::TagHeavyRunningSpeed => {
                        builder = Self::parse_heavy_running_speed(builder, header_field_pair)?;
                    }
                    Rule::TagHeavyRunningSpeedz => {
                        builder = Self::parse_heavy_running_speedz(builder, header_field_pair)?;
                    }
                    Rule::TagJumpHeight => {
                        builder = Self::parse_jump_height(builder, header_field_pair)?;
                    }
                    Rule::TagJumpDistance => {
                        builder = Self::parse_jump_distance(builder, header_field_pair)?;
                    }
                    Rule::TagJumpDistancez => {
                        builder = Self::parse_jump_distancez(builder, header_field_pair)?;
                    }
                    Rule::TagDashHeight => {
                        builder = Self::parse_dash_height(builder, header_field_pair)?;
                    }
                    Rule::TagDashDistance => {
                        builder = Self::parse_dash_distance(builder, header_field_pair)?;
                    }
                    Rule::TagDashDistancez => {
                        builder = Self::parse_dash_distancez(builder, header_field_pair)?;
                    }
                    Rule::TagRowingHeight => {
                        builder = Self::parse_rowing_height(builder, header_field_pair)?;
                    }
                    Rule::TagRowingDistance => {
                        builder = Self::parse_rowing_distance(builder, header_field_pair)?;
                    }
                    _ => {}
                }
                Ok(builder)
            }],
        )
    }

    fn parse_name<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagName,
            &[|mut builder, value_pair| {
                let name = value_pair.as_str().to_string();
                builder = builder.name(name);
                Ok(builder)
            }],
        )
    }

    fn parse_head<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagHead,
            &[|mut builder, value_pair| {
                let head = value_pair.as_str().parse().map_err(|_| Error::ParsePath {
                    field: stringify!(walking_frame_rate),
                    value_pair,
                })?;
                builder = builder.head(head);
                Ok(builder)
            }],
        )
    }

    fn parse_small<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagSmall,
            &[|mut builder, value_pair| {
                let small = value_pair.as_str().parse().map_err(|_| Error::ParsePath {
                    field: stringify!(walking_frame_rate),
                    value_pair,
                })?;
                builder = builder.small(small);
                Ok(builder)
            }],
        )
    }

    fn parse_walking_frame_rate<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagWalkingFrameRate,
            &[|mut builder, value_pair| {
                let walking_frame_rate =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseInt {
                            field: stringify!(walking_frame_rate),
                            value_pair,
                            error,
                        })?;
                builder = builder.walking_frame_rate(walking_frame_rate);
                Ok(builder)
            }],
        )
    }

    fn parse_walking_speed<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagWalkingSpeed,
            &[|mut builder, value_pair| {
                let walking_speed =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(walking_speed),
                            value_pair,
                            error,
                        })?;
                builder = builder.walking_speed(walking_speed);
                Ok(builder)
            }],
        )
    }

    fn parse_walking_speedz<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagWalkingSpeedz,
            &[|mut builder, value_pair| {
                let walking_speedz =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(walking_speedz),
                            value_pair,
                            error,
                        })?;
                builder = builder.walking_speedz(walking_speedz);
                Ok(builder)
            }],
        )
    }

    fn parse_running_frame_rate<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagRunningFrameRate,
            &[|mut builder, value_pair| {
                let running_frame_rate =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseInt {
                            field: stringify!(running_frame_rate),
                            value_pair,
                            error,
                        })?;
                builder = builder.running_frame_rate(running_frame_rate);
                Ok(builder)
            }],
        )
    }

    fn parse_running_speed<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagRunningSpeed,
            &[|mut builder, value_pair| {
                let running_speed =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(running_speed),
                            value_pair,
                            error,
                        })?;
                builder = builder.running_speed(running_speed);
                Ok(builder)
            }],
        )
    }

    fn parse_running_speedz<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagRunningSpeedz,
            &[|mut builder, value_pair| {
                let running_speedz =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(running_speedz),
                            value_pair,
                            error,
                        })?;
                builder = builder.running_speedz(running_speedz);
                Ok(builder)
            }],
        )
    }

    fn parse_heavy_walking_speed<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagHeavyWalkingSpeed,
            &[|mut builder, value_pair| {
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
            }],
        )
    }

    fn parse_heavy_walking_speedz<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagHeavyWalkingSpeedz,
            &[|mut builder, value_pair| {
                let heavy_walking_speedz =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(heavy_walking_speedz),
                            value_pair,
                            error,
                        })?;
                builder = builder.heavy_walking_speedz(heavy_walking_speedz);
                Ok(builder)
            }],
        )
    }

    fn parse_heavy_running_speed<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagHeavyRunningSpeed,
            &[|mut builder, value_pair| {
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
            }],
        )
    }

    fn parse_heavy_running_speedz<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagHeavyRunningSpeedz,
            &[|mut builder, value_pair| {
                let heavy_running_speedz =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(heavy_running_speedz),
                            value_pair,
                            error,
                        })?;
                builder = builder.heavy_running_speedz(heavy_running_speedz);
                Ok(builder)
            }],
        )
    }

    fn parse_jump_height<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagJumpHeight,
            &[|mut builder, value_pair| {
                let jump_height =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(jump_height),
                            value_pair,
                            error,
                        })?;
                builder = builder.jump_height(jump_height);
                Ok(builder)
            }],
        )
    }

    fn parse_jump_distance<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagJumpDistance,
            &[|mut builder, value_pair| {
                let jump_distance =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(jump_distance),
                            value_pair,
                            error,
                        })?;
                builder = builder.jump_distance(jump_distance);
                Ok(builder)
            }],
        )
    }

    fn parse_jump_distancez<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagJumpDistancez,
            &[|mut builder, value_pair| {
                let jump_distancez =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(jump_distancez),
                            value_pair,
                            error,
                        })?;
                builder = builder.jump_distancez(jump_distancez);
                Ok(builder)
            }],
        )
    }

    fn parse_dash_height<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagDashHeight,
            &[|mut builder, value_pair| {
                let dash_height =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(dash_height),
                            value_pair,
                            error,
                        })?;
                builder = builder.dash_height(dash_height);
                Ok(builder)
            }],
        )
    }

    fn parse_dash_distance<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagDashDistance,
            &[|mut builder, value_pair| {
                let dash_distance =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(dash_distance),
                            value_pair,
                            error,
                        })?;
                builder = builder.dash_distance(dash_distance);
                Ok(builder)
            }],
        )
    }

    fn parse_dash_distancez<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagDashDistancez,
            &[|mut builder, value_pair| {
                let dash_distancez =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(dash_distancez),
                            value_pair,
                            error,
                        })?;
                builder = builder.dash_distancez(dash_distancez);
                Ok(builder)
            }],
        )
    }

    fn parse_rowing_height<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagRowingHeight,
            &[|mut builder, value_pair| {
                let rowing_height =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(rowing_height),
                            value_pair,
                            error,
                        })?;
                builder = builder.rowing_height(rowing_height);
                Ok(builder)
            }],
        )
    }

    fn parse_rowing_distance<'i>(
        builder: HeaderBuilder,
        header_field_pair: Pair<'i, Rule>,
    ) -> Result<HeaderBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            header_field_pair,
            Rule::TagRowingDistance,
            &[|mut builder, value_pair| {
                let rowing_distance =
                    value_pair
                        .as_str()
                        .parse()
                        .map_err(|error| Error::ParseFloat {
                            field: stringify!(rowing_distance),
                            value_pair,
                            error,
                        })?;
                builder = builder.rowing_distance(rowing_distance);
                Ok(builder)
            }],
        )
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Header {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            HeaderBuilder::default(),
            pair,
            Rule::Header,
            &[Header::parse_fields],
        )
        .and_then(|builder| builder.build().map_err(Error::DataBuildFailed))
    }
}
