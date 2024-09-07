// module to get all cards and return specific no of cards

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // returns full set of cards
    pub fn new() -> Self {
        let suits: [&str; 4] = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values: [&str; 13] = [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];

        let mut cards: Vec<String> = vec![];

        for suit in suits {
            for value in values {
                cards.push(format!("{value} of {suit}"));
            }
        }

        Deck { cards }
    }

    // -- num_of_shuffle; How many times you want to shuffle the card?
    // max of 255 to not cause system overload
    // -- num_of_cards; Number of cards you want to get
    pub fn deal(&self, num_of_shuffle: u8, num_of_cards: usize) -> Vec<String> {
        let mut cards = self.cards.clone();

        if num_of_shuffle != 0 {
            for _ in [0..num_of_shuffle] {
                cards.shuffle(&mut thread_rng());
            }
        }

        cards.split_off(cards.len() - num_of_cards)
    }
}
