use std::borrow::Borrow;
use crate::r8::utils::android_app::AndroidApp;
use std::fs;

pub struct D8Builder {
    app: AndroidApp,
    intermediate: bool,
    synthesized_class_prefix: String,
    enable_main_dex_list_check: bool,
    minimal_main_dex: bool
}

impl D8Builder {
    pub fn new() -> D8Builder {
        D8Builder {
            app: AndroidApp::new(),
            intermediate: false,
            synthesized_class_prefix: "".to_string(),
            enable_main_dex_list_check: false,
            minimal_main_dex: false
        }
    }

    pub fn set_intermediate(&mut self, value: bool) {
        self.intermediate = value;
    }

    pub fn add_program_files(&self, str: String) {
        let paths = fs::read_dir("./").unwrap();
        for dir in paths {
            let path = dir.unwrap().path();

            if path.is_file() {
                &self.app.add_program_file(path);
            }
        }
    }
}
