
extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::env;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

pub fn load_exoml(file_name: String) -> () {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut depth = 0;
    println!("STARTING PARSE!");
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

#[test]
fn test_file_exoml() {
    println!("Beginning test...");
    load_exoml(String::from("test_file.exoml"));
}
