use crate::r8::graph::dex_encoded_annotation::DexEncodedAnnotation;

#[derive(Debug, Clone)]
pub struct DexAnnotation {
    visibility: i32,
    annotation: DexEncodedAnnotation
}

impl DexAnnotation {
    pub fn new(visibility: i32, annotation: DexEncodedAnnotation) -> DexAnnotation {
        DexAnnotation {
            visibility,
            annotation
        }
    }
}
