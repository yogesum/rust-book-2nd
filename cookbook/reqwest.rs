// cargo-deps: reqwest

extern crate reqwest;

fn main() {
    println!("{}", reqwest::get("https://www.rust-lang.org/en-US/").unwrap().text().unwrap());
}
