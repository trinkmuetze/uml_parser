extern crate xmltree;

mod uml_class;
pub use self::uml_class::{Class, Attribute, Method, Parameter, Relationship, Package, RelationshipClass};

use std::process::Command;
use self::xmltree::Element;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

pub fn validate_xml(uml_type: String, file_name: String) -> bool {
    let mut xsd_file = "";
    if uml_type == "class" {
        xsd_file = "xsd/UML_Class_Schema.xsd";
    } else if uml_type == "package" {
        xsd_file = "xsd/UML_Package_Schema.xsd";
    }

    let mut command = Command::new("sh");
    command.arg("-c")
           .arg("xmllint --schema ".to_string() + xsd_file + " " + &file_name);
    let output = command.output().expect("failed to execute process");
    let xml_output = String::from_utf8_lossy(&output.stdout);
    println!("{:?}", xml_output);

    if xml_output != "" {
        return true;
    } else {
        println!("{}", xml_output );
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

pub fn get_packages(main: Element) -> Vec<Package>{
    //Vektor für alle Packages
    let mut packages: Vec<Package> = Vec::new();

    //Packages durchlaufen
    for mut child in main.children {
        if child.name.to_string() == "package" {
            let mut n = "";
            let mut c: Vec<Class>;

            //Attribute des Elements durchlaufen
            for (key, value) in &child.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            //Vektor von Klassen holen
            c = get_classes(child.children.clone());

            //Klassen zum Vektor hinzufügen
            packages.push(Package{ name: n.to_string(), classes: c});
        }
    }
    return packages;
}

pub fn get_classes(elements:Vec<Element>) -> Vec<Class>{
    //Vektor für alle Klassen
    let mut classes: Vec<Class> = Vec::new();

    //Klassen durchlaufen
    for element in elements {
        let mut n = "";
        let mut a: Vec<Attribute>;
        let mut m: Vec<Method>;

        //Attribute des Elements durchlaufen
        if element.name.to_string() == "class" {
            //Attribute des Elements durchlaufen
            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }
            //Vektoren von Attributen und Methoden holen
            a = get_attributes(element.children.clone());
            m = get_methodes(element.children.clone());

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

pub fn get_relationships(main: Element) -> Vec<Relationship> {
    //Vektor für alle Beziehungen
    let mut relationships: Vec<Relationship> = Vec::new();
    let classes = get_all_classes(get_packages(main.clone()));

    //Beziehungen durchlaufen
    for mut child in main.children {
        if child.name.to_string() == "relationship" {
            let mut t = "";

            //Attribute des Elements durchlaufen
            for (key, value) in &child.attributes {
                if key.to_string() == "type" {
                    t = value;
                }
            }

            let mut c1;
            let mut c2;
            let mut n1 = "".to_string();
            let mut n2 = "".to_string();
            let mut m1 = "".to_string();
            let mut m2 = "".to_string();
            //Klassen der Beziehung durchlaufen
            for class in child.children {
                for (key, value) in class.attributes {
                    if key.to_string() == "name" {
                        if class_exists(classes.clone(), value.to_string()) == true {
                            if class.name == "class" {
                                n1 = value;
                            }
                            else if class.name == "toClass" {
                                n2 = value;
                            }
                        } else {
                            println!("Class {} doesn't exist!", value);
                            return relationships;
                        }
                    }
                    else if key.to_string() == "multiplicity" {
                        if class.name == "class" {
                            m1 = value;
                        }
                        else if class.name == "toClass" {
                            m2 = value;
                        }
                    }
                }

            }
            c1 = RelationshipClass{ name: n1.to_string(), multiplicity: m1.to_string() };
            c2 = RelationshipClass{ name: n2.to_string(), multiplicity: m2.to_string() };

            //Beziehungen zum Vektor hinzufügen
            relationships.push(Relationship{ relation_type: t.to_string(), class: c1, to_class: c2});
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

fn get_all_classes(packages: Vec<Package>) -> Vec<Class> {
    //Vektor für alle Klassen
    let mut classes: Vec<Class> = Vec::new();

    for package in packages {
        for class in package.classes {
            classes.push(class);
        }
    }

    return classes;
}
