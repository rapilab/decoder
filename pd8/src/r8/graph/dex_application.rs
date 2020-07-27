use crate::r8::graph::lazy_loaded_dex_application::LazyLoadedDexApplicationBuilder;

pub struct DexApplication {}

impl DexApplication {
    pub fn new() -> DexApplication {
        DexApplication {}
    }

    pub fn builder() -> LazyLoadedDexApplicationBuilder {
        let builder = LazyLoadedDexApplicationBuilder::new();
        builder
    }
}
