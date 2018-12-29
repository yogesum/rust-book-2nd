#!/usr/bin/env run-cargo-script
// cargo-deps: clap="^2.32.0", ansi_term="^0.10.0"

extern crate clap;
extern crate ansi_term;

use clap::{Arg, App};
use ansi_term::{Colour, Style};

fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Yogesh Choudhry <yogesum@gmail.com>")
        .about("Writes code for living")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("A cool file"))
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("Five less than your favorite number"))
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        },
    }

    println!("");
    println!("A N S I Terminal");
    println!("‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
    println!("This is {} in color,  {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green"));

    println!("{} and this is not", Style::new().bold().paint("This is bold"));

    println!("{}, {} and {}", 
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("This is bold"),
        Colour::Yellow.bold().paint("This is bold and colored"));
}
