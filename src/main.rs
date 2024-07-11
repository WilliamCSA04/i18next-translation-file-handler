mod utils;
use std::{
    collections::HashSet,
    error::Error,
    fs::{self},
};

use walkdir::WalkDir;

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(path)?;
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
    let files: Vec<String> = get_all_files_paths("./src/react").unwrap();
    let mut file_keys = HashSet::new();
    for file_path in files {
        println!("Reading file...");
        match read_file(&file_path) {
            Ok(contents) => {
                let content = utils::remove_using_regex(r"\s+", contents)
                    .trim()
                    .to_string();
                println!("Content: {}", content);
                let keys: Vec<String> = utils::get_i18next_keys(content);
                println!("Keys: {:?}", keys);
                file_keys.extend(keys);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    println!("F Keys: {:?}", file_keys);
}
