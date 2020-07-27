use std::path::{PathBuf};
use crate::r8::graph::dex_program_class::DexProgramClass;

pub trait ProviderClone {
    fn clone_box(&self) -> Box<dyn ClassFileResourceProvider>;
}

impl<T> ProviderClone for T
    where
        T: 'static + ClassFileResourceProvider + Clone,
{
    fn clone_box(&self) -> Box<dyn ClassFileResourceProvider> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn ClassFileResourceProvider> {
    fn clone(&self) -> Box<dyn ClassFileResourceProvider> {
        self.clone_box()
    }
}

pub trait ClassFileResourceProvider: ProviderClone {
    fn get_class_descriptors(&self);
    fn get_program_resource(&self);
}

#[derive(Clone)]
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

    pub fn classes(&self) -> Vec<DexProgramClass> {
        let classes: Vec<DexProgramClass> = vec![];
        classes
    }


    fn is_file_by_type(path_buf: PathBuf, suffix: String) -> bool {
        // path_buf.as_path().ends_with(suffix)
        return true
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

        return self;
    }

    fn add_program_resources(&self) {}
    fn add_program_resource_provider(&self) {}

    //Build final AndroidApp.
    pub fn build(&self) -> AndroidApp {
        let classpath_resource_providers = self.classpath_resource_providers.clone();
        let library_resource_providers = self.library_resource_providers.clone();

        // some things in here
        AndroidApp {
            classpath_resource_providers,
            library_resource_providers,
        }
    }
}