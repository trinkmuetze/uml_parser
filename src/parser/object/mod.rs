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
//METHODS
//-------------------------------------------------------------------------------------------------
