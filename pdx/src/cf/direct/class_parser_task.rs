use crate::cf::direct::direct_class_file::DirectClassFile;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ClassParserTask {
    name: Box<PathBuf>,
    bytes: Box<Vec<u8>>,
}

impl ClassParserTask {
    pub fn new(name: PathBuf, bytes: Vec<u8>) -> ClassParserTask {
        ClassParserTask {
            name: Box::new(name),
            bytes: Box::new(bytes),
        }
    }

    pub fn call(&self) -> DirectClassFile {
        let cf = DirectClassFile::new(self.name.clone(), self.bytes.clone());
        cf.get_magic();
        cf
    }
}
