use crate::cf::direct::file_bytes_consumer::FileBytesConsumer;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

pub struct ClassPathOpener {
    pathname: String,
    pub consumer: FileBytesConsumer,
}

impl ClassPathOpener {
    pub fn new(pathname: String) -> ClassPathOpener {
        ClassPathOpener {
            pathname,
            consumer: FileBytesConsumer::new(),
        }
    }

    pub fn process_directory(&self, path: PathBuf) {}

    pub fn process_archive(&self, path: PathBuf) {}

    pub fn process_one(&self, path: PathBuf) {
        if path.is_dir() {
            return self.process_directory(path);
        }

        if path.ends_with(".zip") || path.ends_with(".jar") || path.ends_with(".apk") {
            return self.process_archive(path);
        }

        let data = fs::read(path.clone()).expect("Unable to read file");
        self.consumer.process_file_bytes(path, data);
    }
}
