
#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

pub struct Card {
    name: String,
    value: u8,
    suit: Suit,
}

impl Card {
    pub fn new(name: String, suit: Suit) -> Card {
        return Card {
            name,
            value: 0,
            suit,
        };
    }
    pub fn print(&self){
        println!("{0} {1:?} {2}", self.name, self.suit, self.value);
    }
}
