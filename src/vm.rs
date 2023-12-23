use opcodes::*;
use std::process::Command;
use value::*;

macro_rules! arith_op {
    ($self:ident, $bytecode:ident, $pos:ident, $b:tt) => {
        let left_pos = $self.next_op() as usize;
        let left = $self.regs[left_pos].as_integer();
        let right_pos = $self.next_op() as usize;
        let right = $self.regs[right_pos].as_integer();
        let total_pos = $self.next_op() as usize;
        let total = VmValue::VmInteger(left $b right);

        _ = std::mem::replace(&mut $self.regs[total_pos], total);
    };
}

pub struct VmFrame {
    pub func: FuncValue,
    pub code_pos: u16,
}

pub struct Vm {
    pub regs: Vec<VmValue>,
    pub call_stack: Vec<VmFrame>,
    pub current_frame: VmFrame,
    pub bytecode_pos: u16,
}

impl Vm {
    pub fn new(main_func: FuncValue) -> Vm {
        Vm {
            regs: Vec::new(),
            call_stack: Vec::new(),
            current_frame: VmFrame {
                code_pos: 0,
                func: main_func,
            },
            bytecode_pos: 0,
        }
    }

    fn exec_command(&mut self) {
        let count = self.next_op() - 1;
        let idx = self.next_op() as usize;
        let mut cmd = Command::new(&self.current_frame.func.strings[idx]);
        let mut args: Vec<String> = Vec::new();

        for _i in 0..count {
            let j = self.next_op() as usize;
            let s = &self.current_frame.func.strings[j];
            args.push(s.to_string());
        }

        let child_result = cmd.args(args).spawn();
        match child_result {
            Ok(mut child) => match child.wait() {
                Ok(_) => (),
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    fn next_op(&mut self) -> u16 {
        let result = self.current_frame.func.bytecode[self.bytecode_pos as usize];

        self.bytecode_pos += 1;
        result
    }

    pub fn exec(&mut self) {
        for _i in 0..=10 {
            self.regs.push(VmValue::VmEmpty);
        }

        loop {
            let op = self.next_op();

            match op {
                OP_ASSIGN => {
                    let left_pos = self.next_op() as usize;
                    let right_pos = self.next_op() as usize;
                    let right = self.regs[right_pos].clone();

                    _ = std::mem::replace(&mut self.regs[left_pos], right);
                }
                OP_COMMAND => self.exec_command(),
                OP_RETURN_FROM_VM => break,
                OP_LOAD_INTEGER => {
                    let num_pos = self.next_op();
                    let storage_id = self.next_op() as usize;
                    let num = self.current_frame.func.numbers[num_pos as usize];

                    self.regs[storage_id as usize] = VmValue::VmInteger(num);
                }
                OP_PLUS => {
                    self.exec();
                    arith_op!(self, bytecode, pos, +);
                }
                _ => panic!("Unknown opcode {:?}.", op),
            }
        }
    }
}
