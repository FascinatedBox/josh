pub struct FuncValue {
    pub bytecode: Vec<u16>,
    pub name: String,
    pub numbers: Vec<i64>,
    pub strings: Vec<String>,
}

#[derive(Clone)]
pub enum VmValue {
    VmInteger(i64),
    VmEmpty,
}

impl VmValue {
    pub fn as_integer(&self) -> i64 {
        match self {
            VmValue::VmInteger(i) => *i,
            _ => panic!("as_integer on non-integer value"),
        }
    }
}
