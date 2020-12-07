use boolinator::Boolinator;
use std::collections::{HashMap, HashSet};

const DAY: &str = "06";

fn part1(filepath: &str) -> usize {
    fileutils::blank_line_delimited(filepath)
        .into_iter()
        .map(|s| {
            let mut y = HashSet::new();
            for c in s.chars() {
                y.insert(c);
            }
            y.remove(&'\n');
            y.len()
        })
        .sum::<usize>()
}

fn part2(filepath: &str) -> usize {
    fileutils::blank_line_delimited(filepath)
        .into_iter()
        .map(|input| {
            let num_participants = input.lines().count();
            let mut yeses = HashMap::new();
            for c in input.chars() {
                *yeses.entry(c).or_insert(0) += 1;
            }
            yeses.remove(&'\n');
            yeses
                .iter()
                .filter_map(|(_, n)| (n == &num_participants).as_some(()))
                .count()
        })
        .sum::<usize>()
}

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
        assert_eq!(part1(&format!("../inputs/day{}.txt", DAY)), 6273);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&format!("../inputs/day{}.txt", DAY)), 3254);
    }
}
