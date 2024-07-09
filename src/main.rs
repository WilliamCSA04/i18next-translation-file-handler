mod utils;
use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
};

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(path)?;
    println!("Read file: {}", file);
    Ok(file)
}

fn get_translations(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let contents = read_file(path)?;
    let mut translations = HashMap::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        translations.insert(key.to_string(), value.to_string());
    }
    Ok(translations)
}

fn main() {
    let paths = fs::read_dir("./src/i18n").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let file_path = path.to_str().unwrap();
        println!("Reading file: {}", file_path);
        match read_file(file_path) {
            Ok(contents) => {
                println!(
                    "Contents: {}",
                    utils::remove_new_lines_and_whitespace(contents)
                );
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
