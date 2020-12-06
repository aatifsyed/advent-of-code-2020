#[macro_use]
extern crate lazy_static;
use regex::Regex;
use regexutils::ExtractCaptured;
use std::collections::{HashMap, HashSet};
use std::fs;

fn part1(filepath: &str) -> usize {
    blank_line_delimited(filepath)
        .into_iter()
        .map(|s| HashMap::from_record(&s, &*KEY_VALUE_REGEX))
        .filter(|m| m.is_valid_shallow())
        .count()
}

fn part2(filepath: &str) -> usize {
    blank_line_delimited(filepath)
        .into_iter()
        .map(|s| HashMap::from_record(&s, &*KEY_VALUE_REGEX))
        .filter(|m| m.is_valid_deep())
        .count()
}

// I'd like to return an iterator over a string here, but I'm not good enough yet
fn blank_line_delimited(filepath: &str) -> Vec<String> {
    fs::read_to_string(filepath)
        .expect("Couldn't read")
        .split("\n\n")
        .map(String::from)
        .collect()
}

// ouch https://github.com/rust-lang-nursery/lazy-static.rs/issues/119#issuecomment-419595818
lazy_static! {
    static ref KEY_VALUE_REGEX: Regex =
        Regex::new(r"(\s|^)(?P<key>\w{3}):(?P<value>\S+)").expect("Couldn't compile kv regex");
    static ref HEX_RGB_REGEX: Regex =
        Regex::new(r"^#[0-9a-f]{6}$").expect("Couldn't compile hex regex");
    static ref VALID_EYE_COLORS: [&'static str; 7] =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    static ref PASSPORT_ID_REGEX: Regex =
        Regex::new(r"^[0-9]{9}$").expect("Couldn't compile pid regex");
}

trait FromRecord {
    fn from_record(line: &String, regex: &Regex) -> Self;
}

trait IsValid {
    fn is_valid_shallow(&self) -> bool;
    fn is_valid_deep(&self) -> bool;
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
    fn is_valid_shallow(&self) -> bool {
        let mut keys: HashSet<&String> = self.keys().clone().collect();
        let cid = String::from("cid");
        keys.remove(&cid);
        keys.len() == 7
    }
    fn is_valid_deep(&self) -> bool {
        self.is_valid_shallow()
            && self.iter().all(|(key, value)| -> bool {
                // print!("{:?}: {:?}", key, value);
                let ret = match key.as_str() {
                    "byr" => match value.parse::<usize>() {
                        Ok(num) => num >= 1920 && num <= 2002,
                        Err(_) => false,
                    },
                    "iyr" => match value.parse::<usize>() {
                        Ok(num) => num >= 2010 && num <= 2020,
                        Err(_) => false,
                    },
                    "eyr" => match value.parse::<usize>() {
                        Ok(num) => num >= 2020 && num <= 2030,
                        Err(_) => false,
                    },
                    "hgt" => {
                        let (num, unit) = (&value[..value.len() - 2], &value[value.len() - 2..]);
                        match unit {
                            "cm" => match num.parse::<usize>() {
                                Ok(num) => num >= 150 && num <= 193,
                                Err(_) => false,
                            },
                            "in" => match num.parse::<usize>() {
                                Ok(num) => num >= 59 && num <= 76,
                                Err(_) => false,
                            },
                            _ => false,
                        }
                    }
                    "hcl" => HEX_RGB_REGEX.is_match(value),
                    "ecl" => VALID_EYE_COLORS.contains(&value.as_str()),
                    "pid" => PASSPORT_ID_REGEX.is_match(value),
                    "cid" => true,
                    _ => false,
                };
                // println!(" ({:?})", ret);
                ret
            })
    }
}

fn main() {
    println!("part 1: {}", part1("inputs/day04.txt"));
    println!("part 2: {}", part2("inputs/day04.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("../inputs/day04.txt"), 213);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("../inputs/day04.txt"), 147);
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
        assert!(HashMap::from_record(&line, &*KEY_VALUE_REGEX).is_valid_shallow());
    }
    #[test]
    fn invalidates_missing_required() {
        let line =
            String::from("pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm");
        assert!(!HashMap::from_record(&line, &*KEY_VALUE_REGEX).is_valid_shallow());
    }
    #[test]
    fn validates_missing_cid() {
        let line =
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 hgt:183cm");
        assert!(HashMap::from_record(&line, &*KEY_VALUE_REGEX).is_valid_shallow());
    }
    #[test]
    fn test_blank_line_delimiting() {
        let filename = "../inputs/examples/day04.txt";
        let delimited = blank_line_delimited(filename);
        assert_eq!(delimited.len(), 4)
    }
    #[test]
    fn hex_regex() {
        assert!(HEX_RGB_REGEX.is_match("#a1b2c3"));
        assert!(!HEX_RGB_REGEX.is_match("#gggggg"));
    }
    #[test]
    fn eye_colors() {
        assert!(VALID_EYE_COLORS.contains(&"amb"));
        assert!(!VALID_EYE_COLORS.contains(&"foo"));
    }
    #[test]
    fn deep_valid_passport() {
        assert!(HashMap::from_record(
            &String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f"),
            &*KEY_VALUE_REGEX,
        )
        .is_valid_deep());
    }
}
