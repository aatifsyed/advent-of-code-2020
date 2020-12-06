use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

// tuple struct
#[derive(Debug, Eq, PartialEq)]
struct BoardingPass {
    id: usize,
}

// https://doc.rust-lang.org/std/str/trait.FromStr.html
impl FromStr for BoardingPass {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // https://github.com/dimbleby/advent-of-code-2020/blob/master/src/day05.rs
        let id = s.chars().fold(0, |accumulator, character| match character {
            'F' | 'L' => 2 * accumulator,
            'B' | 'R' => 1 + 2 * accumulator,
            _ => panic!("Invalid characters in boarding pass"),
        });
        Ok(BoardingPass { id })
    }
}

fn part1(filepath: &str) -> usize {
    fileutils::lines_from_file(filepath)
        .into_iter()
        .map(|s| BoardingPass::from_str(&s))
        .max()
}

fn part2(filepath: &str) {}

const DAY: &str = "XX";
fn main() {
    let filepath = format!("inputs/day{}.txt", DAY);
    println!("part 1: {:?}", part1(&filepath));
    println!("part 2: {:?}", part2(&filepath));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(&format!("../inputs/day{}.txt", DAY)), ());
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&format!("../inputs/day{}.txt", DAY)), ());
    }
}
