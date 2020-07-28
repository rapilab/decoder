use crate::r8::graph::dex_encoded_method::DexEncodedMethod;
use crate::r8::graph::dex_program_class::DexProgramClass;
use crate::r8::graph::app_view::AppView;
use crate::r8::ir::code::ir_code::IRCode;

#[derive(Clone, Debug)]
pub struct ProgramMethod {
    pub(crate) holder: DexProgramClass,
    pub(crate) method: DexEncodedMethod,
}

impl ProgramMethod {
    pub fn new(holder: DexProgramClass, method: DexEncodedMethod) -> ProgramMethod {
        ProgramMethod { holder, method }
    }

    pub fn build_ir(&self, app_view: AppView) -> IRCode {
        let ir_code = IRCode::new(self.clone());
        ir_code
    }
}
