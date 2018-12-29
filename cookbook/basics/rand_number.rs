#!/usr/bin/env run-cargo-script
// cargo-deps: rand="^0.5.0"

extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let num1: u8 = rng.gen();
    let num2: u16 = rng.gen();

    // Integers are uniformly distributed over the type's whole range:
    println!("Random u8: {}", num1);
    println!("Random u16: {}", num2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());

    // Floating point numbers are uniformly distributed in the half-open range [0, 1)
    println!("Random float: {}", rng.gen::<f64>());
}