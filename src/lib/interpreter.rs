use std::{
    collections::HashMap,
    io::{self, Read},
};

use super::{command::Command, program::Program};

pub struct Interpreter {
    program: Program,
    data: Vec<u8>,
    matching_brackets: HashMap<usize, usize>,
    program_ptr: usize,
    data_ptr: usize,
}

impl Interpreter {
    /// Tries to create a new `Interpreter` from a `Program`, matching its brackets.
    ///
    /// # Errors
    ///
    /// If a bracket in program is unmatched, returns an error containing its index.
    pub fn new(program: Program, data_size: usize) -> Result<Self, usize> {
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
            Command::MoveRight => self.data_ptr += 1,
            Command::MoveLeft => self.data_ptr -= 1,
            Command::Increment => {
                self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_add(1);
            }
            Command::Decrement => {
                self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_sub(1);
            }
            Command::Print => print!("{}", self.data[self.data_ptr] as char),
            Command::Read => io::stdin()
                .read_exact(&mut self.data[self.data_ptr..=(self.data_ptr)])
                .unwrap(),
            Command::JumpForward => {
                if self.data[self.data_ptr] == 0 {
                    self.program_ptr = *self.matching_brackets.get(&self.program_ptr).unwrap();
                }
            }
            Command::JumpBackwards => {
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
