use pclass_parser::classfile::ClassFile;
use pclass_parser::parse_class as pclass_parser;

pub fn parse_class(input: &[u8]) -> nom::IResult<&[u8], ClassFile> {
    pclass_parser(input)
}

#[cfg(test)]
mod tests {
    use crate::parse_class;
    use nom::error::ErrorKind;
    use pclass_parser::classfile::ClassFile;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_parse_class() {
        let path = Path::new("../_fixtures/java/hello/HelloWorld.class");
        let buffer = fs::read(path).unwrap();
        let result = parse_class(buffer.as_ref());
        match result {
            Ok((_, class)) => assert_eq!(52, class.version.major),
            Err(_) => {}
        }
    }
}
