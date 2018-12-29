#!/usr/bin/env run-cargo-script
// cargo-deps: rand="^0.5.0"

extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    let mut rng = thread_rng();
    let password: Option<String> = (0..30)
        .map(|_| Some(*rng.choose(CHARSET)? as char))
        .collect();

    println!("{:?}", password);
}