#[derive(Debug, Clone)]
pub struct DexString {
    size: usize,
    content: Vec<u8>
}

impl DexString {
    pub fn new(string: String) -> DexString {
        DexString {
            size: string.len(),
            content: Vec::from(string.as_bytes())
        }
    }
}
