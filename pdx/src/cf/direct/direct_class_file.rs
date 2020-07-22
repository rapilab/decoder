use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DirectClassFile {
    name: Box<PathBuf>,
    bytes: Box<Vec<u8>>,
}

impl DirectClassFile {
    pub fn new(name: &Box<PathBuf>, bytes: &Box<Vec<u8>>) -> DirectClassFile {
        DirectClassFile {
            // name: Box::from(name),
            // bytes
            name: Box::new(Default::default()),
            bytes: Box::new(vec![])
        }
    }

    pub fn parse_to_interfaces_if_necessary(&self) {
        if self.bytes.len() < 10 {}
    }
    pub fn get_magic(&self) -> u8 {
        self.parse_to_interfaces_if_necessary();
        self.bytes[0]
    }
}
