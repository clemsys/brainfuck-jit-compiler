#[derive(PartialEq)]
pub enum Command {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    LoopStart,
    LoopEnd,
}

impl TryFrom<char> for Command {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Self::MoveRight),
            '<' => Ok(Self::MoveLeft),
            '+' => Ok(Self::Increment),
            '-' => Ok(Self::Decrement),
            '.' => Ok(Self::Print),
            ',' => Ok(Self::Read),
            '[' => Ok(Self::LoopStart),
            ']' => Ok(Self::LoopEnd),
            _ => Err(c),
        }
    }
}
