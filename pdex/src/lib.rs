extern crate dex;

use std::io;
use std::path::{Path, PathBuf};

use dex::{Dex, DexReader, Error};
use memmap::Mmap;

pub fn parse_dex_from_file(file: &PathBuf) -> Option<Dex<Mmap>> {
    let dex = DexReader::from_file(file);
    match dex {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_dex_from_file;
    use dex::class::Class;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_parse_c_binary() {
        let string = String::from("../_fixtures/java/hello/classes.dex");
        let file = Path::new(&string);
        let mmap = parse_dex_from_file(&PathBuf::from(file));
        let result = mmap.unwrap().find_class_by_name("LHelloWorld;");

        let option = result.unwrap();
        match option {
            None => {}
            Some(class) => {
                let str = class.source_file().unwrap();
                // assert_eq!("HelloWorld.java", &str.to_string())
            }
        }
    }
}
