use std::{convert::TryFrom, path::PathBuf};

use derive_builder::Builder;
use pest::iterators::Pair;

use crate::{Error, ObjectDataParser, Rule};

#[derive(Builder, Debug, PartialEq)]
#[builder(pattern = "owned")]
pub struct SpriteFile {
    path: PathBuf,
    w: u32,
    h: u32,
    row: u32,
    col: u32,
}

impl SpriteFile {
    fn parse_path<'i>(
        builder: SpriteFileBuilder,
        path_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFileBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            path_pair,
            Rule::TagFileValue,
            &[|mut builder, value_pair| {
                let path = value_pair.as_str().parse().map_err(|_| Error::ParsePath {
                    field: stringify!(path),
                    value_pair,
                })?;
                builder = builder.path(path);
                Ok(builder)
            }],
        )
    }

    fn parse_w<'i>(
        builder: SpriteFileBuilder,
        w_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFileBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            w_pair,
            Rule::TagW,
            &[|mut builder, value_pair| {
                let w = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(w),
                        value_pair,
                        error,
                    })?;
                builder = builder.w(w);
                Ok(builder)
            }],
        )
    }

    fn parse_h<'i>(
        builder: SpriteFileBuilder,
        h_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFileBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            h_pair,
            Rule::TagH,
            &[|mut builder, value_pair| {
                let h = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(h),
                        value_pair,
                        error,
                    })?;
                builder = builder.h(h);
                Ok(builder)
            }],
        )
    }

    fn parse_row<'i>(
        builder: SpriteFileBuilder,
        row_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFileBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            row_pair,
            Rule::TagRow,
            &[|mut builder, value_pair| {
                let row = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(row),
                        value_pair,
                        error,
                    })?;
                builder = builder.row(row);
                Ok(builder)
            }],
        )
    }

    fn parse_col<'i>(
        builder: SpriteFileBuilder,
        col_pair: Pair<'i, Rule>,
    ) -> Result<SpriteFileBuilder, Error<'i>> {
        ObjectDataParser::parse_as_type(
            builder,
            col_pair,
            Rule::TagCol,
            &[|mut builder, value_pair| {
                let col = value_pair
                    .as_str()
                    .parse()
                    .map_err(|error| Error::ParseInt {
                        field: stringify!(col),
                        value_pair,
                        error,
                    })?;
                builder = builder.col(col);
                Ok(builder)
            }],
        )
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for SpriteFile {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        ObjectDataParser::parse_as_type::<'i, '_, _>(
            SpriteFileBuilder::default(),
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
        .and_then(|builder| builder.build().map_err(Error::DataBuildFailed))
    }
}