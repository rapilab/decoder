use crate::r8::graph::dex_annotation_set::DexAnnotationSet;

#[derive(Debug, Clone)]
pub struct ParameterAnnotationsList {
    values: Vec<DexAnnotationSet>,
    missing_parameter_annotations: i32,
}

impl ParameterAnnotationsList {
    pub fn new(values: Vec<DexAnnotationSet>) -> ParameterAnnotationsList {
        ParameterAnnotationsList {
            values,
            missing_parameter_annotations: 0,
        }
    }
}
