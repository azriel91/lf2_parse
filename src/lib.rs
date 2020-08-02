//! Parses Little Fighter 2 (LF2) data files into an in-memory model.

pub use crate::{
    error::Error,
    header::Header,
    object_data::ObjectData,
    object_data_parser::{ObjectDataParser, Rule},
    sprite_file::SpriteFile,
};

mod error;
mod header;
mod object_data;
mod object_data_parser;
mod sprite_file;
