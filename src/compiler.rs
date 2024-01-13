use crate::opcode::O_VM_EXIT;
use crate::value::FuncValue;

pub struct Compiler {
    pub code: Vec<u16>,
    saved_strings: Vec<String>,
}

#[macro_export]
macro_rules! emit_n {
    ($compiler:ident, $($op:expr),*) => {
        $(
            $compiler.code.push($op);
        )*
    }
}

pub use emit_n;

impl Compiler {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            saved_strings: Vec::new(),
        }
    }

    pub fn intern_string(&mut self, s: &String) -> u16 {
        let result = self.saved_strings.len();

        self.saved_strings.push(s.to_string());
        result as u16
    }

    pub fn prepare_main(&mut self) -> FuncValue {
        emit_n!(self, O_VM_EXIT);
        FuncValue {
            code: self.code.split_off(0),
            strings: self.saved_strings.split_off(0),
        }
    }
}
