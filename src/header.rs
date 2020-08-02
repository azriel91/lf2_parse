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
            Rule::HeaderField,
            &[|mut builder, header_field_pair| {
                match header_field_pair.as_rule() {
                    Rule::HeaderFieldName => {
                        builder = Self::parse_name(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldHead => {
                        builder = Self::parse_head(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldSmall => {
                        builder = Self::parse_small(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldFile => {
                        let file = SpriteFile::try_from(header_field_pair)?;
                        if let Some(ref mut files) = builder.file {
                            files.push(file);
                        } else {
                            builder = builder.file(vec![file]);
                        }
                    }
                    Rule::HeaderFieldWalkingFrameRate => {
                        builder = Self::parse_walking_frame_rate(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldWalkingSpeed => {
                        builder = Self::parse_walking_speed(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldWalkingSpeedz => {
                        builder = Self::parse_walking_speedz(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldRunningFrameRate => {
                        builder = Self::parse_running_frame_rate(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldRunningSpeed => {
                        builder = Self::parse_running_speed(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldRunningSpeedz => {
                        builder = Self::parse_running_speedz(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldHeavyWalkingSpeed => {
                        builder = Self::parse_heavy_walking_speed(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldHeavyWalkingSpeedz => {
                        builder = Self::parse_heavy_walking_speedz(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldHeavyRunningSpeed => {
                        builder = Self::parse_heavy_running_speed(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldHeavyRunningSpeedz => {
                        builder = Self::parse_heavy_running_speedz(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldJumpHeight => {
                        builder = Self::parse_jump_height(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldJumpDistance => {
                        builder = Self::parse_jump_distance(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldJumpDistancez => {
                        builder = Self::parse_jump_distancez(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldDashHeight => {
                        builder = Self::parse_dash_height(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldDashDistance => {
                        builder = Self::parse_dash_distance(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldDashDistancez => {
                        builder = Self::parse_dash_distancez(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldRowingHeight => {
                        builder = Self::parse_rowing_height(builder, header_field_pair)?;
                    }
                    Rule::HeaderFieldRowingDistance => {
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
            Rule::HeaderFieldName,
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
            Rule::HeaderFieldHead,
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
            Rule::HeaderFieldSmall,
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
            Rule::HeaderFieldWalkingFrameRate,
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
            Rule::HeaderFieldWalkingSpeed,
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
            Rule::HeaderFieldWalkingSpeedz,
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
            Rule::HeaderFieldRunningFrameRate,
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
            Rule::HeaderFieldRunningSpeed,
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
            Rule::HeaderFieldRunningSpeedz,
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
            Rule::HeaderFieldHeavyWalkingSpeed,
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
            Rule::HeaderFieldHeavyWalkingSpeedz,
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
            Rule::HeaderFieldHeavyRunningSpeed,
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
            Rule::HeaderFieldHeavyRunningSpeedz,
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
            Rule::HeaderFieldJumpHeight,
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
            Rule::HeaderFieldJumpDistance,
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
            Rule::HeaderFieldJumpDistancez,
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
            Rule::HeaderFieldDashHeight,
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
            Rule::HeaderFieldDashDistance,
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
            Rule::HeaderFieldDashDistancez,
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
            Rule::HeaderFieldRowingHeight,
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
            Rule::HeaderFieldRowingDistance,
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
