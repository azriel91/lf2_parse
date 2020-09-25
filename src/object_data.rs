use std::{
    convert::TryFrom,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use lf2_codec::DataDecoder;
use pest::{iterators::Pair, Parser};

use crate::{Error, Frames, Header, ObjectDataParser, Rule, SubRuleFn};

#[derive(Debug, Default, PartialEq)]
pub struct ObjectData {
    pub header: Header,
    pub frames: Frames,
}

impl ObjectData {
    /// Returns the object data string, decoding it if necessary.
    ///
    /// # Parameters
    ///
    /// * `path`: Path to the object data file to open.
    pub fn open(path: &Path) -> Result<String, Error<'static>> {
        // Open the file.
        let file = File::open(path).map_err(|io_error| Error::FileOpenError {
            path: path.to_owned(),
            io_error,
        })?;

        // If the file ends with `.dat`, decode it first.
        let needs_decode = path.extension().map(|ext| ext == "dat").unwrap_or(false);

        // Read the file.
        let mut buf_reader = BufReader::new(file);

        let data_decoded = if needs_decode {
            let decoded_bytes = DataDecoder::decode(buf_reader)?;
            String::from_utf8(decoded_bytes)?
        } else {
            let mut object_data_str = String::new();
            buf_reader
                .read_to_string(&mut object_data_str)
                .map_err(|io_error| Error::FileOpenError {
                    path: path.to_owned(),
                    io_error,
                })?;

            object_data_str
        };

        Ok(data_decoded)
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for ObjectData {
    type Error = Error<'i>;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let sub_rule_fns: &[SubRuleFn<ObjectData>] = &[
            |mut object_data, header_pair| {
                Header::try_from(header_pair).map(|header| {
                    object_data.header = header;
                    object_data
                })
            },
            |mut object_data, frames_pair| {
                Frames::try_from(frames_pair).map(|frames| {
                    object_data.frames = frames;
                    object_data
                })
            },
        ];

        ObjectDataParser::parse_as_type(ObjectData::default(), pair, Rule::Object, sub_rule_fns)
    }
}

impl<'s> TryFrom<&'s str> for ObjectData {
    type Error = Error<'s>;

    fn try_from(object_data_str: &'s str) -> Result<Self, Self::Error> {
        let mut object_data_pairs = ObjectDataParser::parse(Rule::Object, object_data_str)?;
        let object_data = object_data_pairs
            .next()
            .ok_or(Error::ObjectDataExpected)
            .and_then(ObjectData::try_from)?;

        // We should not have another pair.
        if object_data_pairs.peek().is_some() {
            Err(Error::ObjectDataSurplus {
                object_data,
                surplus_pairs: object_data_pairs,
            })
        } else {
            Ok(object_data)
        }
    }
}
