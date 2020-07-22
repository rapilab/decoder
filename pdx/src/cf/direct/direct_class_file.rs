use std::path::PathBuf;

pub struct DirectClassFile {
    name: PathBuf,
    bytes: Vec<u8>
}

impl DirectClassFile {
    pub fn new(name: PathBuf, bytes: Vec<u8>) -> DirectClassFile {
        DirectClassFile {
            name,
            bytes
        }
    }

    pub fn parse_to_interfaces_if_necessary(&self) {
        if self.bytes.len() < 10 {

        }
    }
    pub fn get_magic(&self) -> u8 {
        self.parse_to_interfaces_if_necessary();
        self.bytes[0]
    }
}
