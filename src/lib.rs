mod card;
mod deck;
pub mod errors;
mod suit;

pub use card::Card;
pub use deck::Deck;
pub use suit::Suit;

pub const DECK_SIZE: usize = 52;
pub const SUIT_SIZE: usize = DECK_SIZE / 4;
