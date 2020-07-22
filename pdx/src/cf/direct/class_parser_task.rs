use std::path::PathBuf;
use crate::cf::direct::direct_class_file::DirectClassFile;

#[derive(Debug, Clone)]
pub struct ClassParserTask {
    name: Box<PathBuf>,
    bytes: Box<Vec<u8>>
}

impl ClassParserTask {
    pub fn new(name: PathBuf, bytes: Vec<u8>) -> ClassParserTask {
        ClassParserTask {
            name: Box::from(name),
            bytes: Box::from(bytes)
        }
    }

    pub fn call(&self) -> DirectClassFile {
        let cf = DirectClassFile::new(&self.name, &self.bytes);
        cf.get_magic();
        cf
    }
}