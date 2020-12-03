#[macro_use]
extern crate lazy_static;
use regex::{Captures, Match, Regex};

struct PasswordRule {
    min_occurences: usize,
    max_occurences: usize,
    letter: char,
}

struct PasswordEntry {
    rule: PasswordRule,
    password: String,
}

lazy_static! {
    static ref PASSWORD_ENTRY_REGEX: Regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w+): (?P<password>\w+)$")
            .expect("Couldn't compile regex");
}

fn main() {
    // PASSWORD_ENTRY_REGEX.name("fuck")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn captures_line() {}
}
