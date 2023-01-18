use std::fmt;
use std::fmt::{Display, Formatter};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    pub fn with_color(&mut self, color: &CardColor) -> Option<&Card> {
        match self {
            Card::Wild { color: _ } => {
                *self = Card::Wild { color: Some(*color) };
                Some(self)
            }
            Card::DrawFour { color: _ } => {
                *self = Card::DrawFour { color: Some(*color) };
                Some(self)
            }
            _ => None,
        }
    }

    pub fn can_play(&self, rhs: &Card) -> bool {
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
            (_, Wild { color: _ }) => true,
            (_, DrawFour { color: _ }) => true,
            (_, _) => {
                self.color().expect("special card has no color")
                    == rhs.color().expect("special card has no color")
            }
        }
    }
}
