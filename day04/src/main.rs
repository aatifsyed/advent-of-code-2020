#[macro_use]
extern crate lazy_static;
use regex::Regex;
use regexutils::ExtractCaptured;
use std::collections::{HashMap, HashSet};

fn part1(filepath: &str) {}

fn part2(filepath: &str) {}

// ouch https://github.com/rust-lang-nursery/lazy-static.rs/issues/119#issuecomment-419595818
lazy_static! {
    static ref KEY_VALUE_REGEX: Regex =
        Regex::new(r"(\s|^)(?P<key>\w{3}):(?P<value>\S+)").expect("Couldn't compile regex");
}

trait FromRecord {
    fn from_record(line: &String, regex: &Regex) -> Self;
}

trait IsValid {
    fn is_valid(&self) -> bool;
}

impl FromRecord for HashMap<String, String> {
    fn from_record(line: &String, regex: &Regex) -> Self {
        let mut map = HashMap::new();
        for capture in regex.captures_iter(line) {
            let key: String = capture.extract_captured("key");
            let value: String = capture.extract_captured("value");
            map.insert(key, value);
        }
        map
    }
}

impl IsValid for HashMap<String, String> {
    fn is_valid(&self) -> bool {
        let mut keys: HashSet<&String> = self.keys().clone().collect();
        let cid = String::from("cid");
        keys.remove(&cid);
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
    fn input_contains_expected_number_keys() {
        let map = fileutils::lines_from_file("../inputs/day04.txt")
            .into_iter()
            .map(|s| HashMap::from_record(&s, &*KEY_VALUE_REGEX))
            .fold(HashMap::new(), |mut a, b| {
                a.extend(b);
                a
            });
        assert_eq!(map.len(), 8)
    }
    #[test]
    fn validates_full() {
        let line = String::from(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
        );
        assert!(HashMap::from_record(&line, &*KEY_VALUE_REGEX).is_valid());
    }
    #[test]
    fn invalidates_missing_required() {
        let line =
            String::from("pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm");
        assert!(!HashMap::from_record(&line, &*KEY_VALUE_REGEX).is_valid());
    }
    #[test]
    fn validates_missing_cid() {
        let line =
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 hgt:183cm");
        assert!(HashMap::from_record(&line, &*KEY_VALUE_REGEX).is_valid());
    }
}
