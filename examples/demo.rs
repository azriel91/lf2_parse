use std::{
    convert::TryFrom,
    env,
    fmt::{self, Display},
    fs::File,
    io::{BufReader, Error as IoError, Read},
    path::Path,
    string::FromUtf8Error,
};

use lf2_codec::DataDecoder;
use lf2_parse::{Error as Lf2ParseError, ObjectData, ObjectDataParser, Rule};
use pest::Parser;

fn parse_object_data<'file>(object_data_str: &'file str) -> Result<(), Lf2ParseError<'file>> {
    let mut object_data_pairs = ObjectDataParser::parse(Rule::Object, object_data_str)?;

    object_data_pairs.try_for_each::<_, Result<(), Lf2ParseError<'file>>>(|pair| {
        match pair.as_rule() {
            Rule::Object => {
                let object_data = ObjectData::try_from(pair)?;
                println!(
                    "Name: {}\nFrames:\n{:#?}",
                    object_data.header.name, object_data.frames
                );

                Ok(())
            }
            _ => Ok(()),
        }
    })?;

    Result::<(), Lf2ParseError>::Ok(())
}

fn run() -> Result<(), Error<'static>> {
    let mut args_os = env::args_os();

    // Skip first argument, which tends to be the application name.
    args_os.next();

    args_os.try_for_each(|arg_os| {
        // Open the file.
        let path = AsRef::<Path>::as_ref(&arg_os);
        let file = File::open(path).map_err(|io_error| Lf2ParseError::FileOpenError {
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
                .map_err(|io_error| Lf2ParseError::FileOpenError {
                    path: path.to_owned(),
                    io_error,
                })?;

            object_data_str
        };

        // Parse the data.
        if let Err(e) = parse_object_data(&data_decoded) {
            println!("{}", e);
        }

        Result::<(), Error>::Ok(())
    })?;

    Ok(())
}

fn main() -> Result<(), Error<'static>> {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error<'e> {
    IoError(IoError),
    FromUtf8Error(FromUtf8Error),
    Lf2ParseError(Lf2ParseError<'e>),
}

impl<'e> From<IoError> for Error<'e> {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl<'e> From<FromUtf8Error> for Error<'e> {
    fn from(e: FromUtf8Error) -> Self {
        Self::FromUtf8Error(e)
    }
}

impl<'e> From<Lf2ParseError<'e>> for Error<'e> {
    fn from(e: Lf2ParseError<'e>) -> Self {
        Self::Lf2ParseError(e)
    }
}

impl<'e> Display for Error<'e> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{}", e),
            Self::FromUtf8Error(e) => write!(f, "{}", e),
            Self::Lf2ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl<'e> std::error::Error for Error<'e> {}
