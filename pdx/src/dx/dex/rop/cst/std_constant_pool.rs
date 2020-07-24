use crate::dx::dex::rop::cst::constant::Constant;

pub struct StdConstantPool {
    entries: Vec<Constant>
}

impl StdConstantPool {
    pub fn new() -> StdConstantPool {
        StdConstantPool {
            entries: vec![]
        }
    }
}

