use std::env;

pub mod diagramVisualizer;
pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut valid = true;

    if args.len() == 2  {
        let filename = &args[1];
        let uml_type = parser::get_uml_type(filename.clone());

        //PACKAGE DIAGRAM--------------------------------------------------------------------------
        if uml_type == "uml_package" {
            valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                println!("XML valid, but package diagrams not implemented.");
            } else {
                println!("XML not valid or not found.");
            }

        //CLASS DIAGRAM----------------------------------------------------------------------------
    } else if uml_type == "uml_class" {
            valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                let data = parser::parse_data(filename.to_string());
                let packages = parser::get_packages(data.clone());
                let relationships = parser::get_relationships(data.clone());

                for package in packages {
                    if diagramVisualizer::generateDiagram(relationships.clone(), package.classes, 720, 1280, "Test") {
                        println!("Diagram created!");
                    }
                }
            }
        } else {
            println!("Not a valid XML file.");
        }
    }
    else if args.len() < 2 {
        println!("Too few parameters.");
    }
    else if args.len() > 2 {
        println!("Too much parameters.");
    }

}
