use std::env;

pub mod diagramVisualizer;
pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut valid = true;

    if args.len() == 2  {
        let filename = &args[1];
        println!("{:?}", filename);
        let uml_type = parser::get_uml_type(filename.clone());

        //PACKAGE DIAGRAM--------------------------------------------------------------------------
        if uml_type == "uml_package" {
            //valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                let data = parser::parse_data(filename.to_string());
                let modules = parser::package::get_models(data.clone());

                if diagramVisualizer::generate_package_diagram(modules.clone(), 720, 1280) {
                    println!("Diagram created!");
                }
            }

        //CLASS DIAGRAM----------------------------------------------------------------------------
        } else if uml_type == "uml_class" {
            //valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                let data = parser::parse_data(filename.to_string());
                let packages = parser::class::get_packages(data.clone());
                let relationships = parser::class::get_relationships(data.clone());

                for package in packages {
                    if diagramVisualizer::generate_class_diagram(relationships.clone(), package.classes, 720, 1280, "Test") {
                        println!("Diagram created!");
                    }
                }

            } else {
                println!("XML not valid or not found.");
            }
        //OBJECT DIAGRAM----------------------------------------------------------------------------
        } else if uml_type == "uml_object" {
            //valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                let data = parser::parse_data(filename.to_string());
                let objects = parser::object::get_objects(data.clone());
                let links = parser::object::get_links(data.clone());

                if diagramVisualizer::generate_object_diagram(links.clone(), objects.clone(), 720, 1280, "Test") {
                    println!("Diagram created!");
                }
            }
        } else {
            println!("Not a valid XML file.");
        }
    }
    else if args.len() < 2 {
        println!("Too few parameters.");
    }
}
