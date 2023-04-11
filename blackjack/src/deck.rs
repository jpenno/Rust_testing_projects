use crate::card::*;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        // Diamds
        Deck::spawn_suit_of_cards(&mut cards, Suit::Diamond);

        // Hearts
        Deck::spawn_suit_of_cards(&mut cards, Suit::Heart);

        // spade
        Deck::spawn_suit_of_cards(&mut cards, Suit::Spade);

        // club
        Deck::spawn_suit_of_cards(&mut cards, Suit::Club);

        Deck { cards }
    }

    pub fn print(&self) {
        let mut diamonds: Vec<Card> = Vec::new();
        let mut hearts: Vec<Card> = Vec::new();
        let mut spades: Vec<Card> = Vec::new();
        let mut clubs: Vec<Card> = Vec::new();

        for card in &self.cards {
            match card.suit {
                Suit::Heart => hearts.push(card.clone()),
                Suit::Diamond => diamonds.push(card.clone()),
                Suit::Spade => spades.push(card.clone()),
                Suit::Club => clubs.push(card.clone()),
            }
        }

        for i in 0..=10 {
            Deck::print_card_list(&hearts, i);
            Deck::print_card_list(&diamonds, i);
            Deck::print_card_list(&spades, i);
            Deck::print_card_list(&clubs, i);
            println!("");
        }
    }

    fn print_card_list(card_list: &Vec<Card>, i: u32) {
        // print spades
        for spade in card_list {
            if spade.name == i.to_string() {
                if i != 10 {
                    print!(" ");
                }
                print!("{} {} ", spade.name, spade.get_suit());
                break;
            } else if spade == card_list.last().unwrap() {
                print!("         ");
            }
        }
    }

    fn spawn_suit_of_cards(cards: &mut Vec<Card>, suit: Suit) {
        for i in 1..=10 {
            cards.push(Card::new(i.to_string(), suit.clone()));
        }
    }
}
