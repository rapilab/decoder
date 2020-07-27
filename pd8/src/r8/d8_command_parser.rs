use crate::r8::d8_command::D8Command;
use crate::r8::d8_builder::D8Builder;

pub struct D8CommandParser {

}

impl D8CommandParser {
    pub fn new() -> D8CommandParser {
        D8CommandParser{}
    }

    pub fn parse(&self) -> D8Builder {
        let mut builder = D8Command::builder();
        // do something in builder parser
        builder.set_intermediate(true);
        builder
    }

    pub fn make_command(&self) {

    }
    pub fn build(&self) {
        self.make_command();
    }
}

