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
