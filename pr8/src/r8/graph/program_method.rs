use crate::r8::graph::dex_encoded_method::DexEncodedMethod;
use crate::r8::graph::dex_program_class::DexProgramClass;

pub struct ProgramMethod {
    holder: DexProgramClass,
    method: DexEncodedMethod,
}

impl ProgramMethod {
    pub fn new(holder: DexProgramClass, method: DexEncodedMethod) -> ProgramMethod {
        ProgramMethod { holder, method }
    }

    pub fn get_definition(&self) -> DexEncodedMethod {
        return self.method.clone()
    }
}
