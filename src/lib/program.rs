use std::ops::Deref;

use super::command::BfCommand;

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
