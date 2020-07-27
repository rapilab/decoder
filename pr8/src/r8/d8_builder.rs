use crate::r8::d8_command::D8Command;
use crate::r8::utils::android_app::AndroidApp;
use std::borrow::Borrow;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct D8Builder {
    pub(crate) app: AndroidApp,
    intermediate: bool,
    synthesized_class_prefix: String,
    enable_main_dex_list_check: bool,
    minimal_main_dex: bool,
    program_files: Vec<PathBuf>,
}

impl D8Builder {
    pub fn new() -> D8Builder {
        D8Builder {
            app: AndroidApp::new(),
            intermediate: false,
            synthesized_class_prefix: "".to_string(),
            enable_main_dex_list_check: false,
            minimal_main_dex: false,
            program_files: vec![],
        }
    }

    pub fn set_intermediate(&mut self, value: bool) {
        self.intermediate = value;
    }

    pub fn add_program_files(&mut self, str: String) {
        let paths = fs::read_dir("./").unwrap();
        for dir in paths {
            let path = dir.unwrap().path();

            if path.is_file() {
                &self.app.add_program_file(path.clone());
                &self.program_files.push(path.clone());
            }
        }
    }

    pub fn make_command(&self) -> D8Command {
        let mut command = D8Command::new();
        command.app = self.app.clone();
        command
    }

    pub fn build(&self) -> D8Command {
        self.make_command()
    }
}
