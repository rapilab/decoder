mod highlight;

use clap::{Arg, App};
use papk::{get_content_by_file, get_class_dex};
use crate::highlight::highlight_out;
use pclass::parse_class;

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

pub fn p_class(str: String) {
    let result = get_class_dex(String::from(str));
    if let Ok(bytes) = result {
        // println!("{:?}", bytes)
    }
}

pub fn cmd_papk(str: String) {
    let result = get_content_by_file(str, String::from("AndroidManifest.xml"));
    if let Ok(str) = result {
        highlight_out(str.as_str(), "xml");
    }
}