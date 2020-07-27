use crate::r8::utils::android_app::AndroidApp;

pub struct AppInfo {
    pub(crate) app: AndroidApp
}

impl AppInfo {
    pub fn new(app: AndroidApp) -> AppInfo {
        AppInfo {
            app
        }
    }

    pub fn classes(&self) {
        self.app.classes();
    }
}
