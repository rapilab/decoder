use crate::r8::graph::dex_method::DexMethod;
use crate::r8::graph::method_access_flags::MethodAccessFlags;
use crate::r8::graph::dex_annotation_set::DexAnnotationSet;
use crate::r8::graph::parameter_annotations_list::ParameterAnnotationsList;
use crate::r8::graph::code::Code;

#[derive(Debug, Clone)]
pub struct DexEncodedMethod {
    method: DexMethod,
    access_flags: MethodAccessFlags,
    annotations: DexAnnotationSet,
    parameter_annotations_list: ParameterAnnotationsList,
    code: Box<dyn Code>
}

impl DexEncodedMethod {
    pub fn new() {

    }
}
