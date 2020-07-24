use crate::dx::dex::cf::direct::direct_class_file::DirectClassFile;
use crate::dx::dex::file::class_def_item::ClassDefItem;

pub struct CfTranslator {}

impl CfTranslator {
    pub fn process_fields() {

    }

    pub fn process_methods() {

    }

    pub fn translate0(bytes: Vec<u8>, cf: DirectClassFile) -> ClassDefItem {
        let out = ClassDefItem::new();
        CfTranslator::process_fields();
        CfTranslator::process_methods();

        let cp_size = cf.get_constant_pool().size();
        for i in 0..cp_size - 1 {

        }
        out
    }

    pub fn translate(bytes: Vec<u8>, cf: DirectClassFile) -> ClassDefItem {
        CfTranslator::translate0(bytes, cf)
    }
}