use crate::r8::d8_command_parser::D8CommandParser;
use crate::r8::utils::android_app::AndroidApp;
use crate::r8::dex::application_reader::ApplicationReader;
use crate::r8::graph::app_info::AppInfo;
use crate::r8::ir::ir_converter::IRConverter;
use crate::r8::graph::app_view::AppView;
use crate::r8::jar::cf_application_writer::CfApplicationWriter;

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

    fn run(input_app: AndroidApp) {
        let reader = ApplicationReader::new(input_app.clone());
        let mut app = reader.read();

        let app_info = AppInfo::new(app.clone());
        let app_view = AppView::create_for_d8(app_info.clone());

        let converter = IRConverter::new(app_view.clone());
        app = converter.convert(app);

        let writer = CfApplicationWriter::new(app, app_view);
        writer.write();
        app_info.classes();
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
