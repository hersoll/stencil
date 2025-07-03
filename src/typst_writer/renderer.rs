use std::process::Command;

pub fn compile(file_name: String) {
    println!("Compiling the document...");
    let typ_path: String = file_name.clone() + ".typ";
    Command::new("typst")
        .args(["compile", typ_path.as_str()])
        .status()
        .expect("failed to execute compilation");

    show(file_name);
}

fn show(file_name: String) {
    println!("Opening PDF...");
    let pdf_path: String = file_name + ".pdf";
    Command::new("open")
        .args(["-a", "Skim", pdf_path.as_str()])
        .status()
        .expect("failed to open the PDF");
}
