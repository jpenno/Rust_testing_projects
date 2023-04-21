use std::fs;

use crate::keybind::*;

#[derive(Debug)]
pub struct Keybinds {
    pub keybinds: Vec<Keybind>,
    pub file_path: String,
}

impl Keybinds {
    pub fn new(path: &String) -> Result<Keybinds, String> {
        let file_content: String = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(format!("Path: {err}"));
            }
        };

        Ok(Keybinds {
            keybinds: Self::get_keybinds_from_file(&file_content),
            file_path: path.to_string(),
        })
    }

    fn get_keybinds_from_file(file: &String) -> Vec<Keybind> {
        let mut keybinds: Vec<Keybind> = Vec::new();
        let mut catagori = String::new();
        let mut change_catagori = false;

        for line in file.split('\n') {
            if let Some(first_char) = line.chars().next() {
                if first_char == '#' {
                    // get catagori
                    if (line.as_bytes()[1] as char) == '#' && !change_catagori {
                        change_catagori = true;
                        continue;
                    }
                    if (line.as_bytes()[1] as char) == ' ' && change_catagori {
                        catagori = line.to_string();
                        continue;
                    }
                    if (line.as_bytes()[1] as char) == '#' && change_catagori {
                        change_catagori = false;
                        continue;
                    }
                    keybinds.push(Keybind::new());
                    // set name
                    if let Some(last) = keybinds.last_mut() {
                        last.set_name(&line.to_string());
                        last.set_catagori(&catagori);
                    } else {
                        println!("last not there")
                    }
                    continue;
                }
                if !line.starts_with('\t') {
                    if let Some(last) = keybinds.last_mut() {
                        last.add_key(line.to_string());
                    } else {
                        println!("last not there")
                    }
                    continue;
                }
            };
        }
        keybinds
    }
}
