mod card;

use crate::card::*;

fn main() {
    println!("Hello, world!");
    let card = Card::new("1".to_string(), Suit::Heart);

    card.print();

}
