use crate::r8::d8_builder::D8Builder;
use crate::r8::utils::android_app::AndroidApp;

pub struct D8Command {
    pub(crate) app: AndroidApp,
}

impl D8Command {
    pub fn new() -> D8Command {
        D8Command {
            app: AndroidApp::new(),
        }
    }

    pub fn builder() -> D8Builder {
        let builder = D8Builder::new();
        builder.app.build();
        builder
    }
}
