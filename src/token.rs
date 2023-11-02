pub struct SpannedToken {
    pub kind: Token,
    pub start: u32,
    pub len: u32,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Divide,
    EndOfFile,
    Eq,
    Identifier,
    Invalid,
    Minus,
    Multiply,
    Newline,
    Number(i64),
    Plus,
}
