#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Card {
    pub name: String,
    pub value: u32,
    pub suit: Suit,
    pub done: bool,
}

impl Card {
    pub fn new(name: String, suit: Suit) -> Result<Card, String> {
        let value: u32 = if let Ok(num) = name.parse() {
            num
        } else {
            match name.chars().next() {
                Some('J') | Some('Q') | Some('K') => 10,
                None => {
                    return Err("No name".to_string());
                }
                _ => {
                    return Err("Name not handled".to_string());
                }
            }
        };

        Ok(Card {
            name,
            value,
            suit,
            done: false,
        })
    }
    pub fn get_suit(&self) -> String {
        match self.suit {
            Suit::Heart => "Heart".to_string(),
            Suit::Diamond => "Diamond".to_string(),
            Suit::Spade => "Spade".to_string(),
            Suit::Club => "Club".to_string(),
        }
    }
    pub fn print(&self) {
        println!(
            "Name: {} Suit: {:?} Vallue: {}",
            self.name, self.suit, self.value
        );
    }
}
