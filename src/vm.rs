use opcodes::*;
use std::process::Command;
use std::ptr;
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
    pub code: *const u16,
    pub numbers: *const i64,
    pub strings: *const String,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            regs: Vec::new(),
            call_stack: Vec::new(),
            code: ptr::null(),
            strings: ptr::null(),
            numbers: ptr::null(),
        }
    }

    #[inline]
    fn next_op(&mut self) -> u16 {
        unsafe {
            let result = *self.code;
            self.code = self.code.add(1);
            result
        }
    }

    #[inline]
    fn number_at(&self, index: u16) -> i64 {
        unsafe { *self.numbers.offset(index as isize) }
    }

    #[inline]
    fn string_at(&self, index: u16) -> String {
        unsafe { (*self.strings.offset(index as isize)).clone() }
    }

    fn exec_command(&mut self) {
        let count = self.next_op() - 1;
        let idx = self.next_op();
        let mut cmd = Command::new(self.string_at(idx));
        let mut args: Vec<String> = Vec::new();

        for _i in 0..count {
            let j = self.next_op();
            let s = self.string_at(j);
            args.push(s);
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

    pub fn load_function(&mut self, f: FuncValue) {
        self.call_stack.push(VmFrame {
            func: f,
            code_pos: 0,
        })
    }

    pub fn exec(&mut self) {
        for _i in 0..=10 {
            self.regs.push(VmValue::VmEmpty);
        }

        let func = &self.call_stack.last().unwrap().func;

        self.code = func.bytecode.as_ptr();
        self.strings = func.strings.as_ptr();
        self.numbers = func.numbers.as_ptr();

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
                    let num = self.number_at(num_pos);

                    self.regs[storage_id as usize] = VmValue::VmInteger(num);
                }
                OP_LOAD_STRING => {
                    let num_pos = self.next_op();
                    let storage_id = self.next_op() as usize;
                    let num = self.string_at(num_pos);

                    self.regs[storage_id as usize] = VmValue::VmString(num);
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
