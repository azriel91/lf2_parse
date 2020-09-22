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
use lf2_parse::{Error as Lf2ParseError, ObjectData};

fn run() -> Result<(), Error<'static>> {
    let mut args_os = env::args_os();

    // Skip first argument, which tends to be the application name.
    args_os.next();

    if args_os.len() == 0 {
        print_help();
    }

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
        match ObjectData::try_from(data_decoded.as_str()) {
            Ok(object_data) => println!("{:#?}", object_data),
            Err(e) => eprintln!("{}", e),
        }

        Result::<(), Error>::Ok(())
    })?;

    Ok(())
}

fn main() -> Result<(), Error<'static>> {
    if let Err(e) = run() {
        print_help();

        eprintln!("{}", e);
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "\
        Usage: ./demo [<object.dat | object.txt> ..] \n\
        \n\
        Examples:\n\
        \n\
            ./demo object.dat\n\
            ./demo object.txt\n\
            ./demo object.dat object.txt\n\
        "
    );
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
