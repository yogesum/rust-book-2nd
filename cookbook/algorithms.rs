#!/usr/bin/env run-cargo-script
// cargo-deps: rand="^0.6.0"

extern crate rand;

use rand::Rng;

fn gen_rand() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}                                                                                                                                                                                                                                                                                                                       

fn gen_range() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));
}

fn gen_uniform_range() {
    use rand::distributions::{Uniform, Distribution};

    let mut rng = rand::thread_rng();
    let die = Uniform::new(1, 7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);

        if throw == 6 {
            break;
        }
    }
}

fn gen_normal_range() {
    use rand::distributions::{Normal, Distribution};

    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0);
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
}

use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R:Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.gen();
        Point { x, y }
    }
}

fn gen_custom_rand() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

fn gen_alphanum_passwd() {
    use rand::distributions::Alphanumeric;

    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}

fn gen_custom_passwd() {
    use rand::seq::SliceRandom;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    let mut rng = rand::thread_rng();
    let password: Option<String> = (0..30)
        .map(|_| Some(*CHARSET.choose(&mut rng)? as char))
        .collect();

    println!("{:?}", password);
}

fn sort_int() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

fn sort_float() {
   let mut vec = vec![1.1, 1.15, 5.5,  1.123, 2.0];
   vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
   assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]); 
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn sort_struct() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ]; 

    people.sort();
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]);

    people.sort_by(|a, b| b.age.cmp(&a.age));
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]);
}

fn print_title(title: &str) {
    println!("");
    println!("{}", title);
    println!("{}", "=".repeat(title.len())); 
}

fn main() {
    println!("A L G O R I T H M S");
    println!("‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");

    print_title("Generate random numbers");
    gen_rand();

    print_title("Generate random numbers within a range");
    gen_range();

    print_title("Generate random numbers with Uniform Distribution");
    gen_uniform_range();

    print_title("Generate random numbers with Normal Distribution");
    gen_normal_range();

    print_title("Generate random values of a custom type");
    gen_custom_rand();

    print_title("Create random passwords from alphanumeric characters");
    gen_alphanum_passwd();

    print_title("Create random passwords from user-defined characters");
    gen_custom_passwd();

    sort_int();
    sort_float();
    sort_struct();
}