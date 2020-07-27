pub trait ClassFileConsumer {
    fn accept(&self);
}

pub struct ForwardingConsumer {}

impl ForwardingConsumer {
    pub fn new() -> ForwardingConsumer {
        ForwardingConsumer {}
    }
}

impl ClassFileConsumer for ForwardingConsumer {
    fn accept(&self) {}
}
