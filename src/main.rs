mod deck;
mod kitty;

use crate::deck::Deck;
use crate::kitty::Kitty;

fn main() {
    let deck: Deck = Deck::new();
    let kitty: Kitty = Kitty::new(deck.deal(10, 9));

    kitty.get_optimized_hath();
}
