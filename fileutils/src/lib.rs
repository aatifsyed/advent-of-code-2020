use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path;
use std::result;

/// https://stackoverflow.com/a/35820003
pub fn lines_from_file(filename: impl AsRef<path::Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

pub fn numbers_from_file(filename: impl AsRef<path::Path>) -> Vec<isize> {
    lines_from_file(filename)
        .into_iter()
        .map(|line| line.parse::<isize>().expect("Couldn't parse line"))
        .collect()
}

#[derive(Debug)]
pub struct RectangularCharGrid {
    pub buffer: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl RectangularCharGrid {
    pub fn from_file(filename: impl AsRef<path::Path>) -> RectangularCharGrid {
        let lines = lines_from_file(filename);
        let width = lines[0].len();
        let height = lines.len();
        RectangularCharGrid {
            buffer: lines
                .into_iter()
                .flat_map(|line: String| -> Vec<char> {
                    assert_eq!(line.len(), width, "This file has non-constant width!");
                    line.chars().collect()
                })
                .collect(),
            width,
            height,
        }
    }
}

impl fmt::Display for RectangularCharGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for h in 0..self.height {
            for w in 0..self.width {
                let index = h * self.width + w;
                if let result::Result::Err(e) = write!(f, "{}", self.buffer[index]) {
                    return result::Result::Err(e);
                }
            }
            println!();
        }
        fmt::Result::Ok(())
    }
}

// Wish I could make this an iterator, but not good enough
pub fn blank_line_delimited(filepath: &str) -> Vec<String> {
    fs::read_to_string(filepath)
        .expect("Couldn't read")
        .split("\n\n")
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
