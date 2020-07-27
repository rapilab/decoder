use crate::r8::utils::android_app::AndroidApp;
use crate::r8::graph::dex_application::DexApplication;
use crate::r8::graph::lazy_loaded_dex_application::LazyLoadedDexApplication;

pub struct ApplicationReader {
    app: AndroidApp
}

impl ApplicationReader {
    pub fn new(app: AndroidApp) -> ApplicationReader {
        ApplicationReader {
            app
        }
    }

    pub fn read(&self) -> LazyLoadedDexApplication {
        let builder = DexApplication::builder();
        builder.build()
    }
}