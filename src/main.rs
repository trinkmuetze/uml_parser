use std::env;

pub mod diagramVisualizer;
pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut valid = false;

    if args.len() == 2  {
        let filename = &args[1];
        valid = parser::validate_xml(filename.to_string());

        if valid {
            let data = parser::parse_data(filename.to_string());
            let packages = parser::get_packages(data.clone());

            for package in packages {
                if diagramVisualizer::generateDiagram(package.classes, 720, 1280, "Test") {
                    println!("Diagram created!");
                }
            }
        } else {
            println!("XML not valid or not found.");
        }
    }
    else if args.len() < 2 {
        println!("Too few parameters.");
    }
    else if args.len() > 2 {
        println!("Too much parameters.");
    }

}
