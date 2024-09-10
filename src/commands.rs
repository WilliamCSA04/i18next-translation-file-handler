use crate::{file, utils};

pub fn create() {
    let files: Vec<String> = utils::get_all_files_paths("./src/input").unwrap();
    let file_keys = utils::get_all_keys(files);
    let file_keys: Vec<String> = file_keys.into_iter().collect();
    let json = utils::json_string_from_keys(file_keys);
    let _ = file::write_file("./src/output/keys.json", &json);
}
