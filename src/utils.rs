use std::collections::HashSet;

use regex::Regex;

pub fn remove_using_regex(re: &str, s: String) -> String {
    let re = Regex::new(re).unwrap();
    return re.replace_all(&s, "").to_string();
}

pub fn get_i18next_keys(s: String) -> Vec<String> {
    let re_keys = Regex::new(r"t\('([^']*)'\)").unwrap();
    let mut keys = HashSet::new();
    if let Some(captured_keys) = re_keys.captures(&s) {
        for key in captured_keys.iter() {
            match key {
                Some(k) => {
                    let text = remove_using_regex(&re_keys.to_string(), k.as_str().to_string());
                    keys.insert(text);
                }
                None => {
                    println!("No key found");
                }
            }
        }
    }
    let key_list = keys.into_iter();
    let filtered_keys = key_list.filter(|k| k != "");
    return filtered_keys.into_iter().collect();
}
