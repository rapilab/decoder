use crate::r8::graph::dex_string::DexString;
use crate::r8::graph::dex_type::DexType;
use crate::r8::graph::dex_proto::DexProto;

#[derive(Debug, Clone)]
pub struct DexMethod {
    holder: DexType,
    proto: DexProto,
    name: DexString,
}

impl DexMethod {
    pub fn new(holder: DexType, proto: DexProto, name: DexString) -> DexMethod {
        DexMethod {
            holder,
            proto,
            name
        }
    }
}