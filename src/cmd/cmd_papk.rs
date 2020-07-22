use crate::highlight::highlight_out;
use papk::get_content_by_file;

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

    use crate::cmd_papk;

    #[test]
    fn test_parse_apk_binary() {
        cmd_papk(String::from("_fixtures/apk/app-release-unsigned.apk"));
    }
}
