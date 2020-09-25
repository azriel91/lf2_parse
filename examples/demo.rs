use std::{convert::TryFrom, env, path::Path};

use lf2_parse::{Error, ObjectData};

fn run() -> Result<(), Error<'static>> {
    let mut args_os = env::args_os();

    // Skip first argument, which tends to be the application name.
    args_os.next();

    if args_os.len() == 0 {
        print_help();
    }

    args_os.try_for_each(|arg_os| {
        let path = Path::new(&arg_os);
        let contents = ObjectData::open(&path)?;

        match ObjectData::try_from(contents.as_ref()) {
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
    let app = Path::new(file!())
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap();

    eprintln!(
        "\
        Usage: `./{app} [<object.dat | object.txt> ..]`\n\
        \n\
        Examples:\n\
        \n\
        ```sh\n\
        ./{app} object.dat\n\
        ./{app} object.txt\n\
        ./{app} object.dat object.txt\n\
        ```
        ",
        app = app
    );
}
