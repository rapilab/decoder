use crate::r8::graph::app_view::AppView;
use crate::r8::utils::android_app::AndroidApp;
use crate::r8::graph::dex_application::DexApplication;

pub struct IRConverter {
    app_view: AppView
}

impl IRConverter {
    pub fn new(app_view: AppView) -> IRConverter {
        IRConverter {
            app_view
        }
    }

    pub fn convert(&self, app: DexApplication) -> DexApplication {
        app
    }
}

