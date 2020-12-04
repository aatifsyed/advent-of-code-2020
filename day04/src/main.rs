#[macro_use]
extern crate lazy_static;
use regex::Regex;
use regexutils::ExtractCaptured;
use std::collections::{HashMap, HashSet};

fn part1(filepath: &str) {}

fn part2(filepath: &str) {}

// https://riptutorial.com/rust/example/4149/create-a-hashset-macro
macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

// ouch https://github.com/rust-lang-nursery/lazy-static.rs/issues/119#issuecomment-419595818
lazy_static! {
    static ref KEY_VALUE_REGEX: Regex =
        Regex::new(r"(\s|^)(?P<key>\w{3}):(?P<value>\S+)").expect("Couldn't compile regex");
    static ref EXPECTED_KEYS_MEM: HashSet<String> = set![
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
        String::from("cid")
    ];
}

// Usage

trait FromLine {
    fn from_line(line: String, regex: &Regex) -> Self;
}

trait IsValid {
    fn is_valid(&self) -> bool;
}

impl FromLine for HashMap<String, String> {
    fn from_line(line: String, regex: &Regex) -> Self {
        let mut map = HashMap::new();
        for capture in regex.captures_iter(&line) {
            let key: String = capture.extract_captured("key");
            let value: String = capture.extract_captured("value");
            map.insert(key, value);
        }
        map
    }
}

impl IsValid for HashMap<String, String> {
    fn is_valid(&self) -> bool {
        let keys: HashSet<&String> = self.keys().collect();
        // keys.remove("cid");
        keys.len() == 7
    }
}

fn main() {
    // println!("part 1: {}", part1("inputs/dayXX.txt"));
    // println!("part 2: {}", part2("inputs/dayXX.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("../inputs/dayXX.txt"), ());
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("../inputs/dayXX.txt"), ());
    }
    #[test]
    fn print_lines() {
        for (i, line) in fileutils::lines_from_file("../inputs/examples/day04.txt")
            .into_iter()
            .enumerate()
        {
            println!("line {:3}, length {:2}: {:?}", i, line.len(), line);
        }
    }
    #[test]
    fn contains_no_extra_keys() {
        let map = fileutils::lines_from_file("../inputs/day04.txt")
            .into_iter()
            .map(|s| HashMap::from_line(s, &*KEY_VALUE_REGEX))
            .fold(HashMap::new(), |mut a, b| {
                a.extend(b);
                a
            });
        let keys: HashSet<&String> = map.keys().collect();
        println!("{:?}", keys);
    }
}
