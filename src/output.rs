use crate::ast::{Function, Visibility};
use crate::selector::{compute_selector, format_selector};
use serde::Serialize;

#[derive(Serialize)]
pub struct FunctionOutput {
    pub selector: String,
    pub signature: String,
    #[serde(skip_serializing)]
    pub visibility: String,
}

impl FunctionOutput {
    pub fn from_function(func: &Function) -> Self {
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
        }
    }
}

// tabbed output, default
pub fn output_tsv(functions: &[FunctionOutput]) {
    let max_len = functions
        .iter()
        .map(|f| f.signature.len())
        .max()
        .unwrap_or(0);
    // header
    println!("{:<15} {:<len$}", "selector", "signature", len = max_len);

    // rows
    for func in functions {
        println!(
            "{:<15} {:<len$}",
            func.selector,
            func.signature,
            len = max_len
        );
    }
}

// json output
pub fn output_json(functions: &[FunctionOutput]) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string_pretty(functions)?;
    println!("{}", json);
    Ok(())
}
