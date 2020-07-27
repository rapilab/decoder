use crate::r8::graph::dex_type::DexType;
use crate::r8::graph::dex_annotation_element::DexAnnotationElement;

#[derive(Debug, Clone)]
pub struct DexEncodedAnnotation {
    dex_type: DexType,
    elements: Vec<DexAnnotationElement>
}

impl DexEncodedAnnotation {
    pub fn new(dex_type: DexType, elements: Vec<DexAnnotationElement>) -> DexEncodedAnnotation {
        DexEncodedAnnotation {
            dex_type,
            elements
        }
    }
}
