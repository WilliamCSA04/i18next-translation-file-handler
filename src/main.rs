mod file;
mod utils;
use std::{collections::HashSet, error::Error};

use walkdir::WalkDir;

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

fn get_all_keys(files: Vec<String>) -> HashSet<String> {
    let mut file_keys = HashSet::new();

    for file in files {
        println!("Reading file...");
        match file::read_file(&file) {
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

    file_keys
}

fn json_string_from_keys(keys: Vec<String>) -> String {
    fn insert_key(map: &mut serde_json::Map<String, serde_json::Value>, key_parts: &[&str]) {
        match key_parts {
            [head, tail @ ..] if !tail.is_empty() => {
                let entry = map
                    .entry((*head).to_string())
                    .or_insert_with(|| serde_json::json!({}));
                if let serde_json::Value::Object(ref mut sub_map) = entry {
                    insert_key(sub_map, tail);
                }
            }
            [head] => {
                map.insert(
                    (*head).to_string(),
                    serde_json::Value::String(String::new()),
                );
            }
            _ => {}
        }
    }

    let mut map = serde_json::Map::new();
    for key in keys {
        let key_parts: Vec<&str> = key.split('.').collect();
        insert_key(&mut map, &key_parts);
    }

    serde_json::to_string_pretty(&serde_json::Value::Object(map)).unwrap()
}

fn main() {
    let files: Vec<String> = get_all_files_paths("./src/input").unwrap();
    let file_keys = get_all_keys(files);
    let file_keys: Vec<String> = file_keys.into_iter().collect();
    let json = json_string_from_keys(file_keys);
    let _ = file::write_file("./src/output/keys.json", &json);

    println!("File keys: {:?}", json);
}
