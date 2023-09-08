#[doc = include_str!("../README.md")]
mod card;
mod deck;
pub mod errors;
mod suit;

pub use card::Card;
pub use deck::Deck;
pub use suit::Suit;

/// The size of a full valid deck with no duplicates.
pub const DECK_SIZE: usize = 52;

/// The number of cards in a suit.
pub const SUIT_SIZE: usize = DECK_SIZE / 4;
