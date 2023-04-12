use crate::card::*;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        // Diamonds
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
        for card in &self.cards {
            card.print();
        }
    }

    pub fn print_fmt(&self) {
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

        for i in 0..=13 {
            Deck::print_card_list(&hearts, i);
            Deck::print_card_list(&diamonds, i);
            Deck::print_card_list(&spades, i);
            Deck::print_card_list(&clubs, i);
            println!();
        }
    }

    fn print_card_list(card_list: &Vec<Card>, i: u32) {
        for card in card_list {
            if card.name == i.to_string() {
                if i != 10 {
                    print!(" ");
                }
                print!("{} {} ", card.name, card.get_suit());
                break;
            } else if card == card_list.last().unwrap() {
                // print!("         ");
            } else if card.name.chars().next().unwrap() == 'J'
                || card.name.chars().next().unwrap() == 'Q'
                || card.name.chars().next().unwrap() == 'K'
            {
                print!(" {} {} ", card.name, card.get_suit());
            }
        }
    }

    fn spawn_suit_of_cards(cards: &mut Vec<Card>, suit: Suit) {
        for i in 1..=13 {
            if i < 11 {
                cards.push(Card::new(i.to_string(), suit.clone()).unwrap());
            }
            if i == 11 {
                cards.push(Card::new("J".to_string(), suit.clone()).unwrap());
            }
            if i == 12 {
                cards.push(Card::new("Q".to_string(), suit.clone()).unwrap());
            }
            if i == 13 {
                cards.push(Card::new("K".to_string(), suit.clone()).unwrap());
            }
        }
    }
}
