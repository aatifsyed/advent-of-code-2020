use crate::{Instruction, Memory, ParseInstructionError, Status};
use std::collections::HashSet;
use std::{convert::From, error, fs, path, str::FromStr};

#[derive(Debug, Clone)]
pub struct ImmutableMemory {
    pub memory: Memory,
    visited_indices: HashSet<isize>,
}

impl ImmutableMemory {
    pub fn execute_instruction(mut self) -> Self {
        let old_memory = self.memory.clone();
        let new_memory = self.memory.execute_instruction();
        assert_eq!(
            old_memory.instructions, new_memory.instructions,
            "Assumption broken: instructions have mutated!"
        );
        self.visited_indices.insert(new_memory.index);
        Self {
            memory: new_memory,
            visited_indices: self.visited_indices,
        }
    }
    pub fn new<I>(instructions: I) -> Self
    where
        I: IntoIterator<Item = Instruction>,
    {
        Self {
            memory: Memory::new(instructions),
            visited_indices: HashSet::new(),
        }
    }
    pub fn from_file<P: AsRef<path::Path>>(path: P) -> Result<Self, Box<dyn error::Error>> {
        let m: Memory = fs::read_to_string(path)?.parse()?;
        Ok(Self {
            memory: m,
            visited_indices: HashSet::new(),
        })
    }
    pub fn run(mut self) -> ImmutableStatus {
        loop {
            match ImmutableStatus::from(self.clone()) {
                ImmutableStatus::Running(_) => self = self.execute_instruction(),
                _ => return ImmutableStatus::from(self),
            }
        }
    }
}

impl FromStr for ImmutableMemory {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            memory: s.parse()?,
            visited_indices: HashSet::new(),
        })
    }
}

#[derive(Debug)]
pub enum ImmutableStatus {
    Running(ImmutableMemory),
    Halted(ImmutableMemory),
    InvalidIndex(ImmutableMemory),
    WouldRevisit(ImmutableMemory),
}

impl From<ImmutableMemory> for ImmutableStatus {
    fn from(immutable_memory: ImmutableMemory) -> Self {
        // We want to stop if the **next** instruction would be a revisit, so do a lookahead
        let lookahead = immutable_memory.memory.clone().execute_instruction();
        if immutable_memory.visited_indices.contains(&lookahead.index) {
            return ImmutableStatus::WouldRevisit(immutable_memory);
        } else {
            match Status::from(immutable_memory.memory.clone()) {
                Status::Running(_) => ImmutableStatus::Running(immutable_memory),
                Status::Halted(_) => ImmutableStatus::Halted(immutable_memory),
                Status::InvalidIndex(_) => ImmutableStatus::InvalidIndex(immutable_memory),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        let immutable_memory: ImmutableMemory =
            "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6\n"
                .parse()
                .unwrap();
        match immutable_memory.run() {
            ImmutableStatus::WouldRevisit(state) => assert_eq!(state.memory.accumulator, 5),
            _ => panic!(),
        }
    }
}
