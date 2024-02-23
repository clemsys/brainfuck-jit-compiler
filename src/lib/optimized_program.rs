use super::{command::Command, optimized_command::OptimizedCommand, program::Program};
use std::ops::Deref;

pub struct OptimizedProgram(Vec<OptimizedCommand>);

impl Deref for OptimizedProgram {
    type Target = Vec<OptimizedCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[Command]> for OptimizedProgram {
    fn from(program: &[Command]) -> Self {
        let mut optimized_program = Vec::new();

        for command in program {
            match (optimized_program.last_mut(), command) {
                (Some(OptimizedCommand::Move(offset)), Command::MoveRight) => {
                    *offset += 1;
                }
                (Some(OptimizedCommand::Move(offset)), Command::MoveLeft) => {
                    *offset -= 1;
                }
                (Some(OptimizedCommand::Add(value)), Command::Increment) => {
                    *value = value.wrapping_add(1);
                }
                (Some(OptimizedCommand::Add(value)), Command::Decrement) => {
                    *value = value.wrapping_sub(1);
                }
                _ => {
                    optimized_program.push(command.into());
                }
            }
        }
        Self(optimized_program)
    }
}

impl From<&Program> for OptimizedProgram {
    fn from(program: &Program) -> Self {
        program.as_slice().into()
    }
}

impl From<Program> for OptimizedProgram {
    fn from(program: Program) -> Self {
        program.as_slice().into()
    }
}
