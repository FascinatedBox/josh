mod lexer;
mod parser;
use parser::Parser;
use std::env;

fn main() {
    let mut parser = Parser::new();
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: Usage: josh <path-to-josh-file>");
        std::process::exit(1)
    }
    let path = &args[1];
    parser.load_file(&path);
    parser.parse();
}
