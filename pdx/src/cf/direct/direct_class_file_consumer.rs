use std::path::PathBuf;
use crate::cf::direct::direct_class_file::DirectClassFile;

pub struct DirectClassFileConsumer {
    name: Box<PathBuf>,
    bytes: Box<Vec<u8>>
}

impl DirectClassFileConsumer {
    pub fn new(name: PathBuf, bytes: Vec<u8>) -> DirectClassFileConsumer {
        DirectClassFileConsumer {
            name: Box::from(name),
            bytes: Box::from(bytes)
        }
    }

    pub fn call(&self, class: DirectClassFile) {

    }
}