[![docs](https://docs.rs/lf2_parse/badge.svg)](https://docs.rs/lf2_parse)
[![crates.io](https://img.shields.io/crates/v/lf2_parse.svg)](https://crates.io/crates/lf2_parse)

# LF2 Parse

Parses Little Fighter 2 (LF2) data files into an in-memory model.

## Usage

### Examples

```sh
cargo run --example simple

cargo run --example demo -- \
  examples/frozen.txt \
  examples/frozen.dat
```

### Library

```rust
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

```

## License

Licensed the [Zlib license](LICENSE-ZLIB.md).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.
