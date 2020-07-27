use std::path::PathBuf;

use crate::dx::command::class_def_item_consumer::ClassDefItemConsumer;
use crate::dx::command::class_translator_task::ClassTranslatorTask;
use crate::dx::command::dexer::rotate_dex_file;
use crate::dx::dex::cf::direct::direct_class_file::DirectClassFile;

pub struct DirectClassFileConsumer {
    name: PathBuf,
    bytes: Vec<u8>,
}

impl DirectClassFileConsumer {
    pub fn new(name: PathBuf, bytes: Vec<u8>) -> DirectClassFileConsumer {
        DirectClassFileConsumer { name, bytes }
    }

    pub fn call(&self, class: DirectClassFile) {
        rotate_dex_file();

        let task = ClassTranslatorTask::new(self.name.clone(), self.bytes.clone(), class);
        task.call();
        ClassDefItemConsumer::new();
    }
}
