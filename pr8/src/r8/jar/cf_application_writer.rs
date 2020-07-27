use crate::r8::graph::app_view::AppView;
use crate::r8::graph::dex_application::DexApplication;
use crate::r8::support::class_file_consumer::{ClassFileConsumer, ForwardingConsumer};

pub struct CfApplicationWriter {
    app: DexApplication,
    app_view: AppView,
}

impl CfApplicationWriter {
    pub fn new(app: DexApplication, app_view: AppView) -> CfApplicationWriter {
        CfApplicationWriter { app, app_view }
    }

    pub fn write_application(&self, consumer: Box<dyn ClassFileConsumer>) {}

    pub fn write(&self) {
        let consumer = ForwardingConsumer::new();
        self.write_application(Box::from(consumer))
    }
}
