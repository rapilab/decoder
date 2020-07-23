use std::path::PathBuf;
use crate::dx::dex::cf::direct::direct_class_file::DirectClassFile;

#[derive(Debug, Clone)]
pub struct ClassParserTask {
    name: PathBuf,
    bytes: Vec<u8>,
}

impl ClassParserTask {
    pub fn new(name: PathBuf, bytes: Vec<u8>) -> ClassParserTask {
        ClassParserTask {
            name: name,
            bytes: bytes,
        }
    }

    pub fn call(&self) -> DirectClassFile {
        let cf = DirectClassFile::new(self.name.clone(), self.bytes.clone());
        cf.set_attribute_factory();
        cf.get_magic();
        cf
    }
}
