# `cardseed`

The `cardseed` crate provides parsing and serializing of playing cards.

In cryptographic contexts, cards can be used to encode and generate entropy. A deck of 52 randomly shuffled playing cards encodes $52! \approx 2^{228}$ different possible combinations. The `cardseed` crate makes it easy to parse and convert cards shuffled in the real world into secure pseudo-random data which can be used to derive encryption or signing keys, seed random number generators, or other such mischief.

```rust
use cardseed::Deck;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let deck = "AS 3H KC 3C".parse::<Deck>()?;
    let secret = deck.hash(Some("bestpasswordever"))?;
    let expected = [35, 20, 205, 7, 35, 104, 123, 150, 57, 148, 101,
                    109, 151, 0, 87, 15, 103, 14, 67, 214, 165, 165,
                    44, 218, 5, 232, 30, 26, 100, 90, 169, 244];
    assert_eq!(secret, expected);
    Ok(())
}
```

## Cards

Cards are composed of two fields: a `value` and a `suit`. The `value` field is a `u32` corresponding to the face value of the card, minus 1 (since we index from zero). The `suit` field is a `cardseed::Suit` enum member, corresponding to one of the four playing card suits.

When serialized, a Card is represented as a string with two characters. The first character represents the face value of the card, and the second represents the suit. For instance, `"7H"` is the seven of hearts.

### Face Values

| Character | Card | Value |
|:---------:|:----:|:-----:|
| `A` | Ace | 0 |
| `2` | 2 | 1 |
| `3` | 3 | 2 |
| `4` | 4 | 3 |
| `5` | 5 | 4 |
| `6` | 6 | 5 |
| `7` | 7 | 6 |
| `8` | 8 | 7 |
| `9` | 9 | 8 |
| `T` | Ten | 9 |
| `J` | Jack | 10 |
| `Q` | Queen | 11 |
| `K` | King | 12 |

### Suits

| Character | Suit | Value |
|:---------:|:----:|:-----:|
| `S` | Spades | 0 |
| `C` | Clubs | 1 |
| `H` | Hearts | 2 |
| `D` | Diamonds | 3 |

Any card in a standard deck can be represented uniquely as any `u32` from 0 to 51, by multiplying the `suit` field's value by 13, and adding the card's `value` field.

```rust
use cardseed::{Card, Suit};

let card = Card {
    suit: Suit::Clubs,
    value: 4,
};
assert_eq!(u32::from(card), 17);
```

For instance, the card `"TD"`, the ten of diamonds, could be represented as the integer $(3 \cdot 13) + 9 = 48$.

## Decks

A `Deck` is a vector of any number of cards. A `Deck` can be serialized and parsed as a string of serialized `Card`s, each `Card` string separated by whitespace.

```rust
use cardseed::{Deck, Card, Suit};
use std::vec::Vec;

let deck = Deck {
    cards: vec![
        Card {
            suit: Suit::Clubs,
            value: 10,
        },
        Card {
            suit: Suit::Hearts,
            value: 12,
        },
        Card {
            suit: Suit::Diamonds,
            value: 1,
        },
    ],
};

let deck_str = format!("{}", deck);
assert_eq!(deck_str, "JC KH 2D");

let parsed_deck = match deck_str.parse::<Deck>() {
    Ok(deck) => deck,
    Err(e) => panic!("failed to parse deck: {}", e),
};
assert_eq!(parsed_deck, deck);
```
