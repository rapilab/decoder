use std::path::PathBuf;
use crate::cf::direct::direct_class_file::DirectClassFile;

pub struct ClassParserTask {
    name: PathBuf,
    bytes: Vec<u8>
}

impl ClassParserTask {
    pub fn new() -> ClassParserTask {
        ClassParserTask {
            name: Default::default(),
            bytes: vec![]
        }
    }

    pub fn call(&self) -> DirectClassFile {
        let cf = DirectClassFile::new(self.name, self.bytes);
        cf.get_magic();
        cf
    }
}