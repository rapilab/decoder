extern crate abxml;
extern crate zip;
#[macro_use]
extern crate failure;
extern crate log;

use abxml::encoder::Xml;
use abxml::visitor::{Executor, ModelVisitor, Resources, XmlVisitor};
use failure::{bail, Error, ResultExt};
use std::io::{Cursor, Read};

#[derive(Debug, Clone)]
pub enum ApkType {
    Xml { content: String },
    Class { content: Vec<u8> },
}

pub fn get_classes_dex(apk_path: String) -> Result<Vec<u8>, Error> {
    let mut class_dex: Vec<u8> = vec![];

    let file = std::fs::File::open(&apk_path)?;
    let mut archive = zip::ZipArchive::new(file).unwrap();
    archive
        .by_name("classes.dex")
        .unwrap()
        .read_to_end(&mut class_dex)?;

    if !class_dex.is_empty() {
        Ok(class_dex)
    } else {
        bail!("could not find classes.dex")
    }
}

pub fn get_resources(apk_path: String) -> Result<String, Error> {
    let android_resources_content = abxml::STR_ARSC.to_owned();

    let file = std::fs::File::open(&apk_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    let mut resources_content = Vec::new();
    archive
        .by_name("resources.arsc")
        .unwrap()
        .read_to_end(&mut resources_content)
        .unwrap();
}

pub fn get_content_by_file(apk_path: String, target_file: String) -> Result<String, Error> {
    let android_resources_content = abxml::STR_ARSC.to_owned();

    let file = std::fs::File::open(&apk_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    let mut resources_content = Vec::new();
    archive
        .by_name("resources.arsc")
        .unwrap()
        .read_to_end(&mut resources_content)
        .unwrap();

    let mut resources_visitor = ModelVisitor::default();
    Executor::arsc(&resources_content, &mut resources_visitor)?;
    Executor::arsc(&android_resources_content, &mut resources_visitor)?;

    for i in 0..archive.len() {
        let mut current_file = archive.by_index(i).unwrap();

        if current_file.name().contains(&target_file) {
            {
                let mut xml_content = Vec::new();
                current_file.read_to_end(&mut xml_content)?;
                let new_content = xml_content;

                let resources = resources_visitor.get_resources();
                let out =
                    parse_xml(&new_content, resources).context("could not decode target file")?;
                return Ok(out);
            }
        }
    }

    bail!("could not find results")
}

fn parse_xml<'a>(content: &[u8], resources: &'a Resources<'a>) -> Result<String, Error> {
    let cursor = Cursor::new(content);
    let mut visitor = XmlVisitor::new(resources);

    Executor::xml(cursor, &mut visitor)?;

    match *visitor.get_root() {
        Some(ref root) => match *visitor.get_string_table() {
            Some(_) => {
                let res =
                    Xml::encode(visitor.get_namespaces(), root).context("could note encode XML")?;
                return Ok(res);
            }
            None => {
                println!("No string table found");
            }
        },
        None => {
            println!("No root on target XML");
        }
    }

    bail!("could not decode XML")
}

#[cfg(test)]
mod tests {
    use crate::{get_classes_dex, get_content_by_file};

    #[test]
    fn test_parse_zip_file() {
        let result = get_content_by_file(
            String::from("../_fixtures/apk/app-release-unsigned.apk"),
            String::from("AndroidManifest.xml"),
        );
        if let Ok(str) = result {
            assert_eq!(true, str.contains("com.phodal.myapplication"));
        }
    }

    #[test]
    fn test_get_classes_dex() {
        let result = get_classes_dex(String::from("../_fixtures/apk/app-release-unsigned.apk"));
        if let Ok(bytes) = result {
            assert_eq!(true, bytes.len() > 0);
        }
    }
}
