#!/usr/bin/env run-cargo-script
// cargo-deps: rand="^0.5.0"

extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));

    println!("\n  Use Range to obtain values with uniform distribution\n");
    uniform_dist();
}

use rand::distributions::{Range, Distribution};

fn uniform_dist() {
    let mut rng = rand::thread_rng();
    let die = Range::new(1, 7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);

        if throw == 6 {
            break;
        }
    }
}
