use ast::Expr;
use eval_expr::eval_expr_for_value;
use opcodes::*;
use std::collections::HashMap;
use value::FuncValue;
use vm::Vm;

pub struct EmitStorage {
    pub expr_id: u16,
    pub reg: u16,
}

pub struct FuncBlock {
    pub code_start: u16,
}

pub struct Emitter {
    pub bytecode: Vec<u16>,
    pub func_blocks: Vec<FuncBlock>,
    pub vm: Vm,
    pub locals: HashMap<String, u16>,
    pub storages: Vec<EmitStorage>,
    pub next_sym_id: u16,
    pub expr_id: u16,
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter {
            bytecode: Vec::new(),
            expr_id: 1,
            func_blocks: vec![FuncBlock { code_start: 0 }],
            locals: HashMap::new(),
            storages: Vec::new(),
            next_sym_id: 1,
            vm: Vm::new(),
        }
    }

    pub fn write_1(&mut self, one: u16) {
        self.bytecode.push(one);
    }

    pub fn write_2(&mut self, one: u16, two: u16) {
        self.bytecode.push(one);
        self.bytecode.push(two);
    }

    pub fn write_3(&mut self, one: u16, two: u16, three: u16) {
        self.bytecode.push(one);
        self.bytecode.push(two);
        self.bytecode.push(three);
    }

    pub fn write_4(&mut self, one: u16, two: u16, three: u16, four: u16) {
        self.bytecode.push(one);
        self.bytecode.push(two);
        self.bytecode.push(three);
        self.bytecode.push(four);
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
        self.vm.numbers.push(*n);
        (self.vm.numbers.len() - 1) as u16
    }

    pub fn write_string(&mut self, str: &String) -> u16 {
        self.vm.strings.push(str.into());
        (self.vm.strings.len() - 1) as u16
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> u16 {
        self.expr_id += 1;
        let sym = eval_expr_for_value(self, expr);

        sym.reg
    }

    pub fn finish_main(&mut self) -> FuncValue {
        self.write_1(OP_RETURN_FROM_VM);

        FuncValue {
            bytecode: self.bytecode.split_off(0),
            name: "main".to_string(),
        }
    }
}
