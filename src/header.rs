use std::{convert::TryFrom, path::PathBuf};

use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule, SpriteFile, SubRuleFn};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Header {
    pub name: String,
    pub head: PathBuf,
    pub small: PathBuf,
    pub sprite_files: Vec<SpriteFile>,
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
        header: Header,
        header_data_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        header_data_pair
            .into_inner()
            .try_fold(header, Header::parse_tag)
    }

    fn parse_tag<'i>(header: Header, header_tag_pair: Pair<'i, Rule>) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::HeaderTag,
            &[Self::parse_tag_value as SubRuleFn<_>],
        )
    }

    fn parse_tag_value<'i>(
        mut header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        match header_tag_pair.as_rule() {
            Rule::TagName => {
                header = Self::parse_name(header, header_tag_pair)?;
            }
            Rule::TagHead => {
                header = Self::parse_head(header, header_tag_pair)?;
            }
            Rule::TagSmall => {
                header = Self::parse_small(header, header_tag_pair)?;
            }
            Rule::SpriteFile => {
                let sprite_file = SpriteFile::try_from(header_tag_pair)?;
                header.sprite_files.push(sprite_file);
            }
            Rule::TagWalkingFrameRate => {
                header = Self::parse_walking_frame_rate(header, header_tag_pair)?;
            }
            Rule::TagWalkingSpeed => {
                header = Self::parse_walking_speed(header, header_tag_pair)?;
            }
            Rule::TagWalkingSpeedZ => {
                header = Self::parse_walking_speed_z(header, header_tag_pair)?;
            }
            Rule::TagRunningFrameRate => {
                header = Self::parse_running_frame_rate(header, header_tag_pair)?;
            }
            Rule::TagRunningSpeed => {
                header = Self::parse_running_speed(header, header_tag_pair)?;
            }
            Rule::TagRunningSpeedZ => {
                header = Self::parse_running_speed_z(header, header_tag_pair)?;
            }
            Rule::TagHeavyWalkingSpeed => {
                header = Self::parse_heavy_walking_speed(header, header_tag_pair)?;
            }
            Rule::TagHeavyWalkingSpeedZ => {
                header = Self::parse_heavy_walking_speed_z(header, header_tag_pair)?;
            }
            Rule::TagHeavyRunningSpeed => {
                header = Self::parse_heavy_running_speed(header, header_tag_pair)?;
            }
            Rule::TagHeavyRunningSpeedZ => {
                header = Self::parse_heavy_running_speed_z(header, header_tag_pair)?;
            }
            Rule::TagJumpHeight => {
                header = Self::parse_jump_height(header, header_tag_pair)?;
            }
            Rule::TagJumpDistance => {
                header = Self::parse_jump_distance(header, header_tag_pair)?;
            }
            Rule::TagJumpDistanceZ => {
                header = Self::parse_jump_distance_z(header, header_tag_pair)?;
            }
            Rule::TagDashHeight => {
                header = Self::parse_dash_height(header, header_tag_pair)?;
            }
            Rule::TagDashDistance => {
                header = Self::parse_dash_distance(header, header_tag_pair)?;
            }
            Rule::TagDashDistanceZ => {
                header = Self::parse_dash_distance_z(header, header_tag_pair)?;
            }
            Rule::TagRowingHeight => {
                header = Self::parse_rowing_height(header, header_tag_pair)?;
            }
            Rule::TagRowingDistance => {
                header = Self::parse_rowing_distance(header, header_tag_pair)?;
            }
            _ => {}
        }
        Ok(header)
    }

    fn parse_name<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagName,
            &[Self::parse_name_value as SubRuleFn<_>],
        )
    }

    fn parse_name_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let name = value_pair.as_str().to_string();
        header.name = name;
        Ok(header)
    }

    fn parse_head<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagHead,
            &[Self::parse_head_value as SubRuleFn<_>],
        )
    }

    fn parse_head_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let head = value_pair.as_str().parse().map_err(|_| Error::ParsePath {
            field: stringify!(walking_frame_rate),
            value_pair,
        })?;
        header.head = head;
        Ok(header)
    }

    fn parse_small<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagSmall,
            &[Self::parse_small_value as SubRuleFn<_>],
        )
    }

    fn parse_small_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let small = value_pair.as_str().parse().map_err(|_| Error::ParsePath {
            field: stringify!(walking_frame_rate),
            value_pair,
        })?;
        header.small = small;
        Ok(header)
    }

    fn parse_walking_frame_rate<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagWalkingFrameRate,
            &[Self::parse_walking_frame_rate_value as SubRuleFn<_>],
        )
    }

    fn parse_walking_frame_rate_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let walking_frame_rate = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(walking_frame_rate),
                value_pair,
                error,
            })?;
        header.walking_frame_rate = walking_frame_rate;
        Ok(header)
    }

    fn parse_walking_speed<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagWalkingSpeed,
            &[Self::parse_walking_speed_value as SubRuleFn<_>],
        )
    }

    fn parse_walking_speed_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let walking_speed = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(walking_speed),
                value_pair,
                error,
            })?;
        header.walking_speed = walking_speed;
        Ok(header)
    }

    fn parse_walking_speed_z<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagWalkingSpeedZ,
            &[Self::parse_walking_speed_z_value as SubRuleFn<_>],
        )
    }

    fn parse_walking_speed_z_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let walking_speed_z = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(walking_speed_z),
                value_pair,
                error,
            })?;
        header.walking_speed_z = walking_speed_z;
        Ok(header)
    }

    fn parse_running_frame_rate<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagRunningFrameRate,
            &[Self::parse_running_frame_rate_value as SubRuleFn<_>],
        )
    }

    fn parse_running_frame_rate_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let running_frame_rate = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(running_frame_rate),
                value_pair,
                error,
            })?;
        header.running_frame_rate = running_frame_rate;
        Ok(header)
    }

    fn parse_running_speed<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagRunningSpeed,
            &[Self::parse_running_speed_value as SubRuleFn<_>],
        )
    }

    fn parse_running_speed_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let running_speed = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(running_speed),
                value_pair,
                error,
            })?;
        header.running_speed = running_speed;
        Ok(header)
    }

    fn parse_running_speed_z<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagRunningSpeedZ,
            &[Self::parse_running_speed_z_value as SubRuleFn<_>],
        )
    }

    fn parse_running_speed_z_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let running_speed_z = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(running_speed_z),
                value_pair,
                error,
            })?;
        header.running_speed_z = running_speed_z;
        Ok(header)
    }

    fn parse_heavy_walking_speed<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagHeavyWalkingSpeed,
            &[Self::parse_heavy_walking_speed_value as SubRuleFn<_>],
        )
    }

    fn parse_heavy_walking_speed_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let heavy_walking_speed =
            value_pair
                .as_str()
                .parse()
                .map_err(|error| Error::ParseFloat {
                    field: stringify!(heavy_walking_speed),
                    value_pair,
                    error,
                })?;
        header.heavy_walking_speed = heavy_walking_speed;
        Ok(header)
    }

    fn parse_heavy_walking_speed_z<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagHeavyWalkingSpeedZ,
            &[Self::parse_heavy_walking_speed_z_value as SubRuleFn<_>],
        )
    }

    fn parse_heavy_walking_speed_z_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let heavy_walking_speed_z =
            value_pair
                .as_str()
                .parse()
                .map_err(|error| Error::ParseFloat {
                    field: stringify!(heavy_walking_speed_z),
                    value_pair,
                    error,
                })?;
        header.heavy_walking_speed_z = heavy_walking_speed_z;
        Ok(header)
    }

    fn parse_heavy_running_speed<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagHeavyRunningSpeed,
            &[Self::parse_heavy_running_speed_value as SubRuleFn<_>],
        )
    }

    fn parse_heavy_running_speed_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let heavy_running_speed =
            value_pair
                .as_str()
                .parse()
                .map_err(|error| Error::ParseFloat {
                    field: stringify!(heavy_running_speed),
                    value_pair,
                    error,
                })?;
        header.heavy_running_speed = heavy_running_speed;
        Ok(header)
    }

    fn parse_heavy_running_speed_z<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagHeavyRunningSpeedZ,
            &[Self::parse_heavy_running_speed_z_value as SubRuleFn<_>],
        )
    }

    fn parse_heavy_running_speed_z_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let heavy_running_speed_z =
            value_pair
                .as_str()
                .parse()
                .map_err(|error| Error::ParseFloat {
                    field: stringify!(heavy_running_speed_z),
                    value_pair,
                    error,
                })?;
        header.heavy_running_speed_z = heavy_running_speed_z;
        Ok(header)
    }

    fn parse_jump_height<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagJumpHeight,
            &[Self::parse_jump_height_value as SubRuleFn<_>],
        )
    }

    fn parse_jump_height_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let jump_height = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(jump_height),
                value_pair,
                error,
            })?;
        header.jump_height = jump_height;
        Ok(header)
    }

    fn parse_jump_distance<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagJumpDistance,
            &[Self::parse_jump_distance_value as SubRuleFn<_>],
        )
    }

    fn parse_jump_distance_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let jump_distance = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(jump_distance),
                value_pair,
                error,
            })?;
        header.jump_distance = jump_distance;
        Ok(header)
    }

    fn parse_jump_distance_z<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagJumpDistanceZ,
            &[Self::parse_jump_distance_z_value as SubRuleFn<_>],
        )
    }

    fn parse_jump_distance_z_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let jump_distance_z = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(jump_distance_z),
                value_pair,
                error,
            })?;
        header.jump_distance_z = jump_distance_z;
        Ok(header)
    }

    fn parse_dash_height<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagDashHeight,
            &[Self::parse_dash_height_value as SubRuleFn<_>],
        )
    }

    fn parse_dash_height_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let dash_height = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(dash_height),
                value_pair,
                error,
            })?;
        header.dash_height = dash_height;
        Ok(header)
    }

    fn parse_dash_distance<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagDashDistance,
            &[Self::parse_dash_distance_value as SubRuleFn<_>],
        )
    }

    fn parse_dash_distance_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let dash_distance = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(dash_distance),
                value_pair,
                error,
            })?;
        header.dash_distance = dash_distance;
        Ok(header)
    }

    fn parse_dash_distance_z<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagDashDistanceZ,
            &[Self::parse_dash_distance_z_value as SubRuleFn<_>],
        )
    }

    fn parse_dash_distance_z_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let dash_distance_z = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(dash_distance_z),
                value_pair,
                error,
            })?;
        header.dash_distance_z = dash_distance_z;
        Ok(header)
    }

    fn parse_rowing_height<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagRowingHeight,
            &[Self::parse_rowing_height_value as SubRuleFn<_>],
        )
    }

    fn parse_rowing_height_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let rowing_height = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(rowing_height),
                value_pair,
                error,
            })?;
        header.rowing_height = rowing_height;
        Ok(header)
    }

    fn parse_rowing_distance<'i>(
        header: Header,
        header_tag_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        ObjectDataParser::parse_as_type(
            header,
            header_tag_pair,
            Rule::TagRowingDistance,
            &[Self::parse_rowing_distance_value as SubRuleFn<_>],
        )
    }

    fn parse_rowing_distance_value<'i>(
        mut header: Header,
        value_pair: Pair<'i, Rule>,
    ) -> Result<Header, Error<'i>> {
        let rowing_distance = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseFloat {
                field: stringify!(rowing_distance),
                value_pair,
                error,
            })?;
        header.rowing_distance = rowing_distance;
        Ok(header)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Header {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            Header::default(),
            pair,
            Rule::Header,
            &[Header::parse_tags as SubRuleFn<_>],
        )
        // TODO: validate header sprite_files
    }
}
