use std::fs;
use papk::get_classes_dex;
use memmap::Mmap;
use dex::Dex;
use tempfile::tempdir;
use std::fs::File;
use pdex::parse_dex_from_file;
use std::io::Write;

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

#[cfg(test)]
mod tests {
    use crate::cmd::cmd_pclass::cmd_pclass;
    use failure::Error;
    use memmap::Mmap;

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
