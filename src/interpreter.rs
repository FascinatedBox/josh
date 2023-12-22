use emitter::Emitter;
use eval_stmt::eval_stmt;
use parser::Parser;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn parse_args(self, args: Vec<String>) {
        if args.len() != 2 {
            eprintln!("Error: Usage: josh <path-to-josh-file>");
            std::process::exit(1)
        }

        let path = &args[1];
        let mut parser = Parser::new();
        parser.load_file(&path);
        let statements = parser.parse();
        let mut emitter = Emitter::new();

        for s in statements {
            eval_stmt(&mut emitter, s);
        }

        let main_sym = emitter.finish_main();

        emitter.vm.load_func(main_sym);
        emitter.vm.exec();
    }
}
