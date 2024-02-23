use super::{command::Command, optimized_command::OptimizedCommand, program::Program};
use std::{collections::HashMap, ops::Deref};

pub struct OptimizedProgram(Vec<OptimizedCommand>);

impl Deref for OptimizedProgram {
    type Target = Vec<OptimizedCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl OptimizedProgram {
    /// Returns a `HashMap` which maps the index of a left or right bracket to the index of its matching bracket.
    ///
    /// # Errors
    ///
    /// If a bracket is unmatched, returns an error containing its index.
    pub(super) fn find_matching_brackets(&self) -> Result<HashMap<usize, usize>, usize> {
        let mut matching_brackets = HashMap::new();
        let mut left_brackets_stack = Vec::new();
        for (i, cmd) in self.iter().enumerate() {
            match cmd {
                OptimizedCommand::JumpForward => left_brackets_stack.push(i),
                OptimizedCommand::JumpBackwards => match left_brackets_stack.pop() {
                    Some(j) => {
                        matching_brackets.insert(i, j);
                        matching_brackets.insert(j, i);
                    }
                    None => return Err(i),
                },
                _ => (),
            };
        }
        left_brackets_stack.pop().map_or(Ok(matching_brackets), Err)
    }
}

impl From<&[Command]> for OptimizedProgram {
    fn from(program: &[Command]) -> Self {
        let mut optimized_program = Vec::new();

        let mut iter = program.iter().peekable();
        while let Some(command) = iter.next() {
            match (optimized_program.last_mut(), command, iter.peek()) {
                (
                    Some(OptimizedCommand::JumpForward),
                    Command::Decrement,
                    Some(Command::JumpBackwards),
                ) => {
                    optimized_program.pop();
                    optimized_program.push(OptimizedCommand::SetToZero);
                    iter.next();
                }
                (Some(OptimizedCommand::Move(offset)), Command::MoveRight, _) => {
                    *offset += 1;
                }
                (Some(OptimizedCommand::Move(offset)), Command::MoveLeft, _) => {
                    *offset -= 1;
                }
                (Some(OptimizedCommand::Add(value)), Command::Increment, _) => {
                    *value = value.wrapping_add(1);
                }
                (Some(OptimizedCommand::Add(value)), Command::Decrement, _) => {
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
