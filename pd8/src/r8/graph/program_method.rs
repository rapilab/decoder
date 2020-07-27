use crate::r8::graph::dex_program_class::DexProgramClass;
use crate::r8::graph::dex_encoded_method::DexEncodedMethod;

pub struct ProgramMethod {
    holder: DexProgramClass,
    method: DexEncodedMethod,
}

impl ProgramMethod {
    pub fn new(holder: DexProgramClass, method: DexEncodedMethod) -> ProgramMethod {
        ProgramMethod {
            holder,
            method
        }
    }
}