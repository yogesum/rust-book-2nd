#!/usr/bin/env run-cargo-script
// cargo-deps: rand="^0.5.0"

extern crate rand;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.gen();
        Point { x, y }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();

    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}