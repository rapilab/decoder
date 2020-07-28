use crate::r8::graph::app_view::AppView;
use crate::r8::ir::code::ir_code::IRCode;

pub struct StringOptimizer {
    pub app_view: AppView,
}

impl StringOptimizer {
    pub fn new(app_view: AppView) -> StringOptimizer {
        StringOptimizer { app_view }
    }

    pub fn rewrite_class_get_name(&self, app_view: AppView, code: IRCode) {}
    pub fn compute_trivial_operations_on_const_string(&self, code: IRCode) {}
    pub fn remove_trivial_conversions(&self, code: IRCode) {}
}
