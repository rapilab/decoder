use crate::r8::graph::dex_string::DexString;

#[derive(Debug, Clone)]
pub struct DexType {
    descriptor: DexString
}

impl DexType {
    pub fn new(descriptor: DexString) -> DexType {
        DexType {
            descriptor
        }
    }
}