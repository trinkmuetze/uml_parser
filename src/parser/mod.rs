extern crate xmltree;

pub mod class;
pub mod package;
pub mod object;
pub mod use_case;
//mod uml_class;
//pub use self::uml_class::{Class, Attribute, Method, Parameter, Relationship, Package, RelationshipClass, get_packages, get_relationships};

use std::process::Command;
use self::xmltree::Element;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

pub fn validate_xml(uml_type: String, file_name: String) -> bool {
    let mut xsd_file = "";
    if uml_type == "uml_class" {
        xsd_file = "xsd/UML_Class_Schema.xsd";
    } else if uml_type == "uml_package" {
        xsd_file = "xsd/UML_Package_Schema.xsd";
    } else if uml_type == "uml_object" {
        xsd_file = "xsd/UML_Object_Schema.xsd";
    } else if uml_type == "uml_use_case" {
        xsd_file = "xsd/UML_Use_Case_Schema.xsd";
    }

    let mut command = Command::new("sh");
    command.arg("-c")
           .arg("xmllint --noout --schema ".to_string() + xsd_file + " " + &file_name);
    let output = command.output().expect("failed to execute process");

    let xml_output = String::from_utf8_lossy(&output.stderr);

    if xml_output == file_name + " validates\n" {
        return true;
    } else {
        println!("{}", xml_output);
        return false;
    }
}

pub fn get_uml_type(file_name: String) -> String {
    return parse_data(file_name).name;;
}

fn get_file_data(file_name: String) -> String {
    let file = File::open(file_name).unwrap();
    let mut file = BufReader::new(file);

    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    return String::from_utf8(contents).unwrap();
}

pub fn parse_data(file_name: String) -> Element {
    let data = get_file_data(file_name);
    return Element::parse(data.as_bytes()).unwrap();
}
