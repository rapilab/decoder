#[macro_use]
extern crate failure;

use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use abxml::apk::Apk;
use clap::{App, Arg};
use dex::Dex;
use failure::{Error, Fail};
use memmap::Mmap;
use tempfile::tempdir;

use papk::{get_classes_dex, get_content_by_file};
use pclass::parse_class;
use pdex::parse_dex_from_file;

use crate::highlight::highlight_out;

mod highlight;

fn main() {
    let papk_opt = Arg::with_name("papk").long("papk").takes_value(true);
    let pclass_opt = Arg::with_name("pclass").long("pclass").takes_value(true);
    let unpack_opt = Arg::with_name("unpack").long("unpack").takes_value(true);

    let app = App::new("decoder").version("0.0.1")
        .arg(papk_opt)
        .arg(pclass_opt)
        .arg(unpack_opt);

    let matches = app.get_matches();

    if let Some(str) = matches.value_of("papk") {
        cmd_papk(String::from(str));
    }

    if let Some(str) = matches.value_of("pclass") {
        cmd_pclass(String::from(str));
    }

    if let Some(str) = matches.value_of("unpack_opt") {
        cmd_unpack(String::from(str));
    }
}

fn cmd_unpack(str: String) -> Result<(), failure::Error> {
    let path = Path::new(&str);

    let mut apk = Apk::from_path(&path)?;
    let output = Path::new("apk_output/");
    let result = apk.export(output, true);
    match result {
        Ok(_) => {},
        Err(err) => {
            println!("{:?}", err);
        },
    }

    Ok(())
}

pub fn cmd_pclass(str: String) -> Result<Dex<Mmap>, failure::Error> {
    let result = get_classes_dex(String::from(str));
    let dir = tempdir()?;

    let decoder_dir = dir.path().join("decoder");
    fs::create_dir(decoder_dir.to_path_buf())?;
    let file_path = decoder_dir.join("classes.dex");
    let mut file = File::create(file_path.clone())?;

    if let Ok(bytes) = result {
        file.write(&bytes);

        let option = parse_dex_from_file(&file_path);

        drop(file);
        dir.close()?;

        return match option {
            None => bail!("could not parse dex"),
            Some(mmap) => {
                for cr in mmap.classes() {
                    if let Ok(class) = cr {
                        println!("{:?}", class.source_file());
                    }
                }
                Ok(mmap)
            }
        };
    }

    println!("could not find classes.dex");
    drop(file);
    dir.close()?;

    bail!("could not find classes.dex")
}

pub fn cmd_papk(str: String) {
    let result = get_content_by_file(str, String::from("AndroidManifest.xml"));
    match result {
        Ok(str) => {
            highlight_out(str.as_str(), "xml");
        }
        Err(_) => {
            println!("parse xml error");
        }
    }
}

#[cfg(test)]
mod tests {
    use dex::class::Class;
    use dex::Dex;
    use failure::Error;
    use memmap::Mmap;

    use crate::{cmd_papk, cmd_pclass, cmd_unpack};

    #[test]
    fn test_parse_apk_binary() {
        cmd_papk(String::from("_fixtures/apk/app-release-unsigned.apk"));
    }

    #[test]
    fn test_unpack_apk_binary() {
        cmd_unpack(String::from("_fixtures/apk/app-release-unsigned.apk"));
    }

    #[test]
    fn test_parse_class_binary() {
        let mmap = cmd_pclass(String::from("_fixtures/apk/app-release-unsigned.apk"));
        if let Ok(dex) = mmap {
            for cr in dex.classes() {
                if let Ok(class) = cr {
                    println!("{:?}", class.source_file());
                }
            }
        }
    }
}
