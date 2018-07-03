use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_configs(&args);

    println!("query: {}", query);
    println!("file: {}", filename);

    let mut f = File::open(filename).expect("could not open file");

    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("contents: {}", contents);
}

fn parse_configs(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
