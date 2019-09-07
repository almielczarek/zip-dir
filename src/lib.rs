//! # Example
//! ```
//! use std::fs::File;
//! use std::io;
//! use std::path::Path;
//! 
//! use zip_dir::zip_dir;
//! 
//! fn run() -> io::Result<()> {
//!     let path = Path::new(".");
//!     let file = File::create("archive.zip")?;
//! 
//!     // See http://mvdnes.github.io/rust-docs/zip-rs/zip/write/struct.FileOptions.html for options
//!     // Passing None uses the default options with no compression
//!     let options = None;
//! 
//!     // zip_dir returns its second parameter wrapped in a Result if successful
//!     let _file = zip_dir(path, file, options)?;
//! 
//!     Ok(())
//! }
//!
//! fn main() {
//!     match run() {
//!         Ok(()) => println!("Wrote archive.zip"),
//!         Err(e) => eprintln!("Error: {}", e)
//!     }
//! }
//! ```

use std::fs::{read_dir, DirEntry, File};
use std::io::{self, Read, Seek, Write};
use std::path::Path;

use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

pub fn zip_dir<T: Write + Seek>(
    path: &Path,
    target: T,
    options: Option<FileOptions>,
) -> ZipResult<T> {
    let mut zip = ZipWriter::new(target);

    let options =
        options.unwrap_or(FileOptions::default().compression_method(CompressionMethod::Stored));

    for entry in read_dir(path)? {
        zip_entry(&mut zip, entry?, options)?;
    }

    zip.finish()
}

fn zip_entry<T: Write + Seek>(
    zip: &mut ZipWriter<T>,
    entry: DirEntry,
    options: FileOptions,
) -> io::Result<()> {
    let path = entry.path();

    if path.is_dir() {
        for entry in read_dir(path)? {
            zip_entry(zip, entry?, options)?;
        }
    } else {
        zip.start_file_from_path(&path, options)?;

        let mut file = File::open(path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        zip.write(&buffer)?;
    }

    Ok(())
}
