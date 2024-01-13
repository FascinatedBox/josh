mod ast;
mod compiler;
mod eval_expr;
mod eval_stmt;
mod interpreter;
mod opcode;
mod parser;
mod scanner;
mod token;
mod value;
mod vm;
use interpreter::Interpreter;
use std::env;

fn main() {
    let interp = Interpreter::new();
    let args: Vec<_> = env::args().collect();
    let _ = interp.parse_file(&args[1]);
}
