use crate::token::{SpannedToken, Token};

pub struct Scanner {
    bytes: Vec<u8>,
    text: String,
    offset: usize,
}

impl Scanner {
    pub fn new(text: String) -> Self {
        let mut bytes = text.as_bytes().to_vec();

        /* Prevent reading too far if no ending newline. */
        if bytes[bytes.len() - 1] != b'\n' {
            bytes.push(b'\n');
        }

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

        while ch == ' ' || ch == '\t' {
            self.offset += 1;
            ch = self.bytes[self.offset] as char;
        }

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
            '#' => {
                loop {
                    self.offset += 1;
                    ch = self.bytes[self.offset] as char;

                    if ch == '\n' {
                        break;
                    }
                }

                if self.offset != self.bytes.len() - 1 {
                    self.offset += 1;
                    tok = Token::Newline;
                } else {
                    tok = Token::EndOfFile;
                }
            }
            '\n' => {
                if self.offset != self.bytes.len() - 1 {
                    self.offset += 1;
                    tok = Token::Newline;
                } else {
                    tok = Token::EndOfFile;
                }
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
