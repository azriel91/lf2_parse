//! Parses Little Fighter 2 (LF2) data files into an in-memory model.

pub use crate::{
    element::{
        BPoint, Bdy, BdyKind, BdyKindParseError, CPoint, CPointKind, Effect, EffectParseError,
        Element, Itr, ItrKind, OPoint, OPointFacing, OPointFacingDir, OPointKind, WPoint,
        WPointKind, WPointKindParseError,
    },
    error::Error,
    frame::{Frame, FrameNumber, FrameNumberNext, Pic, State, StateParseError, Wait},
    frames::Frames,
    header::Header,
    object_data::ObjectData,
    object_data_parser::{ObjectDataParser, Rule, SubRuleFn},
    object_id::ObjectId,
    sprite_file::SpriteFile,
    weapon_strength::WeaponStrength,
    weapon_strength_index::WeaponStrengthIndex,
};

mod element;
mod error;
mod frame;
mod frames;
mod header;
mod object_data;
mod object_data_parser;
mod object_id;
mod sprite_file;
mod weapon_strength;
mod weapon_strength_index;
