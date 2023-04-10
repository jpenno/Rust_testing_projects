
#[derive(Debug, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Clone)]
pub struct Card {
    pub name: String,
    pub value: u8,
    pub suit: Suit,
    pub done: bool,
}

impl Card {
    pub fn new(name: String, suit: Suit) -> Card {
        return Card {
            name,
            value: 0,
            suit,
            done: false,
        };
    }
    pub fn get_suit(&self) -> String {
        match self.suit {
            Suit::Heart => "Heart".to_string(),
            Suit::Diamond => "Diamond".to_string(),
            Suit::Spade => "Spade".to_string(),
            Suit::Club => "Club".to_string(),
        }
    }
    pub fn print(&self){
        println!("{0} {1:?} {2}", self.name, self.suit, self.value);
    }
}
