extern crate xmltree;

use self::xmltree::Element;

//-------------------------------------------------------------------------------------------------
//STRUCTS
//-------------------------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Model {
    pub name: String,
    pub packages: Vec<Package>,
}

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub relation: String,
    pub to_package: String,
    pub packages: Vec<Package>,
}

//-------------------------------------------------------------------------------------------------
//METHODS - PUBLIC
//-------------------------------------------------------------------------------------------------
pub fn get_models(main: Element) -> Vec<Model> {
    //Vektor für alle Models
    let mut models: Vec<Model> = Vec::new();

    //Packages durchlaufen
    for mut child in main.children {
        if child.name.to_string() == "model" {
            let mut n = "";
            let mut p: Vec<Package>;

            //Attribute des Elements durchlaufen
            for (key, value) in &child.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            //Vektor von Packages holen
            p = get_packages(child.children.clone());

            //Packages zum Vektor hinzufügen
            models.push(Model{ name: n.to_string(), packages: p});
        }
    }
    return models;
}

//-------------------------------------------------------------------------------------------------
//METHODS - PRIVATE
//-------------------------------------------------------------------------------------------------
fn get_packages(elements:Vec<Element>) -> Vec<Package>{
    //Vektor für alle Packages
    let mut packages: Vec<Package> = Vec::new();

    //Packages durchlaufen
    for element in elements {
        if element.name.to_string() == "package" {
            let mut n = "";
            let mut r = "";
            let mut tp = "";
            let mut p: Vec<Package>;

            //Attribute des Elements durchlaufen
            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                } else if key.to_string() == "relation" {
                    r = value;
                } else if key.to_string() == "toPackage" {
                    tp = value;
                }
            }

            //Vektor von Klassen holen
            p = get_packages(element.children.clone());

            //Klassen zum Vektor hinzufügen
            packages.push(Package{ name: n.to_string(), relation: r.to_string(), to_package: tp.to_string(), packages: p});
        }
    }
    return packages;
}
