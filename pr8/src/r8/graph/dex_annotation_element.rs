use crate::r8::graph::dex_string::DexString;
use crate::r8::graph::dex_value::DexValue;

#[derive(Debug, Clone)]
pub struct DexAnnotationElement {
    name: DexString,
    value: DexValue
}

impl DexAnnotationElement {
    pub fn new(name: DexString, value: DexValue) -> DexAnnotationElement {
        DexAnnotationElement {
            name,
            value
        }
    }
}
