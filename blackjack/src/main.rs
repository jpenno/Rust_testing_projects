//#![allow(dead_code)]

mod card;
mod deck;

use card::Card;

use crate::deck::*;

// test

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    deck.print();

    let mut player = Player::new();
    println!("Print hand");

    player.draw_card(deck.draw());
    player.draw_card(deck.draw());

    player.print_hand();
}

// #[derive(Debug)]
// struct Hand {
//     cards: Card,
// }

#[derive(Debug)]
struct Player {
    hand: Vec<Card>,
}
impl Player {
    pub fn new() -> Player {
        Player { hand: Vec::new(),}
    }
    pub fn draw_card(&mut self, card: Card) {
        self.hand.push(card);
    }
    pub fn print_hand(&mut self) {
        for card in &self.hand {
            card.print();
        }
    }
}
