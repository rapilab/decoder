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

#[cfg(test)]
mod tests {}
