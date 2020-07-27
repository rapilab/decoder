use crate::r8::graph::app_view::AppView;
use crate::r8::utils::android_app::AndroidApp;
use crate::r8::graph::dex_application::DexApplication;
use std::time::Instant;

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
        println!("IR conversion: {}", Instant::now().elapsed().as_secs());

        println!("IR end: {}", Instant::now().elapsed().as_secs());
        app
    }
}

