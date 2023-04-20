use std::fs;

use crate::config::*;
use crate::keybind::*;

#[derive(Debug)]
pub struct Keybinds {
    pub keybinds: Vec<Keybind>,
}

impl Keybinds {
    pub fn new() -> Result<Keybinds, String> {
        let config: Config = match Config::new() {
            Ok(config) => config,
            Err(err) => {
                return Err(format!("Config: {err}"));
            }
        };

        let file_content: String = match fs::read_to_string(config.path) {
            Ok(file) => file,
            Err(err) => {
                return Err(format!("Path: {err}"));
            }
        };

        Ok(Keybinds {
            keybinds: Keybind::get_keybinds_from_file(file_content),
        })
    }
}
