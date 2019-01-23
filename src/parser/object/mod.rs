extern crate xmltree;

use self::xmltree::Element;

//-------------------------------------------------------------------------------------------------
//STRUCTS
//-------------------------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Object {
    pub name: String,
    pub class: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub attrib_type: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct Link {
    pub name: String,
    pub object: LinkObject,
    pub to_object: LinkObject,
}

#[derive(Clone, Debug)]
pub struct LinkObject {
    pub name: String,
    pub role: String,
}

//-------------------------------------------------------------------------------------------------
//METHODS - PUBLIC
//-------------------------------------------------------------------------------------------------
pub fn get_objects(main: Element) -> Vec<Object> {
    //Vektor für alle Objects
    let mut objects: Vec<Object> = Vec::new();

    //Objects durchlaufen
    for mut child in main.children {
        if child.name.to_string() == "object" {
            let mut n = "";
            let mut c = "";
            let mut a: Vec<Attribute>;

            //Attribute des Elements durchlaufen
            for (key, value) in &child.attributes {
                if key.to_string() == "name" {
                    n = value;
                } else if key.to_string() == "class" {
                    c = value;
                }
            }

            //Vektor von Attributes holen
            a = get_attributes(child.children.clone());

            //Packages zum Vektor hinzufügen
            objects.push(Object{ name: n.to_string(), class: c.to_string(), attributes: a});
        }
    }
    return objects;
}

pub fn get_links(main: Element) -> Vec<Link> {
    //Vektor für alle Links
    let mut links: Vec<Link> = Vec::new();
    let objects = get_objects(main.clone());

    //Links durchlaufen
    for mut child in main.children {
        if child.name.to_string() == "link" {
            let mut n = "";

            //Attribute des Elements durchlaufen
            for (key, value) in &child.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            let mut o1 : LinkObject;
            let mut o2 : LinkObject;
            let mut n1 = "".to_string();
            let mut n2 = "".to_string();
            let mut r1 = "".to_string();
            let mut r2 = "".to_string();
            //Objekte der Beziehung durchlaufen
            for obj in child.children {
                for (key, value) in obj.attributes {
                    if key.to_string() == "name" {
                        if object_exists(objects.clone(), value.to_string()) == true {
                            if obj.name == "object" {
                                n1 = value;
                            }
                            else if obj.name == "toObject" {
                                n2 = value;
                            }
                        } else {
                            println!("Object \"{}\" doesn't exist!", value);
                            return links;
                        }
                    }
                    else if key.to_string() == "role" {
                        if obj.name == "object" {
                            r1 = value;
                        }
                        else if obj.name == "toObject" {
                            r2 = value;
                        }
                    }
                }

            }
            o1 = LinkObject{ name: n1.to_string(), role: r1.to_string() };
            o2 = LinkObject{ name: n2.to_string(), role: r2.to_string() };

            //Links zum Vektor hinzufügen
            links.push(Link{ name: n.to_string(), object: o1, to_object: o2});
        }
    }
    return links;
}

//-------------------------------------------------------------------------------------------------
//METHODS - PRIVATE
//-------------------------------------------------------------------------------------------------
fn get_attributes(elements:Vec<Element>) -> Vec<Attribute>{
    //Vektor für alle Attribute
    let mut attributes: Vec<Attribute> = Vec::new();

    //Attribute durchlaufen
    for element in elements {
        if element.name.to_string() == "attribute" {
            let mut n = "";
            let mut a = "";
            let mut v = "";

            //Attribute des Elements durchlaufen
            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                } else if key.to_string() == "type" {
                    a = value;
                } else if key.to_string() == "value" {
                    v = value;
                }
            }

            //Klassen zum Vektor hinzufügen
            attributes.push(Attribute{ name: n.to_string(), attrib_type: a.to_string(), value: v.to_string()});
        }
    }
    return attributes;
}

fn object_exists(objects: Vec<Object>, object_name: String) -> bool {
    for object in objects {
        if object.name == object_name || object.class == object_name {
            return true;
        }
    }
    return false;
}
