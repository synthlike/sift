#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub type_name: Type,
    pub name: String,
    pub visibility: Visibility,
    pub returns: Parameter,
}

impl Variable {
    pub fn signature(&self) -> String {
        let params = self.getter_params(&self.type_name);
        if params.is_empty() {
            format!("{}()", self.name)
        } else {
            format!("{}({})", self.name, params.join(","))
        }
    }

    fn getter_params(&self, t: &Type) -> Vec<String> {
        match t {
            Type::Array(_) | Type::FixedArray(_, _) => {
                vec!["uint256".to_string()]
            }
            Type::Mapping(key, value) => {
                let mut params = vec![key.canonical()];
                if let Type::Mapping(_, _) = value.as_ref() {
                    params.extend(self.getter_params(value));
                }
                params
            }
            Type::Elementary(_) => vec![],
        }
    }
}

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
    Elementary(String),            // uint256, address, bool
    Array(Box<Type>),              // uint256[]
    FixedArray(Box<Type>, usize),  // uint256[3]
    Mapping(Box<Type>, Box<Type>), // mapping(address => uint256)
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
            Type::Mapping(key, value) => {
                format!("mapping({} => {})", key.canonical(), value.canonical())
            }
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
