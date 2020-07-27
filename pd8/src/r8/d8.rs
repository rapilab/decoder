use crate::r8::d8_command_parser::D8CommandParser;
use crate::r8::utils::android_app::AndroidApp;
use crate::r8::dex::application_reader::ApplicationReader;

pub struct D8 {}

impl D8 {
    pub fn new() -> D8 {
        D8 {}
    }

    pub fn start() {
        let mut options = vec![];
        options.push(String::from(""));
        let parser = D8CommandParser::new().parse(options);
        let command = parser.build();
        let app = command.app;
        D8::run(app)
    }

    fn run(app: AndroidApp) {
        let reader = ApplicationReader::new(app);
        reader.read();
    }
}

#[cfg(test)]
mod tests {
    use crate::r8::d8::D8;

    #[test]
    fn test_urn() {
        D8::start();
    }
}
