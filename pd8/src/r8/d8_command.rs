use crate::r8::d8_builder::D8Builder;

pub struct D8Command {

}

impl D8Command {
    pub fn builder() -> D8Builder {
        D8Builder::new()
    }
}
