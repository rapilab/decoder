use std::path::PathBuf;
use crate::dx::command::dexer::translate_class;
use crate::dx::dex::cf::direct::direct_class_file::DirectClassFile;

pub struct ClassTranslatorTask {
    name: PathBuf,
    bytes: Vec<u8>,
    pub cf: DirectClassFile
}

impl ClassTranslatorTask {
    pub fn new(name: PathBuf, bytes: Vec<u8>, cf: DirectClassFile) -> ClassTranslatorTask {
        ClassTranslatorTask {
            name,
            bytes,
            cf
        }
    }

    pub fn call(&self) {
        translate_class(self.bytes.clone(), self.cf.clone());
    }
}