use std::io::stdin;
use std::marker::PhantomData;
use crate::card::{Card, CardColor};
use rand::Rng;
use crate::game::{GameState, Turn};

pub struct Human {
    name: String,
    hand: Vec<Card>,
}

pub struct Ai {
    name: String,
    hand: Vec<Card>,
}

pub trait Player {
    fn name(&self) -> &str;
    fn hand(&mut self) -> &mut Vec<Card>;
    fn execute_turn(&self, turn: &Turn) -> Option<Card>;
    fn observe_turn(&self, other: &dyn Player, card: &Card);
    fn observe_turn_skip(&self, observed_cards: Option<Vec<&Card>>);
}
pub trait AiPlayer: Player {}
pub trait HumanPlayer: Player {}

const AI_NAMES: [&str; 20] = [
    "Yukii", "Kurisu", "Mayuri", "Makise", "Misa", "Rin", "Miku", "Shinobu", "Shiro", "Rem",
    "Asuna", "Kirito", "Kazuto", "Shana", "Yoshino", "Yui", "Touka", "Rize", "Mikasa", "Levi",
];

impl<'h> Human {
    pub fn new(name: String) -> Human {
        Human {
            name,
            hand: vec![],
        }
    }
}

impl Player for Human {
    fn name(&self) -> &str {
        &self.name
    }

    fn hand(&mut self) -> &mut Vec<Card> {
        &mut self.hand
    }

    fn execute_turn(&self, turn: &Turn) -> Option<&Card> {
        todo!()
    }

    fn observe_turn(&self, other: &dyn Player, card: &Card) {
        todo!()
    }

    fn observe_turn_skip(&self, observed_cards: Option<Vec<&Card>>) {
        todo!()
    }
}

impl HumanPlayer for Human {}

impl Ai {

    pub fn new() -> Ai {
        let mut rng = rand::thread_rng();
        let name = AI_NAMES[rng.gen_range(0..AI_NAMES.len())].to_string();

        Ai {
            name,
            hand: vec![],
        }
    }
}
