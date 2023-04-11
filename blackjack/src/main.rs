#![allow(dead_code)]

mod card;
mod deck;

use crate::deck::*;

fn main() {
    let deck = Deck::new();

    deck.print();
    //test dev commit
    // more testing
}
