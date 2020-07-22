mod cf;

use crate::dx::file::dex_file::DexFile;

pub mod dx;

pub fn create_dex_file() {
    let output_dex = DexFile::new();
    process_one();
}

pub fn process_one() {}

#[cfg(test)]
mod tests {}
