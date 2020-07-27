use crate::r8::graph::dex_string::DexString;
use crate::r8::graph::dex_type::DexType;
use crate::r8::graph::dex_type_list::DexTypeList;

#[derive(Debug, Clone)]
pub struct DexProto {
    shorty: DexString,
    return_type: DexType,
    parameters: DexTypeList
}

impl DexProto {
    pub fn new(shorty: DexString, return_type: DexType, parameters: DexTypeList) -> DexProto {
        DexProto {
            shorty,
            return_type,
            parameters
        }
    }
}