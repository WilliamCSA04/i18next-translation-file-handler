use regex::Regex;

pub fn remove_using_regex(re: &str, s: String) -> String {
    let re = Regex::new(re).unwrap();
    return re.replace_all(&s, "").to_string();
}

pub fn get_i18next_keys(s: String) -> Vec<String> {
    println!("Getting keys from a file");
    let re_keys = Regex::new(r"t\('([^']*)'\)").unwrap();
    let mut processed_keys = Vec::new();

    re_keys.captures_iter(&s).for_each(|keys| {
        keys.iter().for_each(|k| {
            let text = remove_using_regex(&re_keys.to_string(), k.unwrap().as_str().to_string());
            println!("Key: {}", text);
            processed_keys.push(text);
        });
    });
    let key_list: std::vec::IntoIter<String> = processed_keys.into_iter();
    let filtered_keys = key_list.filter(|k| k != "");
    filtered_keys.into_iter().collect()
}
