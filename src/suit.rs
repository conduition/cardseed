use crate::errors;
use std::fmt;

/// Represents a playing card suit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl From<Suit> for u32 {
    fn from(suit: Suit) -> u32 {
        match suit {
            Suit::Spades => 0,
            Suit::Clubs => 1,
            Suit::Hearts => 2,
            Suit::Diamonds => 3,
        }
    }
}

impl TryFrom<u32> for Suit {
    type Error = errors::ParseError;

    fn try_from(x: u32) -> Result<Suit, errors::ParseError> {
        match x {
            0 => Ok(Suit::Spades),
            1 => Ok(Suit::Clubs),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Diamonds),
            i => Err(errors::ParseError::BadInt(i)),
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = errors::ParseError;

    fn try_from(c: char) -> Result<Suit, errors::ParseError> {
        match c {
            'S' => Ok(Suit::Spades),
            'C' => Ok(Suit::Clubs),
            'H' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            c => Err(errors::ParseError::BadChar(c)),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Suit::Spades => 'S',
            Suit::Clubs => 'C',
            Suit::Hearts => 'H',
            Suit::Diamonds => 'D',
        };
        write!(f, "{}", c)
    }
}

impl Suit {
    /// Returns the set of all suits.
    pub fn all() -> [Suit; 4] {
        [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_u32() {
        assert_eq!(Suit::try_from(0), Ok(Suit::Spades));
        assert_eq!(Suit::try_from(1), Ok(Suit::Clubs));
        assert_eq!(Suit::try_from(2), Ok(Suit::Hearts));
        assert_eq!(Suit::try_from(3), Ok(Suit::Diamonds));
        assert_eq!(Suit::try_from(5), Err(errors::ParseError::BadInt(5)));
    }

    #[test]
    fn to_u32() {
        assert_eq!(u32::from(Suit::Spades), 0);
        assert_eq!(u32::from(Suit::Clubs), 1);
        assert_eq!(u32::from(Suit::Hearts), 2);
        assert_eq!(u32::from(Suit::Diamonds), 3);
    }
}
