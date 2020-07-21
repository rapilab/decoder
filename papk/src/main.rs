use std::fs;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
}

pub fn parse_apk(fname: String) {
    let file = fs::File::open(&fname).unwrap();
    let reader = BufReader::new(file);

    let mut archive = zip::ZipArchive::new(reader).unwrap();
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("Entry {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!(
                "Entry {} is a directory with name \"{}\"",
                i,
                outpath.as_path().display()
            );
        } else {
            println!(
                "Entry {} is a file with name \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parse_apk;

    #[test]
    fn test_parse_apk_binary() {
        parse_apk(String::from("../_fixtures/apk/app-release-unsigned.apk"));
    }
}
