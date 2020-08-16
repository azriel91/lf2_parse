//! Parses Little Fighter 2 (LF2) data files into an in-memory model.

pub use crate::{
    error::Error,
    frame_number::FrameNumber,
    frame_number_next::FrameNumberNext,
    header::Header,
    object_data::ObjectData,
    object_data_parser::{ObjectDataParser, Rule},
    pic::Pic,
    sprite_file::SpriteFile,
    state::State,
    wait::Wait,
};

mod error;
mod frame_number;
mod frame_number_next;
mod header;
mod object_data;
mod object_data_parser;
mod pic;
mod sprite_file;
mod state;
mod wait;
