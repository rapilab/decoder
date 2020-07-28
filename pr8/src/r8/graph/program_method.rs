use crate::r8::graph::dex_encoded_method::DexEncodedMethod;
use crate::r8::graph::dex_program_class::DexProgramClass;
use crate::r8::graph::app_view::AppView;
use crate::r8::ir::code::ir_code::IRCode;

#[derive(Clone, Debug)]
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

    pub fn build_ir(&self, app_view: AppView) -> IRCode {
        let method = self.get_definition();
        // method.code.build_ir(method, app_view)
        let ir_code = IRCode::new();
        ir_code
    }

}
