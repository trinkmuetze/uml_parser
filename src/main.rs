use std::env;

pub mod diagramVisualizer;
pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut valid = true;

<<<<<<< HEAD
    if args.len() == 3  {
        let uml_type = &args[1];
        let filename = &args[2];

        //PACKAGE DIAGRAM--------------------------------------------------------------------------
        if uml_type == "package" {
            valid = parser::validate_xml(uml_type.to_string(), filename.to_string());

            if valid {
                println!("XML valid, but package diagrams not implemented.");
            } else {
                println!("XML not valid or not found.");
            }

        //CLASS DIAGRAM----------------------------------------------------------------------------
        } else if uml_type == "class" {
            valid = parser::validate_xml(uml_type.to_string(), filename.to_string());

            if valid {
                let data = parser::parse_data(filename.to_string());
                let packages = parser::get_packages(data.clone());

                for package in packages {
                    if diagramVisualizer::generateDiagram(package.classes, 720, 1280, "Test") {
                        println!("Diagram created!");
                    }
=======
    if args.len() == 2  {
        let filename = &args[1];
        //valid = parser::validate_xml(filename.to_string());

        if valid {
            let data = parser::parse_data(filename.to_string());
            let packages = parser::get_packages(data.clone());
            let relationships = parser::get_relationships(data.clone());

            for package in packages {
                if diagramVisualizer::generateDiagram(relationships.clone(), package.classes, 720, 1280, "Test") {
                    println!("Diagram created!");
>>>>>>> 733fef19f1f2bad9e3753aa37f939aa012127dd5
                }
            } else {
                println!("XML not valid or not found.");
            }
        }
    }
    else if args.len() < 2 {
        println!("Too few parameters.");
    }
    else if args.len() > 2 {
        println!("Too much parameters.");
    }

}
