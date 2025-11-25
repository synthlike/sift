use crate::lexer::Token;

pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub visibility: Visibility,
    pub mutability: Option<Mutability>,
    pub returns: Option<Vec<Parameter>>,
}

pub struct Parameter {
    pub type_name: Token,
    pub name: String,
    pub data_location: Option<DataLocation>,
}

pub enum Visibility {
    External,
    Public,
    Internal,
    Private,
}

pub enum Mutability {
    Pure,
    View,
    Payable,
}

pub enum DataLocation {
    Memory,
    Calldata,
    Storage,
}
