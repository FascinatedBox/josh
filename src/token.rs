pub struct SpannedToken {
    pub kind: Token,
    pub start: u32,
    pub len: u32,
}

#[derive(Debug)]
pub enum Token {
    Identifier,
    Invalid,
    EndOfFile,
    Newline,
}
