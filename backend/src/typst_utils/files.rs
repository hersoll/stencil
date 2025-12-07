use std::process::Command;
use std::{fs::File, io::Write};

/// Compiles a Typst file to a PDF. Returns the PDF path.
pub fn compile(typst_file_name: &String) -> std::io::Result<String> {
    #[cfg(feature = "docker")]
    let path: String = String::from("/app/") + typst_file_name.as_str();

    #[cfg(not(feature = "docker"))]
    let path: String = String::from("./") + typst_file_name.as_str();

    let output_name = typst_to_pdf_name(&path);

    match Command::new("typst")
        .args(["compile", &path, output_name.as_str()])
        .status()
    {
        Ok(_) => Ok(output_name),
        Err(e) => Err(e),
    }
}

/// Creates and returns a .typ file from a file name.
///
/// Creates a typst file if it doesn't exist. Overrides any existing file with that name.
/// The file name can be given with or without .typ
pub fn create_typst_file(file_name: &String) -> std::io::Result<File> {
    let typst_file_name = to_typst_file_name(file_name);
    File::create(typst_file_name)
}

pub fn write(mut file: &File, s: String) -> std::io::Result<()> {
    file.write_all(s.as_bytes())?;
    Ok(())
}

pub fn to_typst_file_name(file_name: &String) -> String {
    let mut typst_file_name = file_name.clone();
    if !typst_file_name.ends_with(".typ") {
        typst_file_name += ".typ";
    }
    typst_file_name
}

pub fn typst_to_pdf_name(typst_file_name: &String) -> String {
    let pdf_name = typst_file_name.clone().trim_end_matches(".typ").to_owned() + ".pdf";
    pdf_name
}
