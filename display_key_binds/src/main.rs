#![allow(dead_code)]
mod config;
mod keybind;
mod keybinds;

#[cfg(test)]
mod test;

use config::*;
use keybinds::*;
use std::io;

fn main() {
    let config: Config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            println!("Config: {err}");
            return;
        }
    };
    let keybinds: Keybinds = match Keybinds::new(&config.path) {
        Ok(keys) => keys,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // print all keybinds
    println!("Print Keybinds");
    for keybind in &keybinds.keybinds {
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
    for keybind in keybinds.keybinds {
        if keybind.catagori == input.trim() {
            keybind.print();
            println!();
        }
    }
}
