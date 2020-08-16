use std::{
    convert::TryFrom,
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use pest::Parser;

use lf2_parse::{Error, ObjectData, ObjectDataParser, Rule};

fn parse_object_data<'file>(object_data_str: &'file str) -> Result<(), Error<'file>> {
    let mut object_data_pairs = ObjectDataParser::parse(Rule::Object, object_data_str)?;

    object_data_pairs.try_for_each::<_, Result<(), Error<'file>>>(|pair| {
        println!("{:?}", pair.as_rule());

        match pair.as_rule() {
            Rule::Object => {
                let object_data = ObjectData::try_from(pair)?;
                println!("{:?}", object_data);

                Ok(())
            }
            _ => Ok(()),
        }
    })?;

    Result::<(), Error>::Ok(())
}

fn run() -> Result<(), Error<'static>> {
    let mut args_os = env::args_os();

    // TODO: First argument may be application name, or not.
    args_os.next();

    args_os.try_for_each(|arg_os| {
        // Open the file.
        let path = AsRef::<Path>::as_ref(&arg_os);
        let file = File::open(path).map_err(|io_error| Error::FileOpenError {
            path: path.to_owned(),
            io_error,
        })?;

        // Read the file.
        let mut buf_reader = BufReader::new(file);
        let mut object_data_str = String::new();
        buf_reader
            .read_to_string(&mut object_data_str)
            .map_err(|io_error| Error::FileOpenError {
                path: path.to_owned(),
                io_error,
            })?;

        // Parse the data.
        if let Err(e) = parse_object_data(&object_data_str) {
            eprintln!("{}", e);
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
