use crate::r8::d8_command_parser::D8CommandParser;

pub struct D8 {}

impl D8 {
    pub fn new() -> D8 {
        D8 {}
    }

    pub fn run() {
        let parser = D8CommandParser::new().build();
        // let builder = parser.parse();

    }
}

#[cfg(test)]
mod tests {
    use crate::r8::d8::D8;

    #[test]
    fn test_urn() {
        D8::run();
    }
}
