mod file;
mod utils;
use clap::{self, Command};
use log::{self, warn};

fn main() {
    let matches = Command::new("i18next translation file handler")
        .version("0.1")
        .about("generates a json file with all the keys from pages")
        .subcommand(Command::new("create").about("Creates something"))
        .get_matches();

    match matches.subcommand() {
        Some(("create", _sub_m)) => {
            let files: Vec<String> = utils::get_all_files_paths("./src/input").unwrap();
            let file_keys = utils::get_all_keys(files);
            let file_keys: Vec<String> = file_keys.into_iter().collect();
            let json = utils::json_string_from_keys(file_keys);
            let _ = file::write_file("./src/output/keys.json", &json);
        }
        _ => {
            warn!("Unknown command");
        }
    }
}
