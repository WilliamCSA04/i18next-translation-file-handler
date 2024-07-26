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

    return file_keys;
}

fn json_string_from_keys(keys: Vec<String>) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    for key in keys {
        let key_parts: Vec<String> = key.split(".").map(|s| s.to_string()).collect();
        if key_parts.len() > 1 {
            json.push_str(&format!(
                "    \"{}\": {},\n",
                key,
                json_string_from_keys(key_parts)
            ));
        } else {
            json.push_str(&format!("    \"{}\": {},\n", key, ""));
        }
    }
    json.push_str("}");
    return json;
}

fn main() {
    let files: Vec<String> = get_all_files_paths("./src/react").unwrap();
    let file_keys = get_all_keys(files);
    let file_keys: Vec<String> = file_keys.into_iter().collect();
    let json = json_string_from_keys(file_keys);

    println!("File keys: {:?}", json);
}
