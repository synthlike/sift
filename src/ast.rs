#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub visibility: Visibility,
    pub mutability: Option<Mutability>,
    pub returns: Option<Vec<Parameter>>,
}

impl Function {
    pub fn signature(&self) -> String {
        let param_types: Vec<String> = self
            .parameters
            .iter()
            .map(|p| p.type_name.canonical())
            .collect();

        format!("{}({})", self.name, param_types.join(","))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub type_name: Type,
    pub name: Option<String>,
    pub data_location: Option<DataLocation>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Elementary(String),           // uint256, address, bool
    Array(Box<Type>),             // uint256[]
    FixedArray(Box<Type>, usize), // uint256[3]
}

impl Type {
    pub fn canonical(&self) -> String {
        match self {
            Type::Elementary(name) => match name.as_str() {
                "uint" => "uint256".to_string(),
                "int" => "int256".to_string(),
                "byte" => "bytes1".to_string(),
                _ => name.clone(),
            },
            Type::Array(inner) => format!("{}[]", inner.canonical()),
            Type::FixedArray(inner, size) => format!("{}[{}]", inner.canonical(), size),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    External,
    Public,
    Internal,
    Private,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mutability {
    Pure,
    View,
    Payable,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataLocation {
    Memory,
    Calldata,
    Storage,
}
