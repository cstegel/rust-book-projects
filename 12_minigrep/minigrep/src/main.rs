use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("query: {}", query);
    println!("file: {}", filename);

    let mut f = File::open(filename).expect("could not open file");

    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("contents: {}", contents);
}