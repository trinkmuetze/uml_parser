extern crate xmltree;

use self::xmltree::Element;

//-------------------------------------------------------------------------------------------------
//STRUCTS
//-------------------------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Component {
    pub provided_interface: Vec<ProvidedInterface>,
    pub required_interface: Vec<RequiredInterface>,
    pub realization: Vec<Realization>,
    pub artifact: Vec<Artifact>
}

#[derive(Clone, Debug)]
pub struct ProvidedInterface {
    pub name: String
}

#[derive(Clone, Debug)]
pub struct RequiredInterface {
    pub name: String
}

#[derive(Clone, Debug)]
pub struct Realization {
    pub name: String
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub name: String
}

//-------------------------------------------------------------------------------------------------
//METHODS - PUBLIC
//-------------------------------------------------------------------------------------------------
pub fn get_components(main: Element) -> Vec<Component> {
    let mut components: Vec<Component> = Vec::new();

    for mut child in main.children {
        if child.name.to_string() == "component" {
            let mut pi: Vec<ProvidedInterface> = Vec::new();
            let mut ri: Vec<RequiredInterface> = Vec::new();
            let mut r: Vec<Realization> = Vec::new();
            let mut a: Vec<Artifact> = Vec::new();

            pi = get_provided_interfaces(child.children.clone());
            ri = get_required_interfaces(child.children.clone());
            r = get_realizations(child.children.clone());
            a = get_artifacts(child.children.clone());

            components.push(Component{ provided_interface: pi, required_interface: ri, realization: r, artifact: a});
        }
    }

    return components;
}

//-------------------------------------------------------------------------------------------------
//METHODS - PRIVATE
//-------------------------------------------------------------------------------------------------
pub fn get_provided_interfaces(elements: Vec<Element>) -> Vec<ProvidedInterface> {
    let mut pi: Vec<ProvidedInterface> = Vec::new();

    for element in elements {
        if element.name.to_string() == "provided_interface" {
            let mut n = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            pi.push(ProvidedInterface{name: n.to_string()});

        }
    }

    return pi;
}

pub fn get_required_interfaces(elements: Vec<Element>) -> Vec<RequiredInterface> {
    let mut ri: Vec<RequiredInterface> = Vec::new();

    for element in elements {
        if element.name.to_string() == "required_interface" {
            let mut n = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            ri.push(RequiredInterface{name: n.to_string()});

        }
    }

    return ri;
}

pub fn get_realizations(elements: Vec<Element>) -> Vec<Realization> {
    let mut r: Vec<Realization> = Vec::new();

    for element in elements {
        if element.name.to_string() == "realization" {
            let mut n = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            r.push(Realization{name: n.to_string()});

        }
    }

    return r;
}

pub fn get_artifacts(elements: Vec<Element>) -> Vec<Artifact> {
    let mut a: Vec<Artifact> = Vec::new();

    for element in elements {
        if element.name.to_string() == "artifact" {
            let mut n = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            a.push(Artifact{name: n.to_string()});

        }
    }

    return a;
}
