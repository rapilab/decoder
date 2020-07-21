mod highlight;

use clap::{Arg, App};
use papk::get_content_by_file;
use crate::highlight::highlight_out;
use pclass::parse_class;

fn main() {
    let app = App::new("decoder")
        .version("0.0.1");

    let papk_opt = Arg::with_name("papk")
        .long("papk")
        .takes_value(true);


    let app = app.arg(papk_opt);
    let matches = app.get_matches();

    let papk = matches.value_of("papk");
    match papk {
        None => {}
        Some(str) => {
            cmd_papk(String::from(str))
        }
    }

    parse_class(b"");
}

pub fn p_class() {
    parse_class(b"");
}

pub fn cmd_papk(str: String) {
    let result = get_content_by_file(str, String::from("AndroidManifest.xml"));
    if let Ok(str) = result {
        highlight_out(str.as_str(), "xml");
    }
}