use crate::r8::graph::dex_type::DexType;

#[derive(Debug, Clone)]
pub struct DexTypeList {
    values: Vec<DexType>,
}

impl DexTypeList {
    pub fn new(values: Vec<DexType>) -> DexTypeList {
        DexTypeList { values }
    }
}
