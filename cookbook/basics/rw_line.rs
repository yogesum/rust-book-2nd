#!/usr/bin/env run-cargo-script

use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};

fn run() -> Result<(), io::Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffferd = BufReader::new(input);

    for line in buffferd.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    run()
}