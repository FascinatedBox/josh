use ast::{Expr, Stmt};
use lexer::Lexer;
use parse_expr::parse_expr;
use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use token::{SpannedToken, Token};

pub struct Parser {
    files: Vec<Lexer>,
}

pub struct ParseState<'a> {
    pub lexer: &'a mut Lexer,
    token: SpannedToken,
}

impl<'a> ParseState<'a> {
    pub fn current_ident(&self) -> String {
        self.lexer.identifier_for(&self.token)
    }

    pub fn parse_ident_exec(&mut self) -> Stmt {
        let mut words: Vec<Expr> = Vec::new();
        let ident = self.current_ident();

        words.push(Expr::String(ident));

        loop {
            self.token = self.lexer.next();

            match self.token.kind {
                Token::EndOfFile | Token::Newline => break,
                Token::Identifier => {
                    let ident = self.current_ident();

                    words.push(Expr::String(ident));
                }
                _ => (),
            }
        }

        Stmt::Expr(Expr::Command(words))
    }

    pub fn expect_next_ident(&mut self) -> String {
        self.token = self.lexer.next();

        match self.token.kind {
            Token::Identifier => self.current_ident(),
            _ => panic!("Expected identifier after 'var' keyword."),
        }
    }

    pub fn parse_var(&mut self) -> Stmt {
        let name = self.expect_next_ident();

        self.lexer.next();

        let expr = parse_expr(self);

        Stmt::Var(name, expr)
    }

    pub fn parse_identifier(&mut self) -> Stmt {
        let ident = self.current_ident();

        if ident.len() > 8 {
            return self.parse_ident_exec();
        }

        match ident.as_ref() {
            "var" => self.parse_var(),
            _ => self.parse_ident_exec(),
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        self.token = self.lexer.next();

        let mut statements: Vec<Stmt> = Vec::new();

        loop {
            match &self.token.kind {
                Token::EndOfFile => break,
                Token::Newline => {
                    self.token = self.lexer.next();
                }
                Token::Identifier => {
                    statements.push(self.parse_identifier());
                    continue;
                }
                _ => (),
            }
        }

        statements
    }
}

impl Parser {
    pub fn new() -> Parser {
        Self { files: Vec::new() }
    }

    pub fn load_file(&mut self, path: &str) {
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path, why),
            Ok(file) => file,
        };

        let mut s = String::new();

        if let Err(why) = file.read_to_string(&mut s) {
            panic!("couldn't read {}: {}", path, why);
        }

        let lexer = Lexer::new(s);

        self.files.push(lexer);
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut lexer = self.files.pop().unwrap();
        let mut state = ParseState {
            lexer: &mut lexer,
            token: SpannedToken {
                kind: Token::Invalid,
                start: 0,
                len: 0,
            },
        };

        state.parse()
    }
}
