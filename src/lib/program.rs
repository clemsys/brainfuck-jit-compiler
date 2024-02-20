use super::command::Command;
use std::{collections::HashMap, ops::Deref};

pub struct Program(Vec<Command>);

impl Deref for Program {
    type Target = Vec<Command>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Program {
    pub(super) fn find_matching_brackets(&self) -> Result<HashMap<usize, usize>, usize> {
        let mut matching_brackets = HashMap::new();
        let mut brackets_stack = Vec::new();
        for (i, cmd) in self.iter().enumerate() {
            match cmd {
                Command::LoopStart => brackets_stack.push(i),
                Command::LoopEnd => match brackets_stack.pop() {
                    Some(j) => {
                        matching_brackets.insert(i, j);
                        matching_brackets.insert(j, i);
                    }
                    None => return Err(i),
                },
                _ => (),
            };
        }
        match brackets_stack.pop() {
            Some(j) => Err(j),
            None => Ok(matching_brackets),
        }
    }
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Self(
            input
                .chars()
                .filter_map(|c| Command::try_from(c).ok())
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
