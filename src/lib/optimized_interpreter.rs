use std::{
    collections::HashMap,
    io::{self, Read},
};

use super::{optimized_command::OptimizedCommand, optimized_program::OptimizedProgram};

pub struct OptimizedInterpreter {
    program: OptimizedProgram,
    data: Vec<u8>,
    matching_brackets: HashMap<usize, usize>,
    program_ptr: usize,
    data_ptr: usize,
}

impl OptimizedInterpreter {
    /// Tries to create a new `OptimizedInterpreter` from a `OptimizedProgram`, matching its brackets.
    ///
    /// # Errors
    ///
    /// If a bracket in program is unmatched, returns an error containing its index.
    pub fn new(program: OptimizedProgram, data_size: usize) -> Result<Self, usize> {
        let matching_brackets = program.find_matching_brackets()?;
        Ok(Self {
            program,
            data: vec![0u8; data_size],
            matching_brackets,
            program_ptr: 0,
            data_ptr: 0,
        })
    }

    fn step(&mut self) {
        match self.program[self.program_ptr] {
            OptimizedCommand::Move(n) => self.data_ptr = (self.data_ptr as i32 + n) as usize,
            OptimizedCommand::Add(n) => {
                self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_add(n);
            }
            OptimizedCommand::SetToZero => {
                self.data[self.data_ptr] = 0;
            }
            OptimizedCommand::Print => print!("{}", self.data[self.data_ptr] as char),
            OptimizedCommand::Read => io::stdin()
                .read_exact(&mut self.data[self.data_ptr..=(self.data_ptr)])
                .unwrap(),
            OptimizedCommand::JumpForward => {
                if self.data[self.data_ptr] == 0 {
                    self.program_ptr = *self.matching_brackets.get(&self.program_ptr).unwrap();
                }
            }
            OptimizedCommand::JumpBackwards => {
                if self.data[self.data_ptr] != 0 {
                    self.program_ptr = *self.matching_brackets.get(&self.program_ptr).unwrap();
                }
            }
        };
        self.program_ptr += 1;
    }

    pub fn run(&mut self) {
        while self.program_ptr < self.program.len() {
            self.step();
        }
    }
}
