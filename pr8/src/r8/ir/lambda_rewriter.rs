use crate::r8::graph::app_view::AppView;
use crate::r8::ir::code::ir_code::IRCode;

#[derive(Clone, Debug)]
pub struct LambdaRewriter {
    pub app_view: AppView,
}

impl LambdaRewriter {
    pub fn new(app_view: AppView) -> LambdaRewriter {
        LambdaRewriter { app_view }
    }

    pub fn desugar_lambdas(&self, code: IRCode) {}
}
