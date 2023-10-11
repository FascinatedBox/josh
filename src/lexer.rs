pub struct Lexer {
    bytes: Vec<u8>,
    text: String,
    offset: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer {
            bytes: text.as_bytes().to_vec(),
            offset: 0,
            text: text,
        }
    }
}
