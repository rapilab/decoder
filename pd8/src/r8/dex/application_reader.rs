use crate::r8::utils::android_app::AndroidApp;
use crate::r8::graph::dex_application::DexApplication;

pub struct ApplicationReader {
    app: AndroidApp
}

impl ApplicationReader {
    pub fn new(app: AndroidApp) -> ApplicationReader {
        ApplicationReader {
            app
        }
    }

    pub fn read(&self) {
        let builder = DexApplication::builder();
    }
}