use std::process::Command;

use super::file_helpers;

/// Compiles a Typst file to a PDF. Returns the PDF path.
pub fn compile(typst_file_name: &String) -> std::io::Result<String> {
    #[cfg(feature = "docker")]
    let path: String = String::from("/app/") + typst_file_name.as_str();

    #[cfg(not(feature = "docker"))]
    let path: String = String::from("./") + typst_file_name.as_str();

    let output_name = file_helpers::typst_to_pdf_name(&path);

    match Command::new("typst")
        .args(["compile", &path, output_name.as_str()])
        .status()
    {
        Ok(_) => Ok(output_name),
        Err(e) => Err(e),
    }
}
