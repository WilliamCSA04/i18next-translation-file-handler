use log::{self, error, info};
use regex::Regex;
use std::{collections::HashSet, error::Error};

use walkdir::WalkDir;

use crate::file::read_file;

fn remove_using_regex(re: &str, s: String) -> String {
    let re = Regex::new(re).unwrap();
    return re.replace_all(&s, "").to_string();
}

fn get_prefix(s: &str) -> String {
    let re_quotes = Regex::new(r#"keyPrefix:\s*"\s*([^"]+)\s*""#).unwrap();
    let mut result = String::new();

    // Capture the content inside parentheses
    if let Some(caps) = re_quotes.captures(s) {
        let contents = &caps[1].to_string();
        // Capture the content inside double quotes within parentheses
        result.push_str(contents);
        result.push('.'); // Add a space between captures
    }

    result.trim_end().to_string() // Remove trailing space and convert to String
}

fn get_i18next_keys(s: String) -> Vec<String> {
    info!("Getting keys from a file");
    let re_keys = Regex::new(r"t\('([^']*)'\)").unwrap();
    let mut processed_keys = Vec::new();
    info!("Checking for prefix");
    let prefix = get_prefix(&s);
    re_keys.captures_iter(&s).for_each(|keys| {
        keys.iter().for_each(|k| {
            let text = remove_using_regex(&re_keys.to_string(), k.unwrap().as_str().to_string());
            processed_keys.push(text);
        });
    });
    let key_list: std::vec::IntoIter<String> = processed_keys.into_iter();
    let filtered_keys = key_list.filter(|k| k != "").map(|k| format!("{prefix}{k}"));
    info!("Finished getting keys from a file");
    filtered_keys.into_iter().collect()
}

pub fn get_all_files_paths(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    info!("Getting all files paths");
    let mut files = Vec::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e: Result<walkdir::DirEntry, walkdir::Error>| e.ok())
    {
        if entry.metadata()?.is_file() {
            files.push(entry.path().display().to_string());
        }
    }
    info!("Finished getting all files paths");
    Ok(files)
}

pub fn get_all_keys(files: Vec<String>) -> HashSet<String> {
    let mut file_keys = HashSet::new();

    for file in files {
        info!("Reading file...");
        match read_file(&file) {
            Ok(contents) => {
                let content = remove_using_regex(r"\s+", contents).trim().to_string();
                let keys: Vec<String> = get_i18next_keys(content);
                file_keys.extend(keys);
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }

    file_keys
}

pub fn json_string_from_keys(keys: Vec<String>) -> String {
    info!("Creating json string from keys");
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
    info!("Finished creating json string from keys");
    serde_json::to_string_pretty(&serde_json::Value::Object(map)).unwrap()
}
