use crate::r8::graph::lazy_loaded_dex_application::LazyLoadedDexApplicationBuilder;
use crate::r8::graph::dex_program_class::DexProgramClass;

#[derive(Clone, Debug)]
pub struct DexApplication {}

impl DexApplication {
    pub fn new() -> DexApplication {
        DexApplication {}
    }

    pub fn builder() -> DexApplicationBuilder {
        let builder = DexApplicationBuilder::new();
        builder
    }

    pub fn classes(&self) -> Vec<DexProgramClass> {
        let classes: Vec<DexProgramClass> = vec![];
        classes
    }
}

#[derive(Clone, Debug)]
pub struct DexApplicationBuilder {
    pub app: DexApplication
}

impl DexApplicationBuilder {
    pub fn new() -> DexApplicationBuilder {
        let application = DexApplication::new();
        DexApplicationBuilder{
            app: application
        }
    }
    pub fn build(&self) -> DexApplication {
        self.app.clone()
    }
}