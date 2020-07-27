use crate::r8::d8_builder::D8Builder;
use crate::r8::d8_command::D8Command;

pub struct D8CommandParser {}

impl D8CommandParser {
    pub fn new() -> D8CommandParser {
        D8CommandParser {}
    }

    pub fn parse(&self, options: Vec<String>) -> D8Builder {
        let mut builder = D8Command::builder();
        // do something in builder parser
        builder.add_program_files(String::from(""));
        builder.set_intermediate(true);
        builder
    }
}
