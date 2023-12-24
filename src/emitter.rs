use ast::Expr;
use eval_expr::eval_expr_for_value;
use opcodes::*;
use std::collections::HashMap;
use value::FuncValue;

pub struct EmitStorage {
    pub expr_id: u16,
    pub reg: u16,
}

pub struct FuncBlock {
    pub code_start: u16,
    pub number_count: u16,
    pub string_count: u16,
}

pub struct Emitter {
    pub bytecode: Vec<u16>,
    pub saved_numbers: Vec<i64>,
    pub saved_strings: Vec<String>,

    pub func_blocks: Vec<FuncBlock>,
    pub numbers: Vec<i64>,
    pub locals: HashMap<String, u16>,
    pub storages: Vec<EmitStorage>,
    pub next_sym_id: u16,
    pub expr_id: u16,
    pub func_block: FuncBlock,
}

#[macro_export]
macro_rules! emit_n {
    ($emitter:ident, $( $x:expr ),*) => {
        $(
            $emitter.bytecode.push($x);
        )*
    };
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter {
            bytecode: Vec::new(),
            expr_id: 1,
            saved_numbers: Vec::new(),
            saved_strings: Vec::new(),
            numbers: Vec::new(),
            func_blocks: Vec::new(),
            locals: HashMap::new(),
            storages: Vec::new(),
            next_sym_id: 1,
            func_block: FuncBlock {
                code_start: 0,
                number_count: 0,
                string_count: 0,
            },
        }
    }

    fn next_id(&mut self) -> u16 {
        let result = self.next_sym_id;

        self.next_sym_id += 1;
        result
    }

    pub fn declare_local(&mut self, name: String) -> u16 {
        let l = self.locals.get(&name);

        match l {
            Some(_) => panic!("Error: Local named {:?} already exists.", name),
            None => {
                let id = self.next_id();
                self.locals.insert(name, id);
                id
            }
        }
    }

    pub fn get_storage_id(&mut self) -> u16 {
        for s in &self.storages {
            if s.expr_id != self.expr_id {
                return s.reg;
            }
        }

        let id = self.next_id();

        self.next_sym_id += 1;
        self.storages.push(EmitStorage {
            reg: id,
            expr_id: self.expr_id,
        });

        (self.storages.len() - 1) as u16
    }

    pub fn write_number(&mut self, n: &i64) -> u16 {
        let result = self.func_block.number_count;

        self.numbers.push(*n);
        self.func_block.number_count += 1;

        result
    }

    pub fn write_string(&mut self, str: &String) -> u16 {
        let result = self.func_block.string_count;

        self.saved_strings.push(str.into());
        self.func_block.string_count += 1;
        result
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> u16 {
        self.expr_id += 1;
        let sym = eval_expr_for_value(self, expr);

        sym.reg
    }

    pub fn prepare_main(&mut self) -> FuncValue {
        emit_n!(self, OP_RETURN_FROM_VM);

        FuncValue {
            bytecode: self.bytecode.split_off(0),
            name: "main".to_string(),
            numbers: self.numbers.split_off(0),
            strings: self.saved_strings.split_off(0),
        }
    }
}
