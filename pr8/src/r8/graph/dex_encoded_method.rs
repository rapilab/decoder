use crate::r8::graph::code::Code;
use crate::r8::graph::dex_annotation_set::DexAnnotationSet;
use crate::r8::graph::dex_method::DexMethod;
use crate::r8::graph::method_access_flags::MethodAccessFlags;
use crate::r8::graph::parameter_annotations_list::ParameterAnnotationsList;

#[derive(Debug, Clone)]
pub struct DexEncodedMethod {
    method: DexMethod,
    access_flags: MethodAccessFlags,
    annotations: DexAnnotationSet,
    parameter_annotations_list: ParameterAnnotationsList,
    pub(crate) code: Box<dyn Code>,
}

impl DexEncodedMethod {
    pub fn new() {}

    pub fn is_class_initializer(&self) -> bool {
        // self.access_flags.is_constructor();
        return false;
    }

    pub fn get_code(&self) -> Box<dyn Code> {
        return self.code.clone()
    }
}
