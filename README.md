# zip-dir

![crates.io](https://img.shields.io/crates/v/zip-dir.svg)
![Docs](https://docs.rs/zip-dir/badge.svg)

Usage
-----

```toml
[dependencies]
zip-dir = "0.1"
```

This crate provides the `zip_dir` function which can be used as follows:

```rust
use std::fs::File;
use std::io;
use std::path::Path;

use zip_dir::zip_dir;

fn run() -> io::Result<()> {
    let path = Path::new(".");
    let file = File::create("archive.zip")?;

    // See http://mvdnes.github.io/rust-docs/zip-rs/zip/write/struct.FileOptions.html for options
    // Passing None uses the default options with no compression
    let options = None;

    // zip_dir returns its second parameter wrapped in a Result if successful
    let _file = zip_dir(path, file, options)?;

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => println!("Wrote archive.zip"),
        Err(e) => eprintln!("Error: {}", e)
    }
}
```
