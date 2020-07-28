pub struct R8 {}

impl R8 {
    pub fn start() {}

    pub fn run() {}
}

#[cfg(test)]
mod tests {
    use crate::r8::r8::R8;

    #[test]
    fn test_urn() {
        R8::start();
    }
}
