#[macro_use]
extern crate lazy_static;
use fileutils;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct PasswordRule {
    /// For the rental, this is the minumum occurences.
    ///
    /// For OTC, this is the first position
    a: usize,
    /// For the rental, this is the maximum occurences.
    ///
    /// For OTC, this is the second position
    b: usize,
    letter: char,
}

#[derive(Debug, Eq, PartialEq)]
struct PasswordEntry {
    rule: PasswordRule,
    password: String,
}

trait ExtractsCaptured {
    fn extract_captured<T: std::str::FromStr>(&self, group_name: &str) -> T
    where
        T::Err: std::fmt::Debug;
}

impl ExtractsCaptured for regex::Captures<'_> {
    fn extract_captured<T: std::str::FromStr>(&self, group_name: &str) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.name(group_name)
            .expect(&format!("No {}!", group_name))
            .as_str()
            .parse::<T>()
            .expect(&format!("Couldn't parse {}!", group_name))
    }
}

impl PasswordEntry {
    fn from_line(line: &str) -> PasswordEntry {
        let captures = PASSWORD_ENTRY_REGEX
            .captures(line)
            .expect(&format!("Couldn't parse line {}!", line));
        PasswordEntry {
            rule: PasswordRule {
                a: captures.extract_captured("min"),
                b: captures.extract_captured("max"),
                letter: captures.extract_captured("letter"),
            },
            password: captures.extract_captured("password"),
        }
    }
    fn is_valid_at_rental(&self) -> bool {
        let num_matches = self.password.matches(self.rule.letter).count();
        num_matches >= self.rule.a && num_matches <= self.rule.b
    }
    fn is_valid_at_otc(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        // Convert from 1-based index (OTC rule) to 0-based
        (chars[self.rule.a - 1] == self.rule.letter) ^ (chars[self.rule.b - 1] == self.rule.letter)
    }
}

lazy_static! {
    static ref PASSWORD_ENTRY_REGEX: Regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w+): (?P<password>\w+)$")
            .expect("Couldn't compile regex");
}

fn part1(filepath: &str) -> usize {
    fileutils::lines_from_file(filepath)
        .into_iter()
        .filter(|s| PasswordEntry::from_line(s).is_valid_at_rental())
        .count()
}

fn part2(filepath: &str) -> usize {
    fileutils::lines_from_file(filepath)
        .into_iter()
        .filter(|s| PasswordEntry::from_line(s).is_valid_at_otc())
        .count()
}

fn main() {
    println!("part1: {}", part1("inputs/day02.txt"));
    println!("part2: {}", part2("inputs/day02.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn captures_line() {
        assert_eq!(
            PasswordEntry::from_line("7-8 p: ppppppdx"),
            PasswordEntry {
                rule: PasswordRule {
                    a: 7,
                    b: 8,
                    letter: 'p'
                },
                password: String::from("ppppppdx")
            }
        )
    }
    #[test]
    fn recognises_valid_rental() {
        assert!(PasswordEntry::from_line("1-1 a: abc").is_valid_at_rental())
    }
    #[test]
    fn recognises_invalid_rental() {
        assert!(!PasswordEntry::from_line("0-0 a: abc").is_valid_at_rental())
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1("../inputs/day02.txt"), 536)
    }
    #[test]
    fn recognises_valid_otc() {
        assert!(PasswordEntry::from_line("1-3 a: abc").is_valid_at_otc())
    }
    #[test]
    fn recognises_invalid_otc() {
        assert!(!PasswordEntry::from_line("1-3 a: aba").is_valid_at_otc())
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("../inputs/day02.txt"), 558)
    }
}
