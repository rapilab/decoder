use crate::r8::graph::dex_encoded_method::DexEncodedMethod;
use crate::r8::graph::dex_program_class::DexProgramClass;

#[derive(Debug, Clone)]
pub struct MethodCollection {
    holder: Box<DexProgramClass>,
    direct_methods: Vec<DexEncodedMethod>,
    virtual_methods: Vec<DexEncodedMethod>
}

impl MethodCollection {
    pub fn new(holder: Box<DexProgramClass>, direct_methods: Vec<DexEncodedMethod>, virtual_methods: Vec<DexEncodedMethod>) -> MethodCollection {
        MethodCollection {
            holder,
            direct_methods,
            virtual_methods
        }
    }

    pub fn get_class_initializer(&self) -> Option<DexEncodedMethod> {
        for method in self.direct_methods {
            if method.is_class_initializer() {
                return Some(method)
            }
        }

        None
    }
}
