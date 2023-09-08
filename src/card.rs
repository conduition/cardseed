use crate::suit::Suit;
use crate::{errors, DECK_SIZE, SUIT_SIZE};
use std::{self, fmt};

/// Represents a single playing card. The `suit` field is the card's suit, and the `value`
/// field is the card's face value index from 0 to 12, where ace is zero and king is 12.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: u32,
}

impl Card {
    /// Creates an empty `Card`, which represents the ace of spades.
    /// Think of this as the zero card.
    pub fn ace_of_spades() -> Card {
        Card {
            value: 0,
            suit: Suit::Spades,
        }
    }
}

impl From<Card> for u32 {
    /// Convert a `Card` into a `u32` from 0 to 51. Panics if the
    /// card's value is greater than or equal to `SUIT_SIZE`.
    fn from(card: Card) -> u32 {
        if card.value >= SUIT_SIZE as u32 {
            panic!(
                "attempted to convert card with invalid value {} to u32",
                card.value
            );
        }

        u32::from(card.suit) * SUIT_SIZE as u32 + card.value
    }
}

impl TryFrom<u32> for Card {
    type Error = errors::ParseError;

    /// Parses a card from a `u32` which should be in the range `[0..52]`.
    /// Returns an `Err` if `x` is outside this range.
    ///
    /// ```
    /// use cardseed::{Card, Suit};
    ///
    /// assert_eq!(
    ///     Card::try_from(17),
    ///     Ok(Card {
    ///         suit: Suit::Clubs,
    ///         value: 4,
    ///     })
    /// );
    /// ```
    fn try_from(x: u32) -> Result<Card, errors::ParseError> {
        if x >= DECK_SIZE as u32 {
            return Err(errors::ParseError::BadInt(x));
        }

        Ok(Card {
            suit: Suit::try_from(x / SUIT_SIZE as u32)?,
            value: x % SUIT_SIZE as u32,
        })
    }
}

impl fmt::Display for Card {
    /// Formats a `Card` as a 2-character string. The first character is the `Card`'s
    /// face `value`, and the other is its `suit`.
    ///
    /// Returns an error if the `Card`'s value is greater than or equal to `SUIT_SIZE`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.value {
            0 => 'A',
            9 => 'T',
            10 => 'J',
            11 => 'Q',
            12 => 'K',
            v => match char::from_digit(v + 1, 10) {
                Some(c) => c,
                None => return Err(fmt::Error {}),
            },
        };
        write!(f, "{}{}", c, self.suit)
    }
}

impl std::str::FromStr for Card {
    type Err = errors::ParseError;

    /// Parses a `Card` from a string. The first two characters of the string must be the same format
    /// as `Card`'s string formatter outputs. The first character must be the card face value, and the
    /// second must be its suit.
    ///
    /// ```
    /// use cardseed::{Card, Suit};
    ///
    /// let card = "TH".parse::<Card>().unwrap(); // ten of hearts
    /// assert_eq!(card, Card {
    ///     suit: Suit::Hearts,
    ///     value: 9,
    /// });
    /// ```
    fn from_str(s: &str) -> Result<Card, errors::ParseError> {
        let mut chars = s.chars();

        let value = match chars.next() {
            None => return Err(errors::ParseError::BadString(String::from(s))),
            Some(c) => match c.to_digit(10) {
                Some(v) => v - 1,
                None => match c {
                    'A' => 0,
                    'T' => 9,
                    'J' => 10,
                    'Q' => 11,
                    'K' => 12,
                    _ => return Err(errors::ParseError::BadString(String::from(s))),
                },
            },
        };
        let suit = match chars.next() {
            Some(c) => Suit::try_from(c)?,
            None => return Err(errors::ParseError::BadString(String::from(s))),
        };

        Ok(Card {
            suit: suit,
            value: value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_u32() {
        assert_eq!(
            Card::try_from(0),
            Ok(Card {
                value: 0,
                suit: Suit::Spades,
            })
        );

        assert_eq!(
            Card::try_from(3),
            Ok(Card {
                value: 3,
                suit: Suit::Spades,
            })
        );

        assert_eq!(
            Card::try_from(13),
            Ok(Card {
                value: 0,
                suit: Suit::Clubs,
            })
        );

        assert_eq!(
            Card::try_from(29),
            Ok(Card {
                value: 3,
                suit: Suit::Hearts,
            })
        );

        assert_eq!(Card::try_from(56), Err(errors::ParseError::BadInt(56)));
    }

    #[test]
    fn to_u32() {
        assert_eq!(
            u32::from(Card {
                value: 8,
                suit: Suit::Spades,
            }),
            8
        );

        assert_eq!(
            u32::from(Card {
                value: 3,
                suit: Suit::Clubs,
            }),
            16
        );

        assert_eq!(
            u32::from(Card {
                value: 0,
                suit: Suit::Diamonds,
            }),
            39
        );
    }

    #[test]
    fn to_string() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Card::try_from(0)?.to_string(), "AS");
        assert_eq!(Card::try_from(1)?.to_string(), "2S");
        assert_eq!(Card::try_from(9)?.to_string(), "TS");
        assert_eq!(Card::try_from(32)?.to_string(), "7H");
        assert_eq!(Card::try_from(12)?.to_string(), "KS");
        assert_eq!(Card::try_from(50)?.to_string(), "QD");

        Ok(())
    }

    #[test]
    fn from_string() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            "AC".parse::<Card>()?,
            Card {
                value: 0,
                suit: Suit::Clubs
            }
        );

        assert_eq!(
            "KS".parse::<Card>()?,
            Card {
                value: 12,
                suit: Suit::Spades
            }
        );

        assert_eq!(
            "7C".parse::<Card>()?,
            Card {
                value: 6,
                suit: Suit::Clubs
            }
        );

        assert_eq!(
            "AD".parse::<Card>()?,
            Card {
                value: 0,
                suit: Suit::Diamonds
            }
        );

        assert_eq!(
            "TH".parse::<Card>()?,
            Card {
                value: 9,
                suit: Suit::Hearts
            }
        );

        assert_eq!(
            "QH".parse::<Card>()?,
            Card {
                value: 11,
                suit: Suit::Hearts
            }
        );

        Ok(())
    }
}
