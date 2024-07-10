mod utils;
use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
};

use walkdir::WalkDir;

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(path)?;
    println!("Read file: {}", file);
    Ok(file)
}

fn get_all_files_paths(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e: Result<walkdir::DirEntry, walkdir::Error>| e.ok())
    {
        if entry.metadata()?.is_file() {
            files.push(entry.path().display().to_string());
        }
    }

    Ok(files)
}

fn main() {
    let files = get_all_files_paths("./src/i18n").unwrap();
    for file_path in files {
        println!("Reading file: {}", file_path);
        match read_file(&file_path) {
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
