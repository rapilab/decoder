use crate::r8::graph::app_view::AppView;
use crate::r8::utils::android_app::AndroidApp;
use crate::r8::graph::dex_application::DexApplication;
use std::time::Instant;
use crate::r8::graph::dex_program_class::DexProgramClass;

pub struct IRConverter {
    app_view: AppView
}

impl IRConverter {
    pub fn new(app_view: AppView) -> IRConverter {
        IRConverter {
            app_view
        }
    }

    pub fn convert_methods(&self, clazz: DexProgramClass) {
        clazz.get_class_initializer();
    }

    pub fn convert(&self, app: DexApplication) -> DexApplication {
        println!("IR conversion: {}", Instant::now().elapsed().as_secs());
        for clazz in app.classes() {
            self.convert_methods(clazz);
        }

        println!("IR end: {}", Instant::now().elapsed().as_secs());
        app
    }
}

