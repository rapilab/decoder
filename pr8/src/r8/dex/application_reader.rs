use crate::r8::graph::dex_application::DexApplication;
use crate::r8::utils::android_app::AndroidApp;

pub struct ApplicationReader {
    app: AndroidApp,
}

impl ApplicationReader {
    pub fn new(app: AndroidApp) -> ApplicationReader {
        ApplicationReader { app }
    }

    pub fn read(&self) -> DexApplication {
        let builder = DexApplication::builder();
        let application = builder.build();

        application
    }
}
