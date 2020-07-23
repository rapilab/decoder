use crate::dx::dex::file::header_section::HeaderSection;

pub struct DexFile {}

impl DexFile {
    pub fn new() -> DexFile {
        let section = HeaderSection::new();
        DexFile {}
    }
}
