# zip-dir

![crates.io](https://img.shields.io/crates/v/zip-dir.svg)
![Docs](https://docs.rs/mio/badge.svg)

Usage
-----

```toml
[dependencies]
zip-dir = "0.1"
```

This crate provides the `zip_dir` function which can be used as follows:

```rust
use std::io::Cursor;
use std::path::Path;

use zip_dir::zip_dir;

fn create_archive() {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(buffer);
    let path = Path::new(".");
    
    // See !(http://mvdnes.github.io/rust-docs/zip-rs/zip/write/struct.FileOptions.html) for options
    // Passing None uses the default options with no compression
    let options = None;
    
    zip_dir(path, cursor, options).unwrap()
}
```
