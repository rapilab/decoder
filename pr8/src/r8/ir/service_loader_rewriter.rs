use crate::r8::graph::app_view::AppView;
use crate::r8::ir::code::ir_code::IRCode;

pub struct ServiceLoaderRewriter {
    app_view: AppView
}

impl ServiceLoaderRewriter {
    pub fn new(app_view: AppView) -> ServiceLoaderRewriter {
        ServiceLoaderRewriter { app_view }
    }

    pub fn rewrite(&self, code: IRCode) {

    }
}