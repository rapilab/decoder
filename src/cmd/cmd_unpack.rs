use abxml::apk::Apk;
use std::path::Path;

pub fn cmd_unpack(str: String) -> Result<(), failure::Error> {
    let path = Path::new(&str);

    let mut apk = Apk::from_path(&path)?;
    let output = Path::new("apk_output/");
    let result = apk.export(output, true);
    match result {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use dex::class::Class;
    use dex::Dex;
    use failure::Error;
    use memmap::Mmap;

    use crate::cmd::cmd_unpack::cmd_unpack;

    #[test]
    fn test_unpack_apk_binary() {
        cmd_unpack(String::from("_fixtures/apk/app-release-unsigned.apk"));
    }
}
