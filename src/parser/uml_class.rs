#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub classes: Vec<Class>,
}

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub methods: Vec<Method>,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub visibility: String,
    pub data_type: String,
}

#[derive(Clone, Debug)]
pub struct Method {
    pub name: String,
    pub visibility: String,
    pub return_type: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub data_type: String,
}

#[derive(Clone, Debug)]
pub struct Relationship {
    pub relation_type: String,
    pub class: RelationshipClass,
    pub to_class: RelationshipClass,
}

#[derive(Clone, Debug)]
pub struct RelationshipClass {
    pub name: String,
    pub multiplicity: String,
}
