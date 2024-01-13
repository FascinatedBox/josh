use crate::compiler::Compiler;
use crate::eval_stmt::eval_stmt;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::vm::Vm;
use std::fs::File;
use std::io::Read;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_file(self, path: &String) -> std::io::Result<()> {
        let mut file = match File::open(&path) {
            Err(reason) => panic!("Unable to open {}: {}", path, reason),
            Ok(file) => file,
        };
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let scanner = Scanner::new(contents);
        let mut parser = Parser::new(scanner);
        let actions = parser.parse();
        let mut compiler = Compiler::new();

        for a in actions {
            eval_stmt(&mut compiler, a);
        }

        let main_func = compiler.prepare_main();

        let mut vm = Vm::new();

        vm.exec(main_func);

        Ok(())
    }
}
