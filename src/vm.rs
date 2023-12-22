use opcodes::*;
use std::process::Command;
use value::*;

macro_rules! arith_op {
    ($self:ident, $bytecode:ident, $pos:ident, $b:tt) => {
        let left_pos = $bytecode[$pos + 1] as usize;
        let left = $self.values[left_pos].as_integer();
        let right_pos = $bytecode[$pos + 2] as usize;
        let right = $self.values[right_pos].as_integer();
        let total_pos = $bytecode[$pos + 3] as usize;
        let total = VmValue::VmInteger(left $b right);

        _ = std::mem::replace(&mut $self.values[total_pos], total);
        $pos += 4;
    };
}

pub struct VmFrame {
    pub func: FuncValue,
    pub code_pos: u16,
}

pub struct Vm {
    pub strings: Vec<String>,
    pub numbers: Vec<i64>,
    pub values: Vec<VmValue>,
    pub call_stack: Vec<VmFrame>,
    pub call_stack_pos: u16,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            numbers: Vec::new(),
            strings: Vec::new(),
            values: Vec::new(),
            call_stack: Vec::new(),
            call_stack_pos: 0,
        }
    }

    fn exec_command(&self, frame: &VmFrame, pos: &mut usize) {
        let bytecode = &frame.func.bytecode;

        *pos += 1;
        let count = bytecode[*pos] - 1;

        *pos += 1;
        let program = &self.strings[bytecode[*pos] as usize];
        *pos += 1;
        let mut args: Vec<&String> = Vec::new();

        for _i in 0..count {
            args.push(&self.strings[bytecode[*pos] as usize]);
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

    pub fn load_func(&mut self, func: FuncValue) {
        self.call_stack.push(VmFrame {
            func: func,
            code_pos: 0,
        })
    }

    pub fn exec(&mut self) {
        let mut pos = 0 as usize;

        for _i in 0..=10 {
            self.values.push(VmValue::VmEmpty);
        }

        let current_frame = &self.call_stack[0];
        let bytecode = &current_frame.func.bytecode;

        loop {
            let op = bytecode[pos];

            match op {
                OP_ASSIGN => {
                    let left_pos = bytecode[pos + 1] as usize;
                    let right_pos = bytecode[pos + 2] as usize;
                    let right = self.values[right_pos].clone();

                    _ = std::mem::replace(&mut self.values[left_pos], right);
                    pos += 3;
                }
                OP_COMMAND => self.exec_command(current_frame, &mut pos),
                OP_RETURN_FROM_VM => break,
                OP_LOAD_INTEGER => {
                    let num_pos = bytecode[pos + 1];
                    let storage_id = bytecode[pos + 2] as usize;
                    let num = self.numbers[num_pos as usize];

                    self.values[storage_id as usize] = VmValue::VmInteger(num);
                    pos += 3;
                }
                OP_PLUS => {
                    arith_op!(self, bytecode, pos, +);
                }
                _ => panic!("Unknown opcode {:?}.", op),
            }
        }
    }
}
