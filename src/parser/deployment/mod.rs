extern crate xmltree;

use self::xmltree::Element;

//-------------------------------------------------------------------------------------------------
//STRUCTS
//-------------------------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Node {
    pub node_type: String,
    pub name: String,
    pub nodes: Vec<Node>,
    pub artifacts: Vec<Artifact>
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub artifact_type: String,
    pub name: String
}

#[derive(Clone, Debug)]
pub struct Relations {
    pub node_node: Vec<Node_Node>,
    pub artifact_node: Vec<Artifact_Node>
}

#[derive(Clone, Debug)]
pub struct Node_Node {
    pub name: String,
    pub node: RelationNode,
    pub to_node: RelationNode
}

#[derive(Clone, Debug)]
pub struct RelationNode {
    pub name: String,
    pub multiplicity: String
}

#[derive(Clone, Debug)]
pub struct Artifact_Node {
    pub artifact: String,
    pub to_node: String
}

//-------------------------------------------------------------------------------------------------
//METHODS - PUBLIC
//-------------------------------------------------------------------------------------------------
pub fn get_nodes(elements:Vec<Element>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    for element in elements {
        if element.name.to_string() == "node" {
            let mut n = "";
            let mut t = "";
            let mut no: Vec<Node>;
            let mut ar: Vec<Artifact>;

            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                } else if key.to_string() == "node_type" {
                    t = value;
                }
            }

            no = get_nodes(element.children.clone());
            ar = get_artifacts(element.children.clone());

            nodes.push(Node{node_type: t.to_string() ,name: n.to_string(), nodes: no, artifacts: ar});
        }
    }

    return nodes;
}

pub fn get_artifacts(elements:Vec<Element>) -> Vec<Artifact> {
    let mut artifacts: Vec<Artifact> = Vec::new();

    for element in elements {
        if element.name.to_string() == "artifact" {
            let mut n = "";
            let mut t = "";

            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                } else if key.to_string() == "artifact_type" {
                    t = value;
                }
            }
            if t == "" {
                t = "artifact";
            }

            artifacts.push(Artifact{artifact_type: t.to_string() ,name: n.to_string()});
        }
    }

    return artifacts;
}

pub fn get_relations(main: Element) -> Vec<Relations> {
    let mut rel: Vec<Relations> = Vec::new();

    for mut child in main.clone().children {
        if child.name.to_string() == "relations" {
            let mut n_n: Vec<Node_Node> = Vec::new();
            let mut a_n: Vec<Artifact_Node> = Vec::new();

            n_n = get_node_node(child.children.clone());
            a_n = get_artifact_node(child.children.clone());

            rel.push(Relations{ node_node: n_n, artifact_node: a_n});
        }
    }
    return rel;
}

//-------------------------------------------------------------------------------------------------
//METHODS - PRIVATE
//-------------------------------------------------------------------------------------------------
fn get_node_node(elements:Vec<Element>) -> Vec<Node_Node> {
    let mut n_n: Vec<Node_Node> = Vec::new();

    for element in elements {
        if element.name.to_string() == "node_node" {
            let mut n = "";
            let mut no = RelationNode{name: "".to_string(), multiplicity: "".to_string()};
            let mut no2  = RelationNode{name: "".to_string(), multiplicity: "".to_string()};
            for (key, value) in &element.attributes {
                if key.to_string() == "name" {
                    n = value;
                }
            }

            for child in element.children {
                if child.name.to_string() == "node" {
                    no = get_relation_node(child);
                } else if child.name.to_string() == "to_node" {
                    no2 = get_relation_node(child);
                }
            }

            n_n.push(Node_Node{ name: n.to_string(), node: no, to_node: no2 });
        }
    }
    return n_n;
}

fn get_relation_node(element: Element) -> RelationNode {
    let mut n = "";
    let mut m = "";

    for (key, value) in &element.attributes {
        if key.to_string() == "name" {
            n = value;
        } else if key.to_string() == "multiplicity" {
            m = value;
        }
    }

    return RelationNode{name: n.to_string(), multiplicity: m.to_string()};
}

fn get_artifact_node(elements:Vec<Element>) -> Vec<Artifact_Node> {
    let mut a_n: Vec<Artifact_Node> = Vec::new();

    for element in elements {
        if element.name.to_string() == "artifact_node" {
            let mut a = "";
            let mut n = "";
            for (key, value) in &element.attributes {
                if key.to_string() == "artifact" {
                    a = value;
                } else if key.to_string() == "to_node" {
                    n = value;
                }
            }

            a_n.push(Artifact_Node{ artifact: a.to_string(), to_node: n.to_string()});
        }
    }
    return a_n;
}
