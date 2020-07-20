use std::path::Path;
use std::fs;
use goblin::{Object, error};

pub fn parse_buffer(path: &Path) -> error::Result<()> {
    let buffer = fs::read(path)?;
    match Object::parse(&buffer)? {
        Object::Elf(elf) => {
            println!("elf: {:#?}", &elf);
        }
        Object::PE(pe) => {
            println!("pe: {:#?}", &pe);
        }
        Object::Mach(mach) => {
            println!("mach: {:#?}", &mach);
        }
        Object::Archive(archive) => {
            println!("archive: {:#?}", &archive);
        }
        Object::Unknown(magic) => { println!("unknown magic: {:#x}", magic) }
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::parse_buffer;

    #[test]
    fn test_parser_files() {
        let path = Path::new("../_fixtures/c/hello");
        parse_buffer(path);
        assert_eq!(true, true)
    }
}