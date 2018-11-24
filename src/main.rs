pub mod diagramVisualizer;
pub mod parser;

fn main() {
    let valid = parser::validate_xml("BeispielXML.xml".to_string());

    if valid {
        let data = parser::parse_data("BeispielXML.xml".to_string());
        let packages = parser::get_packages(data.clone());

        for package in packages {
            if diagramVisualizer::generateDiagram(package.classes, 720, 1280, "Test") {
                println!("Diagramm erfolgreich erstellt!");
            }
        }
    } else {
        println!("XML not valid");
    }

    /*//Example class
    let mut attribute1 = String::from("- size");
    let mut attribute2 = String::from("+ name");
    let mut method1 = String::from("+ bark()");
    let mut method2 = String::from("+ eat()");
    let mut method3 = String::from("+ fly()");

    let mut attributes = Vec::new();
    attributes.push(attribute1);
    attributes.push(attribute2);
    let mut methods = Vec::new();
    methods.push(method1);
    methods.push(method2);
    methods.push(method3);
    let mut classes = Vec::new();
    let class = Class{name: "OneClass".to_string(), attributes: attributes, methods: methods};

    let mut attribute3 = String::from("- size");
    let mut attribute4 = String::from("+ name");
    let mut method4 = String::from("+ bark()");
    let mut method5 = String::from("+ eat()");
    let mut method6 = String::from("+ fly()");

    let mut attributes2 = Vec::new();
    attributes2.push(attribute3);
    attributes2.push(attribute4);
    let mut methods2 = Vec::new();
    methods2.push(method4);
    methods2.push(method5);
    methods2.push(method6);
    let class2 = Class{name: "TwoClass".to_string(), attributes: attributes2, methods: methods2};
    classes.push(class);
    classes.push(class2);

    generateDiagram(classes,720,1280, "My First Diagram!");*/
}
