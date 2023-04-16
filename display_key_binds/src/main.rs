#![allow(dead_code)]
mod config;
mod keybind;

use keybind::*;
use std::fs;

use crate::config::Config;

fn main() {
    let config: Config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            println!("Err: {err}");
            return;
        }
    };

    println!("Main config path: {}", config.path);

    let file_content: String = match fs::read_to_string(&config.path) {
        Ok(file) => file,
        Err(err) => {
            println!("Path: {:?}", config.path);
            print!("Config Err: {err}");
            return;
        }
    };

    let mut keybinds: Vec<Keybind> = Vec::<Keybind>::new();

    keybinds.push(Keybind::new());
    let mut catagori = String::new();
    let mut change_catagori = false;

    for line in file_content.split('\n') {
        if let Some(first_char) = line.chars().next() {
            if first_char == '#' {
                // get catagori
                if (line.as_bytes()[1] as char) == '#' && !change_catagori {
                    change_catagori = true;
                    continue;
                }
                if (line.as_bytes()[1] as char) == ' ' && change_catagori {
                    // println!("catagori: {catagori}");
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
                    last.set_name(line.to_string());
                    last.set_catagori(catagori.clone());
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

    println!("Print Keybinds");
    for keybind in keybinds {
        keybind.print();
        println!();
    }
}
