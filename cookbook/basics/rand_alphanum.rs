#!/usr/bin/env run-cargo-script
// cargo-deps: rand="^0.5.0"

extern crate rand;

use rand::{thread_rng, Rng, distributions::Alphanumeric};

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}