use crate::r8::graph::dex_annotation::DexAnnotation;

#[derive(Debug, Clone)]
pub struct DexAnnotationSet {
    annotations: Vec<DexAnnotation>,
}

impl DexAnnotationSet {
    pub fn new(annotations: Vec<DexAnnotation>) -> DexAnnotationSet {
        DexAnnotationSet { annotations }
    }
}
