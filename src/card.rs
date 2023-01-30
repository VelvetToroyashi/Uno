use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use rand::prelude::SliceRandom;
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CardValue {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Copy, Clone, Eq)]
pub enum Card {
    Numeric { color: CardColor, value: CardValue },
    Skip { color: CardColor },
    Reverse { color: CardColor },
    DrawTwo { color: CardColor },
    Wild { color: Option<CardColor> },
    DrawFour { color: Option<CardColor> },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CardColor {
    Red,
    Green,
    Blue,
    Yellow,
}

pub struct Deck {
    pub(crate) cards: Vec<Card>,
}

impl Deck {
    pub fn generate() -> Self {
        Deck { cards: Deck::new_deck() }
    }

    pub fn new_deck() -> Vec<Card> {
        // 2 of each 1-9, 1 of each color zero,
        // 2 of each skip, reverse, draw two, 4 wild, 4 draw four
        let mut deck: Vec<Card> = Vec::with_capacity(108);

        for color in [CardColor::Red, CardColor::Green, CardColor::Blue, CardColor::Yellow] {
            deck.push(Card::Numeric { color, value: CardValue::Zero });
        }

        for color in [CardColor::Red, CardColor::Green, CardColor::Blue, CardColor::Yellow]
        {
            for value in [
                CardValue::One,
                CardValue::Two,
                CardValue::Three,
                CardValue::Four,
                CardValue::Five,
                CardValue::Six,
                CardValue::Seven,
                CardValue::Eight,
                CardValue::Nine,
            ]
            {
                deck.push(Card::Numeric { color, value, });
            }
        }

        for color in [CardColor::Red, CardColor::Green, CardColor::Blue, CardColor::Yellow].iter() {
            deck.push(Card::Skip { color: *color });
            deck.push(Card::Reverse { color: *color });
            deck.push(Card::DrawTwo { color: *color });
        }

        for _ in 0..4 {
            deck.push(Card::Wild { color: None });
            deck.push(Card::DrawFour { color: None });
        }

        deck
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_multiple(&mut self, count: u8) -> Vec<Card> {
        let mut cards = vec![];

        for _ in 0..count {
            if let Some(card) = self.draw() {
                cards.push(card);
            }
        }

        cards
    }

    pub fn reinsert_random(&mut self, card: Card) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.cards.len());

        self.cards.insert(index, card);
    }
}

impl Display for CardColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CardColor::Red => write!(f, "Red"),
            CardColor::Green => write!(f, "Green"),
            CardColor::Blue => write!(f, "Blue"),
            CardColor::Yellow => write!(f, "Yellow"),
        }
    }
}

impl FromStr for CardColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(CardColor::Red),
            "green" => Ok(CardColor::Green),
            "blue" => Ok(CardColor::Blue),
            "yellow" => Ok(CardColor::Yellow),
            _ => Err(format!("{s} is not a valid color")),
        }
    }
}

impl Display for CardValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CardValue::Zero => write!(f, "Zero"),
            CardValue::One => write!(f, "One"),
            CardValue::Two => write!(f, "Two"),
            CardValue::Three => write!(f, "Three"),
            CardValue::Four => write!(f, "Four"),
            CardValue::Five => write!(f, "Five"),
            CardValue::Six => write!(f, "Six"),
            CardValue::Seven => write!(f, "Seven"),
            CardValue::Eight => write!(f, "Eight"),
            CardValue::Nine => write!(f, "Nine"),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        match self {
            Card::Numeric { color, value } => write!(f, "{color} {value}"),
            Card::Skip { color } => write!(f, "{color} Skip"),
            Card::Reverse { color } => write!(f, "{color} Reverse"),
            Card::DrawTwo { color } => write!(f, "{color} Draw Two"),
            Card::Wild { color: Some(color)} => write!(f, "Wild Card ({color})"),
            Card::DrawFour { color: Some(color) } => write!(f, "Draw Four ({color})"),
            Card::Wild { color: None } => write!(f, "Wild Card"),
            Card::DrawFour { color: None } => write!(f, "Draw Four"),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Card::Numeric { color: c1, value: v1 }, Card::Numeric { color: c2, value: v2 }) => {
                c1 == c2 && v1 == v2
            }
            (Card::Skip { color: c1 }, Card::Skip { color: c2 }) => c1 == c2,
            (Card::Reverse { color: c1 }, Card::Reverse { color: c2 }) => c1 == c2,
            (Card::DrawTwo { color: c1 }, Card::DrawTwo { color: c2 }) => c1 == c2,
            (Card::Wild { .. }, Card::Wild { .. }) => true,
            (Card::DrawFour { .. }, Card::DrawFour { .. }) => true,
            _ => false,
        }
    }
}

impl Card {
    pub fn color(&self) -> Option<CardColor> {
        match &self {
            Card::Numeric { color, .. } => Some(*color),
            Card::Skip { color } => Some(*color),
            Card::Reverse { color } => Some(*color),
            Card::DrawTwo { color } => Some(*color),
            Card::Wild { color } => *color,
            Card::DrawFour { color } => *color,
        }
    }

    pub fn with_color(&mut self, color: CardColor) -> Option<&Card> {
        match self {
            Card::Wild { .. } => {
                *self = Card::Wild { color: Some(color) };
                Some(self)
            }
            Card::DrawFour { .. } => {
                *self = Card::DrawFour { color: Some(color) };
                Some(self)
            }
            _ => None,
        }
    }

    pub fn can_play_on(&self, rhs: &Card) -> bool {
        use Card::*;
        match (&self, rhs) {
            (
                Numeric {
                    color: lc,
                    value: lvalue,
                },
                Numeric {
                    color: rc,
                    value: rv,
                },
            ) => lc == rc || lvalue == rv,
            (Skip { color: _ }, Skip { color: _ }) => true,
            (Reverse { color: _ }, Reverse { color: _ }) => true,
            (DrawTwo { color: _ }, DrawTwo { color: _ }) => true,
            (Wild { color: _ }, _) => true,
            (DrawFour { color: _ }, _) => true,
            (_, _) => {
                self.color() == rhs.color()
            }
        }
    }
}
