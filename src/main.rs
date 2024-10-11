mod commands;
mod file;
mod utils;
use clap::{self, Command};
use log::{self, info, warn};

fn main() {
    let matches = Command::new("i18next translation file handler")
        .version("0.1")
        .about("generates a json file with all the keys from pages")
        .subcommand(Command::new("create").about("Creates something"))
        .subcommand(Command::new("update").about("Updates something"))
        .get_matches();

    match matches.subcommand() {
        Some(("create", _sub_m)) => {
            info!("Creating...");
            commands::create(Some("./src/output/keys.json"));
        }
        Some(("update", _sub_m)) => {
            info!("Updating...");
            commands::update();
        }
        _ => {
            warn!("Unknown command");
        }
    }
}
