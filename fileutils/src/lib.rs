#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
use std::fs;
use std::io;
use std::io::BufRead;
use std::path;

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
