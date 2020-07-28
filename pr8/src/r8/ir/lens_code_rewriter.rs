use crate::r8::graph::app_view::AppView;
use crate::r8::graph::program_method::ProgramMethod;
use crate::r8::ir::code::ir_code::IRCode;

#[derive(Clone, Debug)]
pub struct LensCodeRewriter {
    pub app_view: AppView,
}

impl LensCodeRewriter {
    pub fn new(app_view: AppView) -> LensCodeRewriter {
        LensCodeRewriter { app_view }
    }

    pub fn rewrite(&self, code: IRCode, context: ProgramMethod) {}
}
