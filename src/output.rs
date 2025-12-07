use crate::ast::{Function, Visibility};
use crate::selector::{compute_selector, format_selector};
use serde::Serialize;

#[derive(Serialize)]
pub struct FunctionOutput {
    pub selector: String,
    pub signature: String,
    #[serde(skip_serializing)]
    pub visibility: String,
    pub file: String,
}

impl FunctionOutput {
    pub fn from_function(func: &Function, file: String) -> Self {
        let signature = func.signature();
        let selector = compute_selector(&signature);
        let visibility = match func.visibility {
            Visibility::External => "external",
            Visibility::Public => "public",
            Visibility::Internal => "internal",
            Visibility::Private => "private",
        }
        .to_string();

        FunctionOutput {
            selector: format_selector(&selector),
            signature,
            visibility,
            file,
        }
    }
}

// tabbed output, default
pub fn output_tsv(functions: &[FunctionOutput]) {
    // header
    println!("{:<15} {:<30} {}", "selector", "signature", "file");

    // rows
    for func in functions {
        println!("{:<15} {:<30} {}", func.selector, func.signature, func.file);
    }
}

// json output
pub fn output_json(functions: &[FunctionOutput]) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string_pretty(functions)?;
    println!("{}", json);
    Ok(())
}
