extern crate dex;

use std::path::PathBuf;

use dex::{Dex, DexReader};
use memmap::Mmap;

pub fn parse_dex_from_file(file: &PathBuf) -> Option<Dex<Mmap>> {
    let result = DexReader::from_file(file);
    match result {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::parse_dex_from_file;

    #[test]
    fn test_parse_apk_binary() {
        let string = String::from("../_fixtures/java/hello/classes.dex");
        let file = Path::new(&string);
        let mmap = parse_dex_from_file(&PathBuf::from(file));
        let result = mmap.unwrap().find_class_by_name("LHelloWorld;");

        let class = result.expect("Result failed").expect("Class failed");

        let str = class.source_file().unwrap();
        assert_eq!("HelloWorld.java", &str.to_string());
        let methods = class.methods();
        let mut count = 0;
        for method in methods {
            count = count + 1;
        }
        assert_eq!(2, count);
    }
}
