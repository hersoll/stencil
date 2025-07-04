use std::{fs::File, io::Write, process::Command};

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

pub fn open_pdf(pdf_file_name: String) {
    assert!(pdf_file_name.ends_with(".pdf"));
    println!("Opening PDF...");
    Command::new("open")
        .args(["-a", "Skim", pdf_file_name.as_str()])
        .status()
        .expect("failed to open the PDF");
}

pub fn to_typst_file_name(file_name: &String) -> String {
    let mut typst_file_name = file_name.clone();
    if !typst_file_name.ends_with(".typ") {
        typst_file_name += ".typ";
    }
    typst_file_name
}

pub fn typst_to_pdf_name(typst_file_name: &String) -> String {
    assert!(typst_file_name.ends_with(".typ"));
    let pdf_name = typst_file_name.clone().trim_end_matches(".typ").to_owned() + ".pdf";
    pdf_name
}
