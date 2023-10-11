use lexer::Lexer;
use std::fs::File;
use std::io::Read;
use std::vec::Vec;

pub struct Parser {
    files: Vec<Lexer>,
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

    pub fn parse(&mut self) {
        // todo: Parse josh code.
    }
}
