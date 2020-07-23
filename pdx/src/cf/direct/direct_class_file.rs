use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DirectClassFile {
    name: Box<PathBuf>,
    bytes: Box<Vec<u8>>,
}

impl DirectClassFile {
    pub fn new(name: Box<PathBuf>, bytes: Box<Vec<u8>>) -> DirectClassFile {
        DirectClassFile { name, bytes }
    }

    pub fn set_attribute_factory(&self) {}

    pub fn is_good_magic(&self, magic: i32) -> bool {
        return true
    }

    pub fn get_magic0(&self) -> i32 {
        return 0
    }

    pub fn parse0(&self) {
        if self.bytes.len() < 10 {
            // throw error
        }
        if self.is_good_magic(self.get_magic0()) {

        }
        //todo: is good version

    }

    pub fn parse_to_interfaces_if_necessary(&self) {
        self.parse0();
    }

    pub fn get_magic(&self) -> u8 {
        self.parse_to_interfaces_if_necessary();
        self.bytes[0]
    }
}
