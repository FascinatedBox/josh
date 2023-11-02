use ast::Expr;
use eval_expr::eval_expr_for_value;
use std::collections::HashMap;
use vm::Vm;

pub struct EmitStorage {
    pub expr_id: u16,
    pub reg: u16,
}

pub struct Emitter {
    pub vm: Vm,
    pub locals: HashMap<String, u16>,
    pub storages: Vec<EmitStorage>,
    pub next_sym_id: u16,
    pub expr_id: u16,
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter {
            expr_id: 1,
            locals: HashMap::new(),
            storages: Vec::new(),
            next_sym_id: 1,
            vm: Vm::new(),
        }
    }

    pub fn write_1(&mut self, one: u16) {
        self.vm.bytecode.push(one);
    }

    pub fn write_2(&mut self, one: u16, two: u16) {
        self.vm.bytecode.push(one);
        self.vm.bytecode.push(two);
    }

    pub fn write_3(&mut self, one: u16, two: u16, three: u16) {
        self.vm.bytecode.push(one);
        self.vm.bytecode.push(two);
        self.vm.bytecode.push(three);
    }

    pub fn write_4(&mut self, one: u16, two: u16, three: u16, four: u16) {
        self.vm.bytecode.push(one);
        self.vm.bytecode.push(two);
        self.vm.bytecode.push(three);
        self.vm.bytecode.push(four);
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
}
