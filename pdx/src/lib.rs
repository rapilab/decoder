use std::path::PathBuf;

use crate::cf::direct::class_parser_task::ClassParserTask;
use crate::cf::direct::direct_class_file_consumer::DirectClassFileConsumer;
use crate::dx::file::dex_file::DexFile;

mod cf;

pub mod dx;

pub fn create_dex_file() {
    let output_dex = DexFile::new();
    process_one();
}

pub fn rotate_dex_file() {
    create_dex_file();
}

pub fn process_one() {}

pub fn check_class_name(name: PathBuf) {}

pub fn process_class(name: PathBuf, bytes: Vec<u8>) {
    check_class_name(name.clone());

    let consumer = DirectClassFileConsumer::new(name.clone(), bytes.clone());
    let parser_task = ClassParserTask::new(name, bytes);
    let class_file = parser_task.call();
    consumer.call(class_file);
}

pub fn process_file_bytes(name: PathBuf, bytes: Vec<u8>) -> bool {
    let is_class = name.ends_with(".class");
    let is_classes_dex = name.to_path_buf().as_os_str().to_os_string() == "classes.dex";

    if !is_class && !is_classes_dex {
        return false;
    }

    process_class(name, bytes);
    return true;
}

#[cfg(test)]
mod tests {}
