use crate::r8::graph::code::{Code, CodeClone};
use crate::r8::graph::app_view::AppView;
use crate::r8::graph::program_method::ProgramMethod;

#[derive(Clone, Debug)]
pub struct IRCode { pub method: ProgramMethod }

impl IRCode {
    pub fn new(method: ProgramMethod) -> IRCode {
        IRCode {
            method
        }
    }

    pub fn context(&self) -> ProgramMethod {
        self.method.clone()
    }
}
