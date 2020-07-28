use crate::r8::graph::app_view::AppView;
use crate::r8::ir::code::ir_code::IRCode;

pub struct CodeRewriter {
    pub app_view: AppView,
}

impl CodeRewriter {
    pub fn new(app_view: AppView) -> CodeRewriter {
        CodeRewriter { app_view }
    }

    pub fn rewrite_known_array_length_calls(&self, code: IRCode) {}
    pub fn rewrite_assertion_error_two_argument_constructor(&self, code: IRCode) {}
    pub fn common_subexpression_elimination(&self, code: IRCode) {}
    pub fn simplify_array_construction(&self, code: IRCode) {}
    pub fn rewrite_move_result(&self, code: IRCode) {}
    pub fn split_range_invoke_constants(&self, code: IRCode) {}
    pub fn optimize_always_throwing_instructions(&self, code: IRCode) {}
}
