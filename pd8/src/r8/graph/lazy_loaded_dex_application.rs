#[derive(Clone, Debug)]
pub struct LazyLoadedDexApplication {}

impl LazyLoadedDexApplication {
    pub fn new() -> LazyLoadedDexApplication {
        LazyLoadedDexApplication {}
    }
}

pub struct LazyLoadedDexApplicationBuilder {}

impl LazyLoadedDexApplicationBuilder {
    pub fn new() -> LazyLoadedDexApplicationBuilder {
        LazyLoadedDexApplicationBuilder {}
    }

    pub fn build(&self) -> LazyLoadedDexApplication {
        LazyLoadedDexApplication::new()
    }
}
