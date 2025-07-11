use std::process::Command;

use super::file_helpers;

/// Compiles a Typst file to a PDF. Returns the PDF path.
pub fn compile(typst_file_name: &String) -> std::io::Result<String> {
    assert!(typst_file_name.ends_with(".typ"));
    let output_name = file_helpers::typst_to_pdf_name(&typst_file_name);

    match Command::new("typst")
        .args(["compile", typst_file_name.as_str(), output_name.as_str()])
        .status()
    {
        Ok(_) => Ok(output_name),
        Err(e) => Err(e),
    }
}
