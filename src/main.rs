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
                let data = parser::parse_data(filename.to_string());
                let models = parser::package::get_models(data.clone());

                println!("{:#?}", models);
            }

        //CLASS DIAGRAM----------------------------------------------------------------------------
        } else if uml_type == "uml_class" {
            valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                let data = parser::parse_data(filename.to_string());
                let packages = parser::class::get_packages(data.clone());
                let relationships = parser::class::get_relationships(data.clone());

                for package in packages {
                    if diagramVisualizer::generate_class_diagram(relationships.clone(), package.classes, 720, 1280, "Test") {
                        println!("Diagram created!");
                    }
                }
            }
        //OBJECT DIAGRAM---------------------------------------------------------------------------
        } else if uml_type == "uml_object" {
            valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                let data = parser::parse_data(filename.to_string());
                let objects = parser::object::get_objects(data.clone());
                let links = parser::object::get_links(data.clone());

                println!("{:#?}", objects);
                println!("{:#?}", links);
            }
        //USE CASE DIAGRAM-------------------------------------------------------------------------
        } else if uml_type == "uml_use_case" {
            valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                let data = parser::parse_data(filename.to_string());
                let system = parser::use_case::get_system(data.clone());
                let relations = parser::use_case::get_relations(data.clone());

                println!("{:#?}", system);
                println!("{:#?}", relations);
            }
        //DEPLOYMENT DIAGRAM-----------------------------------------------------------------------
        } else if uml_type == "uml_deployment" {
            valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                println!("XML valid but not implemented.");
            }
        //COMPONENT DIAGRAM-----------------------------------------------------------------------
    } else if uml_type == "uml_component" {
            valid = parser::validate_xml(uml_type, filename.to_string());

            if valid {
                println!("XML valid but not implemented.");
            }
        } else {
            println!("XML not found or not valid.");
        }
    }
    else if args.len() < 2 {
        println!("Too few parameters.");
    }
    else if args.len() > 2 {
        println!("Too much parameters.");
    }

}
