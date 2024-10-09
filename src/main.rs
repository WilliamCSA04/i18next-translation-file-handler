mod commands;
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
            commands::create();
        }
        Some(("update", _sub_m)) => {
            commands::update();
        }
        _ => {
            warn!("Unknown command");
        }
    }
}
