use std::io;
use std::path::Path;

use dex::{Dex, DexReader, Error};
use dex::class::Class;
use memmap::Mmap;

fn main() {
    println!("Hello, world!");
}

pub fn parse_dex(path: String) -> Option<Dex<Mmap>> {
    let file = Path::new(&path);
    let dex = DexReader::from_file(file);
    match dex {
        Ok(data) => {
            Some(data)
        }
        Err(_) => {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_dex;
    use dex::class::Class;

    #[test]
    fn test_parse_c_binary() {
        let mmap = parse_dex(String::from("../_fixtures/java/hello/classes.dex"));
        let result = mmap.unwrap().find_class_by_name("LHelloWorld;");

        let option = result.unwrap();
        match option {
            None => {},
            Some(class) => {
                let str = class.source_file().unwrap();
                assert_eq!("HelloWorld.java", &str.to_string())
            },
        }
    }
}
