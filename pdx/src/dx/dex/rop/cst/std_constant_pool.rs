use crate::dx::dex::rop::cst::constant::Constant;

#[derive(Debug, Clone)]
pub struct StdConstantPool {
    entries: Vec<Constant>,
}

impl StdConstantPool {
    pub fn new() -> StdConstantPool {
        StdConstantPool { entries: vec![] }
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
