use crate::{
    file::{self, delete_file},
    utils::{self, handle_keys_on_update},
};

pub fn create(output_path: Option<&str>) {
    let path = output_path.unwrap_or("./src/output/keys.json");
    let files: Vec<String> = utils::get_all_files_paths("./src/input").unwrap();
    let file_keys = utils::get_all_keys(files);
    let file_keys: Vec<String> = file_keys.into_iter().collect();
    let json = utils::json_string_from_keys(file_keys);
    let _ = file::write_file(path, &json);
}

pub fn update() {
    create(Some("./src/aux/keys.json"));
    let aux_files = utils::get_all_files_paths("./src/aux").unwrap();
    let file_keys = utils::keys_from_json(&aux_files);
    let output_files = utils::get_all_files_paths("./src/output").unwrap();
    let existing_keys = utils::keys_from_json(&output_files).into_iter().collect();
    let (keys_to_remove, keys_to_add) = utils::compare_keys(&file_keys, &existing_keys);
    let _ = handle_keys_on_update(
        &output_files,
        &keys_to_remove.into_iter().collect::<Vec<String>>(),
        &keys_to_add.into_iter().collect::<Vec<String>>(),
    );
    let _ = delete_file("./src/aux/keys.json");
}
