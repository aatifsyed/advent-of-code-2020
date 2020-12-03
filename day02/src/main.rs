#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct PasswordRule {
    min_occurences: usize,
    max_occurences: usize,
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
            .expect("Couldn't regex on line");
        PasswordEntry {
            rule: PasswordRule {
                min_occurences: captures.extract_captured("min"),
                max_occurences: captures.extract_captured("max"),
                letter: captures.extract_captured("letter"),
            },
            password: captures.extract_captured("password"),
        }
    }
}

lazy_static! {
    static ref PASSWORD_ENTRY_REGEX: Regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w+): (?P<password>\w+)$")
            .expect("Couldn't compile regex");
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn captures_line() {
        assert_eq!(
            PasswordEntry::from_line("7-8 p: ppppppdx"),
            PasswordEntry {
                rule: PasswordRule {
                    min_occurences: 7,
                    max_occurences: 8,
                    letter: 'p'
                },
                password: String::from("ppppppdx")
            }
        )
    }
}
