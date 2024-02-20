use std::ops::Deref;

#[derive(PartialEq)]
pub enum BfCommand {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    LoopStart,
    LoopEnd,
}

impl TryFrom<char> for BfCommand {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::MoveRight),
            '<' => Ok(Self::MoveLeft),
            '+' => Ok(Self::Increment),
            '-' => Ok(Self::Decrement),
            '.' => Ok(Self::Print),
            ',' => Ok(Self::Read),
            '[' => Ok(Self::LoopStart),
            ']' => Ok(Self::LoopEnd),
            _ => Err(value),
        }
    }
}

pub struct Program(Vec<BfCommand>);

impl Deref for Program {
    type Target = Vec<BfCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Self(
            input
                .chars()
                .filter_map(|c| BfCommand::try_from(c).ok())
                .collect(),
        )
    }
}

impl From<&String> for Program {
    fn from(input: &String) -> Self {
        Self::from(&input[..])
    }
}

impl From<String> for Program {
    fn from(input: String) -> Self {
        Self::from(&input[..])
    }
}

pub struct BfMachine {
    program: Program,
    data: Vec<u8>,
    data_ptr: usize,
    program_ptr: usize,
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
