use crate::r8::graph::app_info::AppInfo;

#[derive(Clone)]
pub struct AppView {
    app_info: AppInfo,
}

impl AppView {
    pub fn create_for_d8(app_info: AppInfo) -> AppView {
        AppView { app_info }
    }
}
