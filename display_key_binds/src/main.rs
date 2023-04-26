#![allow(dead_code)]
mod config;
mod keybind;
mod keybinds;

#[cfg(test)]
mod test;

use config::*;
use keybinds::*;
use std::{io, ops::Index};

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

    keybinds.print_catagories();

    println!("Select a catagori with the number");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let input = input.as_bytes()[0] as char;
        match input {
            '0'..='9' => {
                let i: usize = input
                    .to_digit(10)
                    .expect("failed converting char to int")
                    .try_into()
                    .unwrap();

                for keybind in &keybinds.keybinds {
                    if &keybind.catagori == keybinds.catagories.index(i - 1) {
                        keybind.print();
                        println!();
                    }
                }
            }
            'c' => {
                keybinds.print_catagories();
            }
            'q' => {
                println!("quit");
                break;
            }
            _ => println!("not handled"),
        }
    }
}
