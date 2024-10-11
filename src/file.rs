use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};
pub fn write_file(path: &str, content: &str) -> Result<File, Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(file)
}

pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(path)?;
    Ok(file)
}

pub fn delete_file(path: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_file(path)?;
    Ok(())
}
