#![allow(dead_code)]
mod config;
mod keybind;

use keybind::*;
use std::{fs, io};

use crate::config::Config;

fn main() {
    let config: Config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            println!("Err: {err}");
            return;
        }
    };

    let file_content: String = match fs::read_to_string(&config.path) {
        Ok(file) => file,
        Err(err) => {
            println!("Path: {:?}", config.path);
            print!("Config Err: {err}");
            return;
        }
    };

    let keybinds: Vec<Keybind> = Keybind::get_keybinds_from_file(file_content);

    // print all keybinds
    println!("Print Keybinds");
    for keybind in &keybinds {
        keybind.print();
        println!();
    }

    println!("Select a catagori");

    // get catagori from the user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    // print all keybinds in selected catagori
    for keybind in keybinds {
        if keybind.catagori == input.trim() {
            keybind.print();
            println!();
        }
    }
}
