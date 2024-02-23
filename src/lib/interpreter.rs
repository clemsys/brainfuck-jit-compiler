use std::io::{self, Read};

use super::{command::Command, program::Program};

pub struct Interpreter {
    program: Program,
    data: Vec<u8>,
    program_ptr: usize,
    data_ptr: usize,
}

impl Interpreter {
    pub fn new(program: Program, data_size: usize) -> Self {
        Self {
            program,
            data: vec![0u8; data_size],
            program_ptr: 0,
            data_ptr: 0,
        }
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
                    let mut bracket_nesting = 1;
                    while bracket_nesting > 0 && self.program_ptr < self.program.len() {
                        self.program_ptr += 1;
                        match self.program[self.program_ptr] {
                            Command::JumpForward => bracket_nesting += 1,
                            Command::JumpBackwards => bracket_nesting -= 1,
                            _ => (),
                        }
                    }
                }
            }
            Command::JumpBackwards => {
                if self.data[self.data_ptr] != 0 {
                    let mut bracket_nesting = 1;
                    while bracket_nesting > 0 && self.program_ptr < self.program.len() {
                        self.program_ptr -= 1;
                        match self.program[self.program_ptr] {
                            Command::JumpForward => bracket_nesting -= 1,
                            Command::JumpBackwards => bracket_nesting += 1,
                            _ => (),
                        }
                    }
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
