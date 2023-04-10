mod card;

use crate::card::*;

fn main() {
    println!("Hello, world!");

    let deck = Deck::new();

    deck.print();
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        // Diamds
        for i in 1..=10 {
            cards.push(Card::new(i.to_string(), Suit::Diamond));
        }
        // Hearts
        for i in 5..=10 {
            cards.push(Card::new(i.to_string(), Suit::Heart));
        }

        // spade
        for i in 1..=4 {
            cards.push(Card::new(i.to_string(), Suit::Spade));
        }

        // club
        for i in 1..=10 {
            cards.push(Card::new(i.to_string(), Suit::Club));
        }

        Deck { cards }
    }

    fn print(&self) {
        let mut names: Vec<String> = Vec::new();

        for card in &self.cards {
            if !names.contains(&card.name) {
                names.push(card.name.clone());
            }
        }
        println!("Names: {:?}", names);

        for name in names {
            let mut print_cards: Vec<Card> = Vec::new();
            for card in &self.cards {
                if name == card.name {
                    print_cards.push(card.clone());
                }
            }
            let mut print_string = String::new();
            for p in print_cards {
                print_string.push_str(&p.name);
                print_string.push_str(" ");
                print_string.push_str(&p.get_suit());
                print_string.push_str(" | ");
            }
            println!("{}", print_string);
        }
    }
}
