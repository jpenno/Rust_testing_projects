#![allow(dead_code)]

mod card;
mod deck;

// use crate::card::*;
use crate::deck::*;

fn main() {
    let deck = Deck::new();

    deck.print();
}
