use log::{self, error, info};
use regex::Regex;
use serde_json::Value;
use std::{collections::HashSet, error::Error};

use walkdir::WalkDir;

use crate::file::read_file;
use crate::file::write_file;

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

pub fn keys_from_json(file_paths: &Vec<String>) -> HashSet<String> {
    let mut keys = HashSet::new();

    for path in file_paths {
        let file_content = read_file(&path).expect("Unable to read file");
        let v: Value = serde_json::from_str(&file_content).expect("Invalid JSON");
        collect_keys(&v, &mut keys, String::new());
    }

    keys
}

pub fn compare_keys(
    files_keys: &HashSet<String>,
    current_keys: &HashSet<String>,
) -> (HashSet<String>, HashSet<String>) {
    let mut keys_to_remove = HashSet::new();
    let mut keys_to_add = HashSet::new();

    for key in current_keys {
        if !files_keys.contains(key) {
            keys_to_remove.insert(key.to_string());
        }
    }

    for key in files_keys {
        if !current_keys.contains(key) {
            keys_to_add.insert(key.to_string());
        }
    }

    (keys_to_remove, keys_to_add)
}

fn collect_keys(value: &Value, keys: &mut HashSet<String>, parent_key: String) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let full_key = if parent_key.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", parent_key, key)
                };
                keys.insert(full_key.clone());
                collect_keys(val, keys, full_key);
            }
        }
        Value::Array(arr) => {
            for (index, val) in arr.iter().enumerate() {
                let full_key = format!("{}[{}]", parent_key, index);
                collect_keys(val, keys, full_key);
            }
        }
        _ => {} // Ignore other types like strings, numbers, etc.
    }
}

pub fn handle_keys_on_update(
    file_paths: &[String],
    keys_to_remove: &[String],
    keys_to_add: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    for file_path in file_paths {
        // Read the JSON file
        let file_content = read_file(&file_path)?;
        let mut json_value: Value = serde_json::from_str(&file_content)?;

        // Remove the key
        for key_to_remove in keys_to_remove {
            remove_key(&mut json_value, key_to_remove);
        }

        for key_to_add in keys_to_add {
            add_key(&mut json_value, key_to_add, Value::String("".to_string()));
        }

        let new_content = serde_json::to_string_pretty(&json_value)?;
        write_file(&file_path, &new_content)?;
    }

    Ok(())
}

fn remove_key(value: &mut Value, key_to_remove: &str) {
    match value {
        Value::Object(map) => {
            map.remove(key_to_remove);
            for val in map.values_mut() {
                remove_key(val, key_to_remove);
            }
        }
        Value::Array(arr) => {
            for val in arr.iter_mut() {
                remove_key(val, key_to_remove);
            }
        }
        _ => {}
    }
}

fn add_key(value: &mut Value, key: &str, new_value: Value) {
    if let Value::Object(map) = value {
        map.insert(key.to_string(), new_value);
    }
}
