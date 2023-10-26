mod ast;
mod emitter;
mod eval_expr;
mod eval_stmt;
mod interpreter;
mod lexer;
mod lexer_data;
mod opcodes;
mod parser;
mod token;
mod vm;
use interpreter::Interpreter;
use std::env;

fn main() {
    let interp = Interpreter::new();
    let args: Vec<_> = env::args().collect();

    interp.parse_args(args);
}
