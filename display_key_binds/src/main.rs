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
    // clear console and set courser to the top
    print!("\x1B[2J\x1B[1;1H");

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

    print_menu();

    // println!("Select a catagori with the number");
    loop {
        // clear console and set courser to the top
        print!("\x1B[2J\x1B[1;1H");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let input = input.as_bytes()[0] as char;
        match input {
            '0'..='9' => print_selected_catagory(input, &keybinds),
            'c' => keybinds.print_catagories(),
            's' => search(&keybinds),
            'm' => print_menu(),
            'q' => {
                println!("quit");
                break;
            }
            _ => {
                println!("not handled");
                print_menu();
            }
        }
    }
}

fn print_menu() {
    println!("Menu");
    println!("'c' for a list of catcgories");
    println!("'s' to search for a button name");
    println!("'q' to quit");
}

fn print_selected_catagory(input: char, keybinds: &Keybinds) {
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

fn search(keybinds: &Keybinds) {
    println!("Type the name of the key bind you are looking for:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    for keybind in &keybinds.keybinds {
        if keybind.name.contains(input) {
            keybind.print();
            println!();
        }
    }
}
