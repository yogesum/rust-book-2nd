#!/usr/bin/env run-cargo-script
// cargo-deps: rand="^0.5.0"

extern crate rand;

use rand::distributions::{Normal, Distribution};

fn main() {
    let mut rng = rand::thread_rng();

    // mean 2, standard deviation 3
    let normal = Normal::new(2.0, 3.0);

    let v = normal.sample(&mut rng);
    println!("{} is from N(2, 9) distribution", v);
}