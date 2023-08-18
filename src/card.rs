use crate::suit::Suit;
use crate::{errors, DECK_SIZE, SUIT_SIZE};
use std::{self, fmt};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: u32,
}

impl Card {
    pub fn ace_of_spades() -> Card {
        Card {
            value: 0,
            suit: Suit::Spades,
        }
    }
}

impl From<Card> for u32 {
    fn from(card: Card) -> u32 {
        u32::from(card.suit) * SUIT_SIZE as u32 + card.value
    }
}

impl TryFrom<u32> for Card {
    type Error = errors::ParseError;

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
