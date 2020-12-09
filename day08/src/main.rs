use aocompute::{Computer, ComputerState, Instruction};
use std::{collections::HashSet, str::FromStr};

const DAY: &str = "08";

fn part1(filepath: &str) -> isize {
    let instructions = fileutils::lines_from_file(filepath)
        .iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();
    let computer = Computer::new(&instructions);
    let mut accumulator = computer.state.accumulator;
    let mut passed_indices = HashSet::new();
    passed_indices.insert(computer.state.instruction_index);
    for state in computer {
        if passed_indices.contains(&state.instruction_index) {
            break;
        }
        accumulator = state.accumulator;
        passed_indices.insert(state.instruction_index);
    }
    accumulator
}

fn part2(filepath: &str) {}

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
        assert_eq!(part1(&format!("../inputs/day{}.txt", DAY)), 1941);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&format!("../inputs/day{}.txt", DAY)), ());
    }
    #[test]
    fn test_example() {
        assert_eq!(part1("../inputs/examples/day08.txt"), 5);
    }
}
