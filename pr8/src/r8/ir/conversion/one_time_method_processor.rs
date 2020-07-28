use crate::r8::graph::app_view::AppView;
use crate::r8::graph::program_method::ProgramMethod;

pub struct OneTimeMethodProcessor {
    pub method: ProgramMethod,
    pub app_view: AppView
}

impl OneTimeMethodProcessor {
    pub fn create(method: ProgramMethod, app_view: AppView) -> OneTimeMethodProcessor {
        OneTimeMethodProcessor {
            method,
            app_view
        }
    }
}
