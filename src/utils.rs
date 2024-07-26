use regex::Regex;

pub fn remove_using_regex(re: &str, s: String) -> String {
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

pub fn get_i18next_keys(s: String) -> Vec<String> {
    println!("Getting keys from a file");
    let re_keys = Regex::new(r"t\('([^']*)'\)").unwrap();
    let mut processed_keys = Vec::new();
    let prefix = get_prefix(&s);
    println!("Prefix: {prefix}");
    re_keys.captures_iter(&s).for_each(|keys| {
        keys.iter().for_each(|k| {
            let text = remove_using_regex(&re_keys.to_string(), k.unwrap().as_str().to_string());
            println!("Key: {}", text);
            processed_keys.push(text);
        });
    });
    let key_list: std::vec::IntoIter<String> = processed_keys.into_iter();
    let filtered_keys = key_list.filter(|k| k != "").map(|k| format!("{prefix}{k}"));
    filtered_keys.into_iter().collect()
}
