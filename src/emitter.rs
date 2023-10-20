use expr::Expr;
use opcodes::*;
use vm::Vm;

pub struct Emitter {
    pub vm: Vm,
}

struct TypedValue {
    reg: u16,
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter { vm: Vm::new() }
    }

    fn write_1(&mut self, one: u16) {
        self.vm.bytecode.push(one);
    }

    fn write_2(&mut self, one: u16, two: u16) {
        self.vm.bytecode.push(one);
        self.vm.bytecode.push(two);
    }

    fn emit_command(&mut self, trees: &Vec<Expr>) -> TypedValue {
        self.write_2(OP_COMMAND, trees.len() as u16);

        for t in trees {
            let value = self.walk_tree(t);

            self.write_1(value.reg);
        }

        TypedValue { reg: 0 }
    }

    fn emit_string(&mut self, str: &String) -> TypedValue {
        self.vm.strings.push(str.into());

        TypedValue {
            reg: (self.vm.strings.len() - 1) as u16,
        }
    }

    fn walk_tree(&mut self, tree: &Expr) -> TypedValue {
        match tree {
            Expr::Command(ref trees) => self.emit_command(trees),
            Expr::String(ref s) => self.emit_string(s),
        }
    }

    pub fn emit(&mut self, statements: Vec<Expr>) {
        for s in statements {
            self.walk_tree(&s);
        }

        self.write_1(OP_RETURN_FROM_VM);
    }
}
