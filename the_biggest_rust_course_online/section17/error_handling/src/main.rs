use std::fs;
use std::{error::Error, io};

fn write_to_file() -> Result<String, io::Error> {
    let mut file_name = String::default();
    let mut file_content = String::default();

    let res = io::stdin().read_line(&mut file_name)?;
    file_name = file_name.trim().to_string();
    let res2 = io::stdin().read_line(&mut file_content)?;
    file_content = file_content.trim().to_string();
    fs::write(&file_name, file_content.as_bytes());

    Ok(file_name)
}

fn main() {
    // println!("Hello, world!");
    let result = write_to_file();
    match result {
        Ok(val) => println!("File named {} created successfully.", val),
        Err(e) => println!("Error {:#?}.", e),
    }
}
