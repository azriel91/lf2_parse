use std::{convert::TryFrom, path::PathBuf};

use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule, SubRuleFn};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpriteFile {
    path: PathBuf,
    w: u32,
    h: u32,
    row: u32,
    col: u32,
}

impl SpriteFile {
    fn parse_path<'i>(
        sprite_file: SpriteFile,
        path_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        ObjectDataParser::parse_as_type(
            sprite_file,
            path_pair,
            Rule::TagFileValue,
            &[Self::parse_path_value as SubRuleFn<_>],
        )
    }

    fn parse_path_value<'i>(
        mut sprite_file: SpriteFile,
        value_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        let path = value_pair.as_str().parse().map_err(|_| Error::ParsePath {
            field: stringify!(path),
            value_pair,
        })?;
        sprite_file.path = path;
        Ok(sprite_file)
    }

    fn parse_w<'i>(
        sprite_file: SpriteFile,
        w_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        ObjectDataParser::parse_as_type(
            sprite_file,
            w_pair,
            Rule::TagW,
            &[Self::parse_w_value as SubRuleFn<_>],
        )
    }

    fn parse_w_value<'i>(
        mut sprite_file: SpriteFile,
        value_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        let w = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(w),
                value_pair,
                error,
            })?;
        sprite_file.w = w;
        Ok(sprite_file)
    }

    fn parse_h<'i>(
        sprite_file: SpriteFile,
        h_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        ObjectDataParser::parse_as_type(
            sprite_file,
            h_pair,
            Rule::TagH,
            &[Self::parse_h_value as SubRuleFn<_>],
        )
    }

    fn parse_h_value<'i>(
        mut sprite_file: SpriteFile,
        value_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        let h = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(h),
                value_pair,
                error,
            })?;
        sprite_file.h = h;
        Ok(sprite_file)
    }

    fn parse_row<'i>(
        sprite_file: SpriteFile,
        row_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        ObjectDataParser::parse_as_type(
            sprite_file,
            row_pair,
            Rule::TagRow,
            &[Self::parse_row_value as SubRuleFn<_>],
        )
    }

    fn parse_row_value<'i>(
        mut sprite_file: SpriteFile,
        value_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        let row = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(row),
                value_pair,
                error,
            })?;
        sprite_file.row = row;
        Ok(sprite_file)
    }

    fn parse_col<'i>(
        sprite_file: SpriteFile,
        col_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        ObjectDataParser::parse_as_type(
            sprite_file,
            col_pair,
            Rule::TagCol,
            &[Self::parse_col_value as SubRuleFn<_>],
        )
    }

    fn parse_col_value<'i>(
        mut sprite_file: SpriteFile,
        value_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFile, Error<'i>> {
        let col = value_pair
            .as_str()
            .parse()
            .map_err(|error| Error::ParseInt {
                field: stringify!(col),
                value_pair,
                error,
            })?;
        sprite_file.col = col;
        Ok(sprite_file)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for SpriteFile {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type(
            SpriteFile::default(),
            pair,
            Rule::SpriteFile,
            &[
                Self::parse_path,
                Self::parse_w,
                Self::parse_h,
                Self::parse_row,
                Self::parse_col,
            ],
        )
    }
}
