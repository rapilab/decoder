use std::path::PathBuf;
use crate::cf::direct::direct_class_file::DirectClassFile;

pub struct DirectClassFileConsumer {
    name: PathBuf,
    bytes: Vec<u8>
}

impl DirectClassFileConsumer {
    pub fn new() -> DirectClassFileConsumer {
        DirectClassFileConsumer {
            name: Default::default(),
            bytes: vec![]
        }
    }

    pub fn call(&self, class: DirectClassFile) {

    }
}