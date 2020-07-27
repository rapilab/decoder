use std::borrow::Borrow;

pub struct D8Builder {
    intermediate: bool,
    synthesized_class_prefix: String,
    enable_main_dex_list_check: bool,
    minimal_main_dex: bool
}

impl D8Builder {
    pub fn new() -> D8Builder {
        D8Builder {
            intermediate: false,
            synthesized_class_prefix: "".to_string(),
            enable_main_dex_list_check: false,
            minimal_main_dex: false
        }
    }

    pub fn set_intermediate(&mut self, value: bool) {
        self.intermediate = value;
    }
}
