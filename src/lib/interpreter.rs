use super::{command::BfCommand, program::Program};

pub struct BfMachine {
    program: Program,
    data: Vec<u8>,
    program_ptr: usize,
    data_ptr: usize,
}

impl BfMachine {
    pub fn new(program: Program, data_size: usize) -> Self {
        Self {
            program,
            program_ptr: 0,
            data: vec![0u8; data_size],
            data_ptr: 0,
        }
    }

    fn step(&mut self) {
        match self.program[self.program_ptr] {
            BfCommand::MoveRight => self.data_ptr = self.data_ptr + 1,
            BfCommand::MoveLeft => self.data_ptr = self.data_ptr - 1,
            BfCommand::Increment => {
                self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_add(1)
            }
            BfCommand::Decrement => {
                self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_sub(1)
            }
            BfCommand::Print => print!("{}", self.data[self.data_ptr] as char),
            BfCommand::Read => todo!(),
            BfCommand::LoopStart => {
                if self.data[self.data_ptr] == 0 {
                    let mut bracket_nesting = 1;
                    while bracket_nesting > 0 && self.program_ptr < self.program.len() {
                        self.program_ptr += 1;
                        match self.program[self.program_ptr] {
                            BfCommand::LoopStart => bracket_nesting += 1,
                            BfCommand::LoopEnd => bracket_nesting -= 1,
                            _ => (),
                        }
                    }
                }
            }
            BfCommand::LoopEnd => {
                if self.data[self.data_ptr] != 0 {
                    let mut bracket_nesting = 1;
                    while bracket_nesting > 0 && self.program_ptr < self.program.len() {
                        self.program_ptr -= 1;
                        match self.program[self.program_ptr] {
                            BfCommand::LoopStart => bracket_nesting -= 1,
                            BfCommand::LoopEnd => bracket_nesting += 1,
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
