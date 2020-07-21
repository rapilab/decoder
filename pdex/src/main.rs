use std::io;
use std::path::Path;
use dex::DexReader;

fn main() {
    println!("Hello, world!");
}


fn parse_dex(path: String) -> io::Result<()> {
    let file = Path::new(&path);
    let dex = DexReader::from_file(file).unwrap();
    let class = dex
        .find_class_by_name("LHelloWorld;")
        .expect("Failed to load class")
        .expect("class not found");
    println!("class type: {}", class.jtype());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_dex;

    #[test]
    fn test_parse_c_binary() {
        parse_dex(String::from("../_fixtures/java/hello/classes.dex"));
    }
}
