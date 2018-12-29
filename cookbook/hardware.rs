#!/usr/bin/env run-cargo-script
// cargo-deps: num_cpus

extern crate num_cpus;

fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}