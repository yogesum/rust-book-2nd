#!/usr/bin/env run-cargo-script
// cargo-deps: rand="^0.5.0", rayon="^1.0.1"

extern crate rand;
extern crate rayon;

use rand::{FromEntropy, rngs::SmallRng};
use rayon::prelude::*;

fn main() {
    // [1]
    let mut vec = vec![String::new(); 100_000];

    // [2]
    vec.par_iter_mut().for_each(|p| {
        // [3]
        *p = SmallRng::from_entropy().next_u32().take(5).collect()
    });

    // [4]
    vec.par_sort_unstable();
}
