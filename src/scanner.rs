use crate::token::{SpannedToken, Token};

pub struct Scanner {
    bytes: Vec<u8>,
    text: String,
    offset: usize,
}

impl Scanner {
    pub fn new(text: String) -> Self {
        let bytes = text.as_bytes().to_vec();

        Self {
            bytes: bytes,
            offset: 0,
            text: text,
        }
    }

    pub fn text_for(&self, token: &SpannedToken) -> String {
        match token.kind {
            Token::Identifier => {
                let start = token.start as usize;
                let end = (token.start + token.len) as usize;

                String::from(&self.text[start..end])
            }
            _ => String::from(""),
        }
    }

    pub fn next(&mut self) -> SpannedToken {
        let mut ch = self.bytes[self.offset] as char;
        let start = self.offset;
        let tok;

        match ch {
            'a'..='z' | 'A'..='Z' | '_' => {
                loop {
                    self.offset += 1;
                    ch = self.bytes[self.offset] as char;

                    match ch {
                        'a'..='z' | 'A'..='Z' | '_' => continue,
                        _ => break,
                    }
                }

                tok = Token::Identifier;
            }
            '\n' => {
                self.offset += 1;
                tok = Token::Newline;
            }
            _ => {
                panic!("Scanner.next case {} not handled.", ch as u8);
            }
        }

        SpannedToken {
            kind: tok,
            start: start as u32,
            len: (self.offset - start) as u32,
        }
    }
}
