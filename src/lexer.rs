use lexer_data::IS_IDENT_TABLE;
use token::{SpannedToken, Token};

pub struct Lexer {
    bytes: Vec<u8>,
    text: String,
    offset: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let mut bytes = text.as_bytes().to_vec();

        if bytes[bytes.len() - 1] != '\n' as u8 {
            // Prevent next from reading too far by making sure there's a
            // newline to act as a fence at the end.
            bytes.push('\n' as u8);
        }

        Lexer {
            bytes: bytes,
            offset: 0,
            text: text,
        }
    }

    pub fn identifier_for(&self, token: &SpannedToken) -> String {
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
        loop {
            if self.offset >= self.bytes.len() {
                return SpannedToken {
                    kind: Token::EndOfFile,
                    start: self.offset as u32,
                    len: 0,
                };
            }

            let mut ch = self.bytes[self.offset];

            while ch == (' ' as u8) {
                self.offset += 1;
                ch = self.bytes[self.offset];
            }

            let start = self.offset;
            let mut tok = Token::Invalid;

            match ch as char {
                'a'..='z' | 'A'..='Z' | '_' => {
                    loop {
                        self.offset += 1;
                        ch = self.bytes[self.offset];

                        if IS_IDENT_TABLE[ch as usize] == 0 {
                            break;
                        }
                    }

                    tok = Token::Identifier;
                }
                '=' => {
                    self.offset += 1;
                    tok = Token::Eq;
                }
                '\n' => {
                    self.offset += 1;
                    tok = Token::Newline;
                }
                _ => (),
            }

            return SpannedToken {
                kind: tok,
                start: start as u32,
                len: (self.offset - start) as u32,
            };
        }
    }
}
