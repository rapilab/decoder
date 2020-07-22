#[macro_use]
extern crate failure;

use std::fs::File;
use std::io::Write;

use clap::{App, Arg};
use dex::Dex;
use tempfile::tempdir;

use papk::{get_class_dex, get_content_by_file};
use pclass::parse_class;
use pdex::parse_dex_from_file;

use crate::highlight::highlight_out;
use memmap::Mmap;

mod highlight;

fn main() {
    let app = App::new("decoder")
        .version("0.0.1");

    let papk_opt = Arg::with_name("papk")
        .long("papk")
        .takes_value(true);

    let pclass_opt = Arg::with_name("pclass")
        .long("pclass")
        .takes_value(true);

    let app = app.arg(papk_opt)
        .arg(pclass_opt);

    let matches = app.get_matches();

    match matches.value_of("papk") {
        None => {}
        Some(str) => {
            cmd_papk(String::from(str));
        }
    }
    match matches.value_of("pclass") {
        None => {}
        Some(str) => {
            p_class(String::from(str));
        }
    }
}

pub fn p_class(str: String) -> Result<Dex<Mmap>, failure::Error> {
    let result = get_class_dex(String::from(str));
    let dir = tempdir()?;

    let file_path = dir.path().join("decoder").join("classes.dex");
    let mut file = File::create(file_path.clone())?;

    if let Ok(bytes) = result {
        file.write(&bytes);
        drop(file);
        dir.close()?;

        let option = parse_dex_from_file(&file_path);
        return match option {
            None => {
                bail!("could not parse dex")
            },
            Some(mmap) => {
                Ok(mmap)
            },
        }
    }

    drop(file);
    dir.close()?;

    bail!("could not find class.dex")
}

pub fn cmd_papk(str: String) {
    let result = get_content_by_file(str, String::from("AndroidManifest.xml"));
    if let Ok(str) = result {
        highlight_out(str.as_str(), "xml");
    }
}

#[cfg(test)]
mod tests {
    use crate::p_class;
    use dex::Dex;
    use failure::Error;
    use memmap::Mmap;

    #[test]
    fn test_parse_c_binary() {
        let mmap = p_class(String::from("../_fixtures/apk/app-release-unsigned.apk"));
        if let Ok(dex) = mmap {
            for c in dex.classes() {
                println!("{:?}", c.unwrap().source_file());
            }
        }
    }
}