extern crate xml;

use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use std::process::Command;

fn get_classes(fileName: String) -> Vec<Class> {
    let file = File::open(&fileName).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut classes: Vec<Class> = Vec::new();

    let class_name;
    let mut attribs: Vec<Attribute> = Vec::new();
    let mut methods: Vec<Method> = Vec::new();

    for e in parser {
        match e {
            //XML Element
            Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                if name == "class" {
                    for attribute in attributes {
                        class_name = attribute.to_string();
                    }
                }
                else if name == "attribute" {
                    let name;
                    let visibility;
                    let data_type;
                    for attribute in attributes {
                        name = "".to_string();
                        visibility = "".to_string();
                        data_type = "".to_string();
                    }
                    let attrib = Attribute {
                        name: name,
                        visibility: visibility,
                        data_type: data_type
                    };
                    attribs.push(attrib);
                }
                else if name == "method" {
                    let name;
                    let visibility;
                    let return_type;
                    let parameters;
                    for attribute in attributes {
                        name = "".to_string();
                        visibility = "".to_string();
                        return_type = "".to_string();
                        parameters = "".to_string();
                    }
                    let method = Method {
                        name: name,
                        visibility: visibility,
                        return_type: return_type,
                        parameters: parameters
                    };
                    methods.push(method);
                }
                println!("{}", name );
                let mut attribs: Vec<Attribute> = Vec::new();
                let mut methods: Vec<Method> = Vec::new();

                //Attribute des XML Elements
                for attribute in attributes {
                    println!("{}", attribute );
                    let attrib = Attribute { name: attribute.to_string(), visibility: "".to_string(), data_type: "".to_string() };
                    attribs.push(attrib);

                    //Methode
                    let method = Method { name: attribute.to_string(), visibility: "".to_string(), return_type: "".to_string(), parameters: "".to_string() };
                    methods.push(method);
                }
                let c = Class{name: name.to_string(), attributes: attribs, methods: methods};
                classes.push(c);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    return classes;
}

//--------------------------------------------------------------------------------------

fn print_class(c: &Class) {
    println!("Name: {}", &c.name);
    for attrib in &c.attributes {
        println!("{}{}", indent(1), attrib.name);
    }
    for method in &c.methods {
        println!("{}{}", indent(1), method.name);
    }
}
