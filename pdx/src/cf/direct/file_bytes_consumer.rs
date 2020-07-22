use std::path::PathBuf;
use crate::cf::direct::process_file_bytes;

pub struct FileBytesConsumer {

}

impl FileBytesConsumer {
    pub fn new() -> FileBytesConsumer {
        FileBytesConsumer {

        }
    }

    pub fn process_file_bytes(&self, name: PathBuf, bytes: Vec<u8>) {
        process_file_bytes(name, bytes);
    }
}