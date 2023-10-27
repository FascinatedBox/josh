use vm::Vm;

pub struct Emitter {
    pub vm: Vm,
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter { vm: Vm::new() }
    }

    pub fn write_1(&mut self, one: u16) {
        self.vm.bytecode.push(one);
    }

    pub fn write_2(&mut self, one: u16, two: u16) {
        self.vm.bytecode.push(one);
        self.vm.bytecode.push(two);
    }

    pub fn write_string(&mut self, str: &String) -> u16 {
        self.vm.strings.push(str.into());
        (self.vm.strings.len() - 1) as u16
    }
}
