extern crate xmltree;

mod uml_class;
pub use self::uml_class::{Class, Attribute, Method, Parameter, Relationship};

use std::process::Command;
use self::xmltree::Element;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

pub fn validate_xml() -> bool {
    let output = if cfg!(target_os = "windows") {
        //TODO: Windows implementation
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("xmllint BeispielXML.xml")
                .output()
                .expect("failed to execute process")
    };
    let xml_output = String::from_utf8_lossy(&output.stdout);

    if xml_output != "" {
        return true;
    } else {
        return false;
    }
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

pub fn get_classes(main: Element) -> Vec<Class>{
    //Vektor für alle Klassen
    let mut classes: Vec<Class> = Vec::new();

    //Klassen durchlaufen
    for mut child in main.children {
        if child.name.to_string() == "class" {
            let mut n = "";
            let mut a: Vec<Attribute>;
            let mut m: Vec<Method>;

            //Attribute des Elements durchlaufen
            for (key, value) in &child.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            //Vektoren von Attributen und Methoden holen
            a = get_attributes(child.children.clone());
            m = get_methodes(child.children.clone());

            //Klassen zum Vektor hinzufügen
            classes.push(Class{ name: n.to_string(), attributes: a, methods: m});
        }
    }
    return classes;
}

fn get_attributes(elements:Vec<Element>) -> Vec<Attribute> {
    //Vektor für alle Attribute
    let mut attribs: Vec<Attribute> = Vec::new();

    //Attribute durchlaufen
    for element in elements {
        let mut n = "";
        let mut v = "";
        let mut d = "";

        //Attribute des Elements durchlaufen
        if element.name.to_string() == "attribute" {
            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
                else if key.to_string() == "visibility" {
                    v = value;
                }
                else if key.to_string() == "data-type" {
                    d = value;
                }
            }
            //Attribut zum Vektor hinzufügen
            attribs.push(Attribute{ name: n.to_string(), visibility: v.to_string(), data_type: d.to_string()});
        }
    }
    return attribs;
}

fn get_methodes(elements: Vec<Element>) -> Vec<Method> {
    //Vektor für alle Methoden
    let mut methods: Vec<Method> = Vec::new();

    //Methoden durchlaufen
    for element in elements {
        let mut n = "";
        let mut v = "";
        let mut r = "";
        let mut p: Vec<Parameter>;

        //Attribute des Elements durchlaufen
        if element.name.to_string() == "method" {
            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
                else if key.to_string() == "visibility" {
                    v = value;
                }
                else if key.to_string() == "return-type" {
                    r = value;
                }
            }
            //Vektor von Parametern holen
            p = get_parameters(element.children.clone());

            //Methode zum Vektor hinzufügen
            methods.push(Method{ name: n.to_string(), visibility: v.to_string(), return_type: r.to_string(), parameters: p});
        }
    }
    return methods;
}

fn get_parameters(elements: Vec<Element>) -> Vec<Parameter> {
    //Vektor für alle Parameter
    let mut parameters: Vec<Parameter> = Vec::new();

    //Parameter durchlaufen
    for element in elements {
        let mut n = "";
        let mut d = "";

        //Attribute des Elements durchlaufen
        if element.name.to_string() == "parameter" {
            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
                else if key.to_string() == "data-type" {
                    d = value;
                }
            }
            //Paramter zum Vektor hinzufügen
            parameters.push(Parameter{ name: n.to_string(), data_type: d.to_string()});
        }
    }
    return parameters;
}

pub fn get_relationships(main: Element, classes: Vec<Class>) -> Vec<Relationship> {
    //Vektor für alle Beziehungen
    let mut relationships: Vec<Relationship> = Vec::new();

    //Beziehungen durchlaufen
    for mut child in main.children {
        if child.name.to_string() == "relationship" {
            let mut t = "";
            let mut c1 = "";
            let mut c2 = "";

            //Attribute des Elements durchlaufen
            for (key, value) in &child.attributes {
                if key.to_string() == "type" {
                    t = value;
                }
                else if key.to_string() == "class" {
                    if class_exists(classes.clone(), value.to_string()) == true {
                        c1 = value;
                    }
                    else {
                        println!("Class {} doesn't exist!", value.to_string());
                        return relationships;
                    }
                }
                else if key.to_string() == "toClass" {
                    if class_exists(classes.clone(), value.to_string()) == true {
                        c2 = value;
                    }
                    else {
                        println!("Class {} doesn't exist!", value.to_string());
                        return relationships;
                    }
                }
            }

            //Beziehungen zum Vektor hinzufügen
            relationships.push(Relationship{ relation_type: t.to_string(), class: c1.to_string(), to_class: c2.to_string()});
        }
    }
    return relationships;
}

fn class_exists(classes: Vec<Class>, class_name: String) -> bool {
    for class in classes {
        if class.name == class_name {
            return true;
        }
    }
    return false;
}
