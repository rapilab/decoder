use crate::r8::graph::dex_application::DexApplication;

#[derive(Clone)]
pub struct AppInfo {
    pub(crate) app: DexApplication,
}

impl AppInfo {
    pub fn new(app: DexApplication) -> AppInfo {
        AppInfo { app }
    }

    pub fn classes(&self) {
        self.app.classes();
    }
}
