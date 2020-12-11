use aocompute::{immutable::*, *};
use std::{fs, panic::catch_unwind};

const DAY: &str = "08";

fn part1(filepath: &str) -> isize {
    let immutable_memory = ImmutableMemory::from_file(filepath).unwrap();
    match immutable_memory.run() {
        ImmutableStatus::WouldRevisit(state) => return state.memory.accumulator,
        _ => panic!(),
    }
}

fn corrupt_at(index: usize, instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut instructions = instructions;
    match instructions.get(index) {
        Some(instruction) => match *instruction {
            Instruction::Noop(n) => {
                instructions[index] = Instruction::Jump(n);
                instructions
            }
            Instruction::Jump(j) => {
                instructions[index] = Instruction::Noop(j);
                instructions
            }
            _ => instructions,
        },
        _ => panic!("Out of bounds!"),
    }
}

fn part2(filepath: &str) -> isize {
    let instructions = fs::read_to_string(filepath)
        .unwrap()
        .parse::<Memory>()
        .unwrap()
        .instructions;
    for i in 0..instructions.len() {
        let corrupted_instructions = corrupt_at(i, instructions.clone());
        let immutable_memory = ImmutableMemory::new(corrupted_instructions);
        match catch_unwind(|| immutable_memory.run()) {
            Ok(result) => match result {
                ImmutableStatus::Halted(m) => return m.memory.accumulator,
                _ => continue,
            },
            _ => continue,
        }
    }
    panic!("No match found!")
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
        assert_eq!(part1(&format!("../inputs/day{}.txt", DAY)), 1941);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&format!("../inputs/day{}.txt", DAY)), 1);
    // }
    #[test]
    fn corruption() {
        let before = vec![Instruction::Accumulate(1)];
        let after = corrupt_at(0, before.clone());
        assert_eq!(before, after);

        let before = vec![Instruction::Noop(2)];
        let after = corrupt_at(0, before.clone());
        assert_eq!(after[0], Instruction::Jump(2));

        let before = vec![Instruction::Jump(3)];
        let after = corrupt_at(0, before.clone());
        assert_eq!(after[0], Instruction::Noop(3))
    }
}
