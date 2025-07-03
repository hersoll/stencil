use std::error::Error;
use std::{fs::File, io::Write};

mod renderer;

pub fn write(message: &str) -> Result<(), Box<dyn Error>> {
    println!("Opening and writing...");
    let file_name = "typst";
    let typst_path = file_name.to_owned() + ".typ";
    let mut buffer = File::create(typst_path)?;
    buffer.write_all(message.as_bytes())?;
    println!("Finished writing to the Typst document.");

    renderer::compile(file_name.to_owned());

    Ok(())
}
