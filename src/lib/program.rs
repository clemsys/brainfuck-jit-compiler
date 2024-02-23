use super::command::Command;
use std::ops::Deref;

pub struct Program(Vec<Command>);

impl Deref for Program {
    type Target = Vec<Command>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
