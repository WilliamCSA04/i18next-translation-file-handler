use regex::Regex;

pub fn remove_new_lines_and_whitespace(s: String) -> String {
    let re_spaces = Regex::new(r"\s+").unwrap();
    return re_spaces
        .replace_all(&re_spaces.replace_all(&s, " "), " ")
        .to_string();
}
