use clap::{Arg, App};
use papk::get_content_by_file;

fn main() {
    let app = App::new("decoder")
        .version("0.0.1");

    let papk_opt = Arg::with_name("papk").long("papk");

    let app = app.arg(papk_opt);
    let matches = app.get_matches();

    let papk = matches.value_of("papk");
    match papk {
        None => {}
        Some(str) => {
            cmd_papk(String::from(str))
        }
    }
}

pub fn cmd_papk(str: String) {
    let result = get_content_by_file(str, String::from("AndroidManifest.xml"));
    if let Ok(str) = result {
        println!("{:?}", str)
    }
}