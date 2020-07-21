fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod tests {
    use std::io;
    use dex::DexReader;
    use std::path::Path;

    #[test]
    fn test_parse_c_binary() {
        parse_dex();
    }

    fn parse_dex() -> io::Result<()> {
        let file = Path::new("../_fixtures/java/hello/classes.dex");
        let dex = DexReader::from_file(file).unwrap();
        let class = dex
            .find_class_by_name("LHelloWorld;")
            .expect("Failed to load class")
            .expect("class not found");
        println!("class type: {}", class.jtype());
        Ok(())
    }
}
