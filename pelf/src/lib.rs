use goblin::{error, Object};
use std::fs;
use std::path::Path;

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
        Object::Unknown(magic) => println!("unknown magic: {:#x}", magic),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_buffer;
    use std::path::Path;

    #[test]
    fn test_parse_c_binary() {
        let path = Path::new("../_fixtures/c/hello");
        parse_buffer(path);
        assert_eq!(true, true)
    }

    #[test]
    fn test_parse_rust_binary() {
        let path = Path::new("../_fixtures/rust/hello");
        parse_buffer(path);
        assert_eq!(true, true)
    }
}
