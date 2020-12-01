use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, day01!");
    let mut file = File::create("day1.file").expect("File error");
    file.write_all(b"Hello, day01!").expect("Write error");
}
