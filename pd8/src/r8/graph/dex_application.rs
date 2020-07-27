use crate::r8::graph::lazy_loaded_dex_application::LazyLoadedDexApplicationBuilder;
use crate::r8::graph::dex_program_class::DexProgramClass;

#[derive(Clone, Debug)]
pub struct DexApplication {}

impl DexApplication {
    pub fn new() -> DexApplication {
        DexApplication {}
    }

    pub fn builder() -> LazyLoadedDexApplicationBuilder {
        let builder = LazyLoadedDexApplicationBuilder::new();
        builder
    }

    pub fn classes(&self) -> Vec<DexProgramClass> {
        let classes: Vec<DexProgramClass> = vec![];
        classes
    }
}
