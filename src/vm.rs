use opcodes::*;
use std::process::Command;

pub struct Vm {
    pub strings: Vec<String>,
    pub bytecode: Vec<u16>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            bytecode: Vec::new(),
            strings: Vec::new(),
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

        loop {
            let op = self.bytecode[pos];

            match op {
                OP_COMMAND => self.exec_command(&mut pos),
                OP_RETURN_FROM_VM => break,
                _ => panic!("Unknown opcode {:?}.", op),
            }
        }
    }
}
