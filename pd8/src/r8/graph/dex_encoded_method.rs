use crate::r8::graph::dex_method::DexMethod;

#[derive(Debug, Clone)]
pub struct DexEncodedMethod {
    method: DexMethod,
    // access_flags: MethodAccessFlags,
    // annotations: DexAnnotationSet,
    // parameter_annotations_list: ParameterAnnotationsList,
    // code: Code
}

impl DexEncodedMethod {
    pub fn new() {

    }
}
