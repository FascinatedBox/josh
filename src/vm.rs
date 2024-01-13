use crate::opcode::*;
use crate::value::FuncValue;
use std::process::Command;
use std::ptr;

pub struct Vm {
    code: *const u16,
    strings: *const String,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            code: ptr::null(),
            strings: ptr::null(),
        }
    }

    fn unget_op(&mut self) {
        unsafe {
            self.code = self.code.sub(1);
        }
    }

    fn next_op(&mut self) -> u16 {
        unsafe {
            let result = *self.code;
            self.code = self.code.add(1);
            result
        }
    }

    fn next_op_as_string(&mut self) -> String {
        let pos = self.next_op();

        self.string_at(pos)
    }

    fn string_at(&self, index: u16) -> String {
        unsafe { (*self.strings.offset(index as isize)).clone() }
    }

    fn cmd_exec(&mut self, name: &String, args: &Vec<String>) {
        let child = Command::new(name).args(args).spawn();

        match child {
            Ok(mut child) => match child.wait() {
                Ok(_) => (),
                Err(_) => (),
            },
            Err(e) => eprintln!("vm exec failed: {}", e),
        }
    }

    fn exec_command(&mut self) {
        let mut name: String = String::new();
        let mut args: Vec<String> = Vec::new();

        loop {
            let op = self.next_op();

            match op {
                C_LOAD_STRING => {
                    let s = self.next_op_as_string();

                    if name.is_empty() {
                        name = s;
                    } else {
                        args.push(s);
                    }
                }
                C_EXEC => {
                    self.cmd_exec(&name, &args);
                }
                _ => {
                    self.unget_op();
                    return;
                }
            }
        }
    }

    pub fn exec(&mut self, func: FuncValue) {
        self.strings = func.strings.as_ptr();
        self.code = func.code.as_ptr();

        loop {
            let op = self.next_op();

            match op {
                O_COMMAND => {
                    self.exec_command();
                }
                O_VM_EXIT => {
                    return;
                }
                _ => panic!("Invalid vm opcode {}", op),
            }
        }
    }
}
