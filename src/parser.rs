use crate::ast::{Expr, Stmt};
use crate::scanner::Scanner;
use crate::token::{SpannedToken, Token};
use std::collections::HashMap;

pub struct Parser {
    keywords: HashMap<&'static str, i32>,
    scanner: Scanner,
}

impl Parser {
    pub fn new(s: Scanner) -> Self {
        let mut keywords = HashMap::from([("var", 1)]);

        keywords.insert("var", 1);

        Self {
            keywords: keywords,
            scanner: s,
        }
    }

    pub fn dispatch_keyword(&mut self, _id: i32) -> Stmt {
        todo!("Keyword dispatch.");
    }

    pub fn keyword_id_for(&self, s: &String) -> i32 {
        match self.keywords.get(s as &str) {
            Some(s) => *s,
            None => -1,
        }
    }

    pub fn parse_command(&mut self, first_word: &String) -> Stmt {
        let mut words: Vec<Expr> = Vec::new();

        words.push(Expr::String(first_word.into()));

        // todo: the other words

        Stmt::Expr(Expr::Command(words))
    }

    pub fn parse_identifier(&mut self, tok: SpannedToken) -> Stmt {
        let text = self.scanner.text_for(&tok);
        let keyword_id = self.keyword_id_for(&text);

        if keyword_id != -1 {
            self.dispatch_keyword(keyword_id);
            return Stmt::Todo;
        }

        self.parse_command(&text)
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut result: Vec<Stmt> = Vec::new();

        loop {
            let token = self.scanner.next();

            match token.kind {
                Token::Identifier => {
                    result.push(self.parse_identifier(token));
                }
                Token::Newline => break,
                Token::EndOfFile => break,
            }
        }

        result
    }
}
