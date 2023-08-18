use crate::card::Card;
use crate::errors;
use crate::suit::Suit;
use crate::{DECK_SIZE, SUIT_SIZE};
use hmac;
use pbkdf2;
use rand;
use sha2;
use std::{self, fmt};

const PBKDF2_ITERATIONS: u32 = 1 << 16;

#[derive(Debug, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, card) in self.cards.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", self.cards[0])?;
            } else {
                write!(f, " {}", card)?;
            }
        }
        Ok(())
    }
}

impl std::str::FromStr for Deck {
    type Err = errors::ParseError;

    fn from_str(s: &str) -> Result<Deck, errors::ParseError> {
        let mut deck = Deck { cards: vec![] };
        for chunk in s.split_whitespace() {
            deck.cards.push(chunk.parse::<Card>()?);
        }

        Ok(deck)
    }
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: vec![] };
        let suits = Suit::all();
        for i in 0..DECK_SIZE {
            deck.cards.push(Card {
                value: (i % SUIT_SIZE) as u32,
                suit: suits[i / SUIT_SIZE],
            });
        }
        deck
    }

    pub fn shuffle(&self) -> Deck {
        let samples = rand::seq::index::sample(&mut rand::rngs::OsRng, DECK_SIZE, DECK_SIZE);
        let mut shuffled = Deck::new();
        for (i, j) in std::iter::zip(0..DECK_SIZE, samples) {
            shuffled.cards[i] = self.cards[j];
        }
        shuffled
    }

    pub fn hash(&self, password: Option<&str>) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        let mut preimage = self.to_string();
        match password {
            Some(password) => {
                preimage.push_str(":");
                preimage.push_str(password);
            }
            None => {}
        };

        let mut output = [0u8; 32];
        pbkdf2::pbkdf2::<hmac::Hmac<sha2::Sha256>>(
            preimage.as_bytes(),
            b"",
            PBKDF2_ITERATIONS,
            &mut output,
        )?;
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_new() {
        let deck = Deck::new();
        assert_eq!(
            deck.cards[15],
            Card {
                value: 2,
                suit: Suit::Clubs,
            }
        )
    }

    #[test]
    fn shuffle() {
        let deck = Deck::new().shuffle();
        assert_ne!(deck.cards[0], Card::ace_of_spades());
    }

    #[test]
    fn to_string() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            Deck::new().to_string(),
            "AS 2S 3S 4S 5S 6S 7S 8S 9S TS JS QS KS AC 2C 3C 4C 5C 6C \
             7C 8C 9C TC JC QC KC AH 2H 3H 4H 5H 6H 7H 8H 9H TH JH QH \
             KH AD 2D 3D 4D 5D 6D 7D 8D 9D TD JD QD KD"
        );

        Ok(())
    }

    #[test]
    fn from_string() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            " AS\n 2D 3C  8H \tQD\n".parse::<Deck>(),
            Ok(Deck {
                cards: vec![
                    Card {
                        value: 0,
                        suit: Suit::Spades
                    },
                    Card {
                        value: 1,
                        suit: Suit::Diamonds
                    },
                    Card {
                        value: 2,
                        suit: Suit::Clubs
                    },
                    Card {
                        value: 7,
                        suit: Suit::Hearts
                    },
                    Card {
                        value: 11,
                        suit: Suit::Diamonds
                    },
                ],
            })
        );

        Ok(())
    }

    #[test]
    fn hash() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            Deck::new().hash(None)?,
            [
                204, 147, 92, 129, 195, 255, 197, 30, 16, 196, 216, 17, 114, 172, 27, 55, 31, 20,
                238, 190, 66, 93, 236, 204, 173, 229, 53, 227, 189, 76, 227, 224
            ]
        );

        assert_eq!(
            Deck::new().hash(Some("slick"))?,
            [
                234, 182, 196, 8, 21, 159, 226, 239, 223, 128, 66, 185, 211, 166, 63, 83, 198, 254,
                27, 246, 199, 237, 44, 207, 237, 34, 164, 191, 222, 104, 17, 133
            ]
        );

        Ok(())
    }
}
