use crate::dx::dex::cf::direct::direct_class_file::DirectClassFile;

pub struct CfTranslator {}

impl CfTranslator {
    pub fn process_fields() {

    }

    pub fn process_methods() {

    }

    pub fn translate0(bytes: Vec<u8>, cf: DirectClassFile) {
        CfTranslator::process_fields();
        CfTranslator::process_methods();

    }

    pub fn translate(bytes: Vec<u8>, cf: DirectClassFile) {
        CfTranslator::translate0(bytes, cf);
    }
}