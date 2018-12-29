#!/usr/bin/env run-cargo-script
// cargo-deps: flate2="^1.0.5", tar="^0.4.20"

extern crate flate2;
extern crate tar;

use std::fs::File;

fn extract() -> Result<(), std::io::Error> {
    use flate2::read::GzDecoder;
    use tar::Archive;
    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

fn compress() -> Result<(), std::io::Error> {
    use flate2::{Compression, write::GzEncoder};

    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "./concurrency")?;
    Ok(())
}

#[derive(Debug)]
enum ExtractError {
    Io(std::io::Error),
    StripPrefixError(std::path::StripPrefixError),
}

impl From<std::io::Error> for ExtractError {
    fn from(error: std::io::Error) -> Self {
        ExtractError::Io(error)
    }
}

impl From<std::path::StripPrefixError> for ExtractError {
    fn from(error: std::path::StripPrefixError) -> Self {
        ExtractError::StripPrefixError(error)
    }
}

fn extract_flat() -> Result<(), ExtractError> {
    use std::path::PathBuf;
    use flate2::read::GzDecoder;
    use tar::Archive;

    let file = File::open("archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "backup/logs";

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, ExtractError> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}

fn main() -> Result<(), ExtractError> {
    extract()?;
    compress()?;
    extract_flat()?;

    Ok(())
}