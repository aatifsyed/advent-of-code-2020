use std::{iter::Iterator, num::ParseIntError, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
pub struct Computer<'v> {
    pub instructions: &'v Vec<Instruction>,
    pub state: ComputerState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComputerState {
    pub accumulator: isize,
    pub instruction_index: isize,
}

impl<'v> Computer<'v> {
    pub fn new(instructions: &'v Vec<Instruction>) -> Self {
        Computer {
            instructions,
            state: ComputerState {
                accumulator: 0,
                instruction_index: 0,
            },
        }
    }
}

impl<'v> Iterator for Computer<'v> {
    type Item = ComputerState;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.state.instruction_index;

        if index as usize > self.instructions.len() {
            return None;
        }

        self.state.instruction_index = match self.instructions[index as usize] {
            Instruction::Noop(_) => index + 1,
            Instruction::Accumulate(acc) => {
                self.state.accumulator += acc;
                index + 1
            }
            Instruction::Jump(jump) => index + jump,
        };
        Some(self.state)
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
                Some("nop") if args.len() == 1 => Ok(Instruction::Noop(args[0])),
                Some("acc") if args.len() == 1 => Ok(Instruction::Accumulate(args[0])),
                Some("jmp") if args.len() == 1 => Ok(Instruction::Jump(args[0])),
                _ => Err(ParseInstructionError),
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
    fn nop() {
        let instructions = vec![Instruction::Noop(-2)];
        let mut comp = Computer::new(&instructions);

        // Assert initial conditions
        assert_eq!(
            comp.state,
            ComputerState {
                accumulator: 0,
                instruction_index: 0
            }
        );

        let new_state = comp.next().unwrap();

        // Ensure what we've returned matches our internal state
        assert_eq!(new_state, comp.state);

        // Ensure we've (only) incremented the program counter as a result of the Noop
        assert_eq!(
            new_state,
            ComputerState {
                accumulator: 0,
                instruction_index: 1
            }
        );
    }
}
