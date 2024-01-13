pub struct SpannedToken {
    pub kind: Token,
    pub start: u32,
    pub len: u32,
}

pub enum Token {
    Identifier,
    EndOfFile,
    Newline,
}
