use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    match read_username_from_file() {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Failed!"),
    }
}
