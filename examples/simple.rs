use std::{convert::TryFrom, env, iter::FromIterator, path::PathBuf};

use lf2_parse::{Error, ObjectData};

fn main() -> Result<(), Error<'static>> {
    let path = PathBuf::from_iter(&[env!("CARGO_MANIFEST_DIR"), "examples", "frozen.dat"]);
    let contents = ObjectData::open(&path)?;

    match ObjectData::try_from(contents.as_ref()) {
        Ok(object_data) => println!("{:?}", object_data),
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
