use crate::r8::graph::app_view::AppView;
use crate::r8::graph::program_method::ProgramMethod;
use crate::r8::ir::inliner::Inliner;
use crate::r8::ir::code::ir_code::IRCode;

#[derive(Clone, Debug)]
pub struct LambdaMerger { pub app_view: AppView }

impl LambdaMerger {
    pub fn new(app_view: AppView) -> LambdaMerger {
        LambdaMerger {app_view}
    }

    pub fn rewrite_code(&self, method: ProgramMethod, code: IRCode, inliner: Inliner) {

    }
}
