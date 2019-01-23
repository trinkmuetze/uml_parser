extern crate xmltree;

use self::xmltree::Element;

//-------------------------------------------------------------------------------------------------
//STRUCTS
//-------------------------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct System {
    pub name: String,
    pub akteurs: Vec<Akteur>,
    pub use_cases: Vec<UseCase>,
}

#[derive(Clone, Debug)]
pub struct UseCase {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Akteur {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Relations {
    pub akteur_akteur: Vec<Akteur_Akteur>,
    pub akteur_use_case: Vec<Akteur_UseCase>,
    pub use_case_use_case: Vec<UseCase_UseCase>,
}

#[derive(Clone, Debug)]
pub struct Akteur_Akteur {
    pub akteur: String,
    pub to_akteur: String,
}

#[derive(Clone, Debug)]
pub struct Akteur_UseCase {
    pub akteur: String,
    pub to_use_case: String,
}

#[derive(Clone, Debug)]
pub struct UseCase_UseCase {
    pub relation_type: String,
    pub use_case: String,
    pub to_use_case: String,
}

//-------------------------------------------------------------------------------------------------
//METHODS - PUBLIC
//-------------------------------------------------------------------------------------------------
pub fn get_system(main: Element) -> Vec<System> {
    let mut sys: Vec<System> = Vec::new();

    for mut child in main.children {
        if child.name.to_string() == "system" {
            let mut n = "";
            let mut a: Vec<Akteur>;
            let mut u: Vec<UseCase>;

            for (key, value) in &child.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            a = get_akteurs(child.children.clone());
            u = get_use_cases(child.children.clone());

            sys.push(System{ name: n.to_string(), akteurs: a, use_cases: u});
        }
    }

    return sys;
}

pub fn get_relations(main: Element) -> Vec<Relations> {
    let mut rel: Vec<Relations> = Vec::new();

    for mut child in main.clone().children {
        if child.name.to_string() == "relations" {
            let mut a_a: Vec<Akteur_Akteur>;
            let mut a_u: Vec<Akteur_UseCase>;
            let mut u_u: Vec<UseCase_UseCase>;

            a_a = get_akteur_akteur(child.children.clone(), get_system(main.clone())[0].akteurs.clone());
            a_u = get_akteur_use_case(child.children.clone(), get_system(main.clone())[0].akteurs.clone(), get_system(main.clone())[0].use_cases.clone());
            u_u = get_use_case_use_case(child.children.clone(), get_system(main.clone())[0].use_cases.clone());

            rel.push(Relations{ akteur_akteur: a_a, akteur_use_case: a_u, use_case_use_case: u_u});
        }
    }
    return rel;
}
//-------------------------------------------------------------------------------------------------
//METHODS - PRIVATE
//-------------------------------------------------------------------------------------------------
fn get_akteurs(elements:Vec<Element>) -> Vec<Akteur>{
    //Vektor für alle Akteurs
    let mut akteurs: Vec<Akteur> = Vec::new();

    //Akteurs durchlaufen
    for element in elements {
        if element.name.to_string() == "akteur" {
            let mut n = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            akteurs.push(Akteur{ name: n.to_string()});
        }
    }
    return akteurs;
}

fn get_use_cases(elements:Vec<Element>) -> Vec<UseCase>{
    let mut use_cases: Vec<UseCase> = Vec::new();

    for element in elements {
        if element.name.to_string() == "use_case" {
            let mut n = "";
            let children = element.children;

            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            use_cases.push(UseCase{ name: n.to_string()});
        }
    }
    return use_cases;
}

fn get_akteur_akteur(elements:Vec<Element>, akteurs: Vec<Akteur>) -> Vec<Akteur_Akteur>{
    let mut a_a: Vec<Akteur_Akteur> = Vec::new();

    for element in elements {
        if element.name.to_string() == "akteur_akteur" {
            let mut a = "";
            let mut a2 = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "akteur" || key.to_string() == "to_akteur" {
                    if akteur_exists(akteurs.clone(), value.to_string()) == true {
                        if key.to_string() == "akteur" {
                            a = value;
                        } else if key.to_string() == "to_akteur" {
                            a2 = value;
                        }
                    } else {
                        println!("Akteur \"{}\" doesn't exist!", value);
                        return a_a;
                    }
                }
            }

            a_a.push(Akteur_Akteur{ akteur: a.to_string(), to_akteur: a2.to_string() });
        }
    }
    return a_a;
}

fn get_akteur_use_case(elements:Vec<Element>, akteurs: Vec<Akteur>, use_cases: Vec<UseCase>) -> Vec<Akteur_UseCase>{
    let mut a_u: Vec<Akteur_UseCase> = Vec::new();

    for element in elements {
        if element.name.to_string() == "akteur_use_case" {
            let mut a = "";
            let mut u = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "akteur" {
                    if akteur_exists(akteurs.clone(), value.to_string()) == true {
                        a = value;
                    } else {
                        println!("Akteur \"{}\" doesn't exist!", value);
                        return a_u;
                    }
                } else if key.to_string() == "to_use_case" {
                    if use_case_exists(use_cases.clone(), value.to_string()) == true {
                        u = value;
                    } else {
                        println!("Use Case \"{}\" doesn't exist!", value);
                        return a_u;
                    }
                }
            }

            a_u.push(Akteur_UseCase{ akteur: a.to_string(), to_use_case: u.to_string() });
        }
    }
    return a_u;
}

fn get_use_case_use_case(elements:Vec<Element>, use_cases: Vec<UseCase>) -> Vec<UseCase_UseCase>{
    let mut u_u: Vec<UseCase_UseCase> = Vec::new();

    for element in elements {
        if element.name.to_string() == "use_case_use_case" {
            let mut t = "";
            let mut u = "";
            let mut u2 = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "relation_type" {
                    t = value;
                } else if key.to_string() == "use_case" || key.to_string() == "to_use_case" {
                    if use_case_exists(use_cases.clone(), value.to_string()) == true {
                        if key.to_string() == "use_case" {
                            u = value;
                        } else if key.to_string() == "to_use_case" {
                            u2 = value;
                        }
                    } else {
                        println!("Use Case \"{}\" doesn't exist!", value);
                        return u_u;
                    }
                }
            }

            u_u.push(UseCase_UseCase{ relation_type: t.to_string(), use_case: u.to_string(), to_use_case: u2.to_string() });
        }
    }
    return u_u;
}

fn akteur_exists(akteurs: Vec<Akteur>, akteur_name: String) -> bool {
    for akteur in akteurs {
        if akteur.name == akteur_name {
            return true;
        }
    }
    return false;
}

fn use_case_exists(use_cases: Vec<UseCase>, use_case_name: String) -> bool {
    for use_case in use_cases {
        if use_case.name == use_case_name {
            return true;
        }
    }
    return false;
}
