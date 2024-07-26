mod utils;
use std::{
    collections::HashSet,
    error::Error,
    fs::{self, File},
    io::Write,
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

fn write_file(path: &str, content: &str) -> Result<File, Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(file)
}

fn get_all_keys(files: Vec<String>) -> HashSet<String> {
    let mut file_keys = HashSet::new();

    for file in files {
        println!("Reading file...");
        match read_file(&file) {
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

    return file_keys;
}

fn main() {
    let files: Vec<String> = get_all_files_paths("./src/react").unwrap();
    let file_keys = get_all_keys(files);
    println!("File keys: {:?}", file_keys);
}
