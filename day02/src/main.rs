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

impl PasswordEntry {
    fn from_line(line: &str) -> PasswordEntry {
        let captures = PASSWORD_ENTRY_REGEX
            .captures(line)
            .expect("Couldn't regex on line");
        PasswordEntry {
            rule: PasswordRule {
                min_occurences: captures
                    .name("min")
                    .expect("No min!")
                    .as_str()
                    .parse::<usize>()
                    .expect("Couldn't parse min"),
                max_occurences: captures
                    .name("max")
                    .expect("No max!")
                    .as_str()
                    .parse::<usize>()
                    .expect("Couldn't parse max"),
                letter: captures
                    .name("letter")
                    .expect("No letter!")
                    .as_str()
                    .parse::<char>()
                    .expect("Couldn't parse letter"),
            },
            password: String::from(captures.name("password").expect("No password!").as_str()),
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
