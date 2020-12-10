use std::{convert::From, iter::Iterator, num::ParseIntError, str::FromStr};
use Instruction::*;
use Status::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Memory {
    pub instructions: Vec<Instruction>,
    pub accumulator: isize,
    pub index: isize,
}

impl Memory {
    pub fn execute_instruction(mut self) -> Self {
        match self.instructions[self.index as usize] {
            Noop(_) => self.index += 1,
            Accumulate(a) => {
                self.accumulator += a;
                self.index += 1
            }
            Jump(j) => self.index += j,
        };
        self
    }
    pub fn new<I>(instructions: I) -> Self
    where
        I: IntoIterator<Item = Instruction>,
    {
        Self {
            instructions: instructions.into_iter().collect(),
            accumulator: 0,
            index: 0,
        }
    }
}

impl FromStr for Memory {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // https://users.rust-lang.org/t/solved-whats-the-proper-way-to-bubble-up-errors-from-within-closures/13400/2
        let instructions = s
            .lines()
            .map(|line: &str| line.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>();
        match instructions {
            Ok(i) => Ok(Self::new(i)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Running(Memory),
    Halted(Memory),
    InvalidIndex(Memory),
}

impl From<Memory> for Status {
    fn from(memory: Memory) -> Self {
        if memory.index as usize == memory.instructions.len() {
            Halted(memory)
        } else {
            match memory.instructions.get(memory.index as usize) {
                Some(_) => Running(memory),
                None => InvalidIndex(memory),
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseInstructionError;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Noop(isize),
    Accumulate(isize),
    Jump(isize),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let (instruction, args) = (words.next(), words);
        let args: Result<Vec<isize>, ParseIntError> = args.map(|s| s.parse()).collect();
        match args {
            Ok(args) => match instruction {
                Some("nop") if args.len() == 1 => Ok(Noop(args[0])),
                Some("acc") if args.len() == 1 => Ok(Accumulate(args[0])),
                Some("jmp") if args.len() == 1 => Ok(Jump(args[0])),
                Some(_) | None => Err(ParseInstructionError),
            },
            Err(_) => Err(ParseInstructionError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_numargs() {
        assert_eq!(Instruction::from_str("nop +1"), Ok(Instruction::Noop(1)));
        assert_eq!(
            Instruction::from_str("nop +1 +2"),
            Err(ParseInstructionError)
        );
    }
    #[test]
    fn parse_instructions() {
        let memory: Memory =
            "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6\n"
                .parse()
                .unwrap();
        assert_eq!(
            memory.instructions,
            vec![
                Noop(0),
                Accumulate(1),
                Jump(4),
                Accumulate(3),
                Jump(-3),
                Accumulate(-99),
                Accumulate(1),
                Jump(-4),
                Accumulate(6)
            ]
        );
    }
}
