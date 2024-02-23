use super::command::Command;

#[derive(PartialEq, Eq)]
pub enum OptimizedCommand {
    Move(i32),
    Add(u8),
    SetToZero,
    Print,
    Read,
    JumpForward,
    JumpBackwards,
}

impl From<&Command> for OptimizedCommand {
    fn from(command: &Command) -> Self {
        match command {
            Command::MoveRight => Self::Move(1),
            Command::MoveLeft => Self::Move(-1),
            Command::Increment => Self::Add(1),
            Command::Decrement => Self::Add(u8::MAX), // decr is equivalent to add 255 modulo 256
            Command::Print => Self::Print,
            Command::Read => Self::Read,
            Command::JumpForward => Self::JumpForward,
            Command::JumpBackwards => Self::JumpBackwards,
        }
    }
}
