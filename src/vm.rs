use opcodes::*;
use std::process::Command;

macro_rules! arith_op {
    ($self:ident, $pos:ident, $b:tt) => {
        let left_pos = $self.bytecode[$pos + 1] as usize;
        let left = $self.values[left_pos].as_integer();
        let right_pos = $self.bytecode[$pos + 2] as usize;
        let right = $self.values[right_pos].as_integer();
        let total_pos = $self.bytecode[$pos + 3] as usize;
        let total = VmValue::VmInteger(left $b right);

        _ = std::mem::replace(&mut $self.values[total_pos], total);
        $pos += 4;
    };
}

#[derive(Clone)]
pub enum VmValue {
    VmInteger(i64),
    VmEmpty,
}

impl VmValue {
    fn as_integer(&self) -> i64 {
        match self {
            VmValue::VmInteger(i) => *i,
            _ => panic!("as_integer on non-integer value"),
        }
    }
}

pub struct Vm {
    pub strings: Vec<String>,
    pub numbers: Vec<i64>,
    pub bytecode: Vec<u16>,
    pub values: Vec<VmValue>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            bytecode: Vec::new(),
            numbers: Vec::new(),
            strings: Vec::new(),
            values: Vec::new(),
        }
    }

    fn exec_command(&mut self, pos: &mut usize) {
        *pos += 1;
        let count = self.bytecode[*pos] - 1;

        *pos += 1;
        let program = &self.strings[self.bytecode[*pos] as usize];
        *pos += 1;
        let mut args: Vec<&String> = Vec::new();

        for _i in 0..count {
            args.push(&self.strings[self.bytecode[*pos] as usize]);
        }

        let child_result = Command::new(program).args(args).spawn();
        match child_result {
            Ok(mut child) => match child.wait() {
                Ok(_) => (),
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    pub fn exec(&mut self) {
        let mut pos = 0 as usize;

        for _i in 0..=10 {
            self.values.push(VmValue::VmEmpty);
        }

        loop {
            let op = self.bytecode[pos];

            match op {
                OP_ASSIGN => {
                    let left_pos = self.bytecode[pos + 1] as usize;
                    let right_pos = self.bytecode[pos + 2] as usize;
                    let right = self.values[right_pos].clone();

                    _ = std::mem::replace(&mut self.values[left_pos], right);
                    pos += 3;
                }
                OP_COMMAND => self.exec_command(&mut pos),
                OP_RETURN_FROM_VM => break,
                OP_LOAD_INTEGER => {
                    let num_pos = self.bytecode[pos + 1];
                    let storage_id = self.bytecode[pos + 2] as usize;
                    let num = self.numbers[num_pos as usize];

                    self.values[storage_id as usize] = VmValue::VmInteger(num);
                    pos += 3;
                }
                OP_PLUS => {
                    arith_op!(self, pos, +);
                }
                _ => panic!("Unknown opcode {:?}.", op),
            }
        }
    }
}
