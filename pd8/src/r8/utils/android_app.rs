use std::path::{PathBuf};

pub trait ClassFileResourceProvider {
    fn get_class_descriptors(&self);
    fn get_program_resource(&self);
}

pub struct AndroidApp {
    classpath_resource_providers: Vec<Box<dyn ClassFileResourceProvider>>,
    library_resource_providers: Vec<Box<dyn ClassFileResourceProvider>>,
}

impl AndroidApp {
    pub fn new() -> AndroidApp {
        AndroidApp {
            classpath_resource_providers: vec![],
            library_resource_providers: vec![],
        }
    }


    fn is_file_by_type(path_buf: PathBuf, suffix: String) -> bool {
        path_buf.as_path().ends_with(suffix)
    }

    // todo: implement it
    fn is_class_file(path_buf: PathBuf) -> bool {
        return false;
    }
    fn is_aar_file(path_buf: PathBuf) -> bool {
        return false;
    }
    fn is_archive(path_buf: PathBuf) -> bool {
        return false;
    }

    pub fn add_program_file(&self, path_buf: PathBuf) -> &AndroidApp {
        if AndroidApp::is_file_by_type(path_buf.clone(), String::from(".dex")) {
            self.add_program_resources();
        } else if AndroidApp::is_class_file(path_buf.clone()) {
            self.add_program_resources();
        } else if AndroidApp::is_aar_file(path_buf.clone()) {
            self.add_program_resource_provider();
        } else if AndroidApp::is_archive(path_buf.clone()) {
            self.add_program_resource_provider();
        } else {
            panic!("error")
        }

        return self
    }

    fn add_program_resources(&self) {}
    fn add_program_resource_provider(&self) {}
}